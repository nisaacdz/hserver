use actix_web::{
    Error, HttpMessage,
    cookie::{CookieJar, Key},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use bincode::{Decode, Encode, config};
use chrono::Utc;
use domain::JwtConfig;
use futures_util::future::{Ready, ok};
use futures_util::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use uuid::Uuid;

// ==========================================
// 1. Data Models
// ==========================================

#[derive(Clone, Debug, PartialEq)]
pub struct SessionUser {
    pub id: Uuid,
    pub staff_id: Option<Uuid>,
    pub email: String,
}

bincode::impl_borrow_decode! {SessionUser}

impl Encode for SessionUser {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.id.as_bytes(), encoder)?;
        bincode::Encode::encode(&self.staff_id.as_ref().map(|id| id.as_bytes()), encoder)?;
        bincode::Encode::encode(&self.email, encoder)?;
        Ok(())
    }
}

impl<Ctx> Decode<Ctx> for SessionUser {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let id = Uuid::from_bytes(bincode::Decode::decode(decoder)?);
        let staff_id = Option::<[u8; 16]>::decode(decoder)?.map(Uuid::from_bytes);
        let email = bincode::Decode::decode(decoder)?;

        Ok(SessionUser {
            id,
            staff_id,
            email,
        })
    }
}

#[derive(Encode, Decode, Debug)]
pub struct AuthSession {
    pub exp: i64,       // Expiration Timestamp
    pub origin: String, // The domain this token is bound to
    pub user: SessionUser,
}

// ==========================================
// 2. Token Logic (Helper)
// ==========================================

/// Helper to generate the cookie value during Login
pub fn create_session_token(user: SessionUser, origin: String) -> String {
    let session = AuthSession {
        exp: Utc::now().timestamp() + 3600, // 1 hour expiry
        origin,
        user,
    };

    let config = config::standard();
    let encoded_bytes = bincode::encode_to_vec(session, config).unwrap();
    unsafe { String::from_utf8_unchecked(encoded_bytes) }
}

// ==========================================
// 3. The Middleware
// ==========================================

pub struct JwtContext {
    key: Key,
}

impl JwtContext {
    pub fn new(config: &JwtConfig) -> Self {
        Self {
            key: Key::from(config.secret.as_bytes()),
        }
    }

    pub fn with_key(key: Key) -> Self {
        Self { key }
    }
}

pub struct AuthMiddleware;

// Middleware Factory
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
        })
    }
}

// Middleware Logic
pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            let jwt_context = req
                .app_data::<JwtContext>()
                .expect("JwtContext not in app data");

            // 1. Extract Cookie
            if let Some(cookie) = req.cookie("auth-token") {
                // 2. Load into Jar for Decryption
                let mut jar = CookieJar::new();
                jar.add_original(cookie);

                // 3. Verify Signature & Decrypt
                if let Some(private_cookie) = jar.private(&jwt_context.key).get("auth-token") {
                    let value_str = private_cookie.value();

                    // 4. Base64 Decode
                    if let Ok(decoded_bytes) = BASE64.decode(value_str) {
                        // 5. Bincode Decode
                        let config = config::standard();
                        if let Ok((session, _size)) =
                            bincode::decode_from_slice::<AuthSession, _>(&decoded_bytes, config)
                        {
                            // 6. Validations

                            // A. Check Expiry
                            if session.exp < Utc::now().timestamp() {
                                return Err(ErrorUnauthorized("Token expired"));
                            }

                            // B. Check Origin (Strict)
                            // If Origin header is missing OR mismatch -> Reject
                            let origin_header =
                                req.headers().get("Origin").and_then(|h| h.to_str().ok());

                            match origin_header {
                                Some(o) if o == session.origin => {
                                    // 7. Success: Attach USER (not the whole session)
                                    req.extensions_mut().insert(Rc::new(session.user));
                                    return srv.call(req).await;
                                }
                                _ => return Err(ErrorUnauthorized("Invalid Origin")),
                            }
                        }
                    }
                }
            }

            Err(ErrorUnauthorized("Invalid or missing authentication token"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::cookie::Cookie;
    use actix_web::test::{self, TestRequest};

    fn create_test_cookie(
        key: &Key,
        user: SessionUser,
        origin: &str,
        exp_offset: i64,
    ) -> Cookie<'static> {
        let session = AuthSession {
            exp: Utc::now().timestamp() + exp_offset,
            origin: origin.to_string(),
            user,
        };
        let bytes = bincode::encode_to_vec(session, config::standard()).unwrap();
        let b64_str = BASE64.encode(bytes);
        let mut jar = CookieJar::new();
        jar.private_mut(key).add(Cookie::new("auth-token", b64_str));
        jar.get("auth-token").unwrap().clone()
    }

    fn default_user() -> SessionUser {
        SessionUser {
            id: Uuid::new_v4(),
            staff_id: None,
            email: "test@test.com".to_string(),
        }
    }

    #[actix_web::test]
    async fn test_auth_success() {
        let key = Key::generate();
        let cookie = create_test_cookie(&key, default_user(), "http://localhost", 3600);

        let req = TestRequest::default()
            .app_data(JwtContext::with_key(key))
            .cookie(cookie)
            .insert_header(("Origin", "http://localhost"))
            .to_srv_request();

        let resp = AuthMiddleware
            .new_transform(test::ok_service())
            .await
            .unwrap()
            .call(req)
            .await;

        assert!(resp.is_ok());
    }

    #[actix_web::test]
    async fn test_auth_fails_expired() {
        let key = Key::generate();
        // Expiry set to -10 seconds (Past)
        let cookie = create_test_cookie(&key, default_user(), "http://localhost", -10);

        let req = TestRequest::default()
            .app_data(JwtContext::with_key(key))
            .cookie(cookie)
            .insert_header(("Origin", "http://localhost"))
            .to_srv_request();

        let resp = AuthMiddleware
            .new_transform(test::ok_service())
            .await
            .unwrap()
            .call(req)
            .await;

        // Should return 401 Unauthorized
        assert!(resp.is_err());
    }

    #[actix_web::test]
    async fn test_auth_fails_wrong_origin() {
        let key = Key::generate();
        let cookie = create_test_cookie(&key, default_user(), "http://localhost", 3600);

        let req = TestRequest::default()
            .app_data(JwtContext::with_key(key))
            .cookie(cookie)
            // Header says evil.com, token says localhost
            .insert_header(("Origin", "http://evil.com"))
            .to_srv_request();

        let resp = AuthMiddleware
            .new_transform(test::ok_service())
            .await
            .unwrap()
            .call(req)
            .await;

        assert!(resp.is_err());
    }

    #[actix_web::test]
    async fn test_auth_fails_tampered_cookie() {
        let key = Key::generate();
        let mut cookie = create_test_cookie(&key, default_user(), "http://localhost", 3600);

        // Maliciously modify the encrypted string
        let mut bad_value = cookie.value().to_string();
        bad_value.push('a'); // Corrupt the signature/data
        cookie.set_value(bad_value);

        let req = TestRequest::default()
            .app_data(JwtContext::with_key(key))
            .cookie(cookie)
            .insert_header(("Origin", "http://localhost"))
            .to_srv_request();

        let resp = AuthMiddleware
            .new_transform(test::ok_service())
            .await
            .unwrap()
            .call(req)
            .await;

        // PrivateJar signature check should fail -> None -> Unauthorized
        assert!(resp.is_err());
    }

    #[actix_web::test]
    async fn test_auth_fails_missing_cookie() {
        let req = TestRequest::default()
            .app_data(JwtContext::with_key(Key::generate()))
            .insert_header(("Origin", "http://localhost"))
            .to_srv_request();

        let resp = AuthMiddleware
            .new_transform(test::ok_service())
            .await
            .unwrap()
            .call(req)
            .await;

        assert!(resp.is_err());
    }
}
