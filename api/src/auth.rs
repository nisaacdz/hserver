use actix_web::{
    Error, HttpMessage,
    cookie::{CookieJar, Key},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use bincode::{Decode, Encode, config};
use chrono::Utc;
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
    pub staff_id: Uuid,
    pub email: String,
}

bincode::impl_borrow_decode! {SessionUser}

impl Encode for SessionUser {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.id.as_bytes(), encoder)?;
        bincode::Encode::encode(&self.staff_id.as_bytes(), encoder)?;
        bincode::Encode::encode(&self.email, encoder)?;
        Ok(())
    }
}

impl<Ctx> Decode<Ctx> for SessionUser {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let id = Uuid::from_bytes(bincode::Decode::decode(decoder)?);
        let staff_id = Uuid::from_bytes(bincode::Decode::decode(decoder)?);
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
    BASE64.encode(encoded_bytes)
}

// ==========================================
// 3. The Middleware
// ==========================================

pub struct AuthMiddleware {
    key: Key,
}

impl AuthMiddleware {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

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
            key: self.key.clone(),
        })
    }
}

// Middleware Logic
pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    key: Key,
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
        let key = self.key.clone();

        Box::pin(async move {
            // 1. Extract Cookie
            if let Some(cookie) = req.cookie("auth-token") {
                // 2. Load into Jar for Decryption
                let mut jar = CookieJar::new();
                jar.add_original(cookie.clone());

                // 3. Verify Signature & Decrypt
                if let Some(private_cookie) = jar.private(&key).get("auth-token") {
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
                                    req.extensions_mut().insert(session.user);
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
    use actix_web::cookie::*;

    #[test]
    fn test_token_creation_and_decoding() {
        // Setup Data
        let user = SessionUser {
            id: Uuid::new_v4(),
            staff_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
        };
        let origin = "http://localhost".to_string();

        // 1. Create Token (using our helper logic)
        let token_str = create_session_token(user.clone(), origin.clone());

        // 2. Decode manually to verify
        let decoded_bytes = BASE64.decode(token_str).unwrap();
        let (decoded_session, _): (AuthSession, usize) =
            bincode::decode_from_slice(&decoded_bytes, config::standard()).unwrap();

        assert_eq!(decoded_session.user, user);
        assert_eq!(decoded_session.origin, origin);
    }

    #[actix_web::test]
    async fn test_auth_middleware_flow() {
        let key = Key::generate();

        // 1. Setup User & Token
        let user = SessionUser {
            id: Uuid::new_v4(),
            staff_id: Uuid::new_v4(),
            email: "middleware@test.com".to_string(),
        };

        // Generate the inner payload
        let payload_str = create_session_token(user.clone(), "http://localhost".to_string());

        // 2. Encrypt it into a Cookie (Simulating the Server's Login Response)
        let mut jar = CookieJar::new();
        jar.private_mut(&key)
            .add(Cookie::new("auth-token", payload_str));
        let encrypted_value = jar.get("auth-token").unwrap().value().to_string();

        // 3. Create Request
        let req = actix_web::test::TestRequest::default()
            .cookie(Cookie::build("auth-token", encrypted_value).finish())
            .insert_header(("Origin", "http://localhost")) // Header matches token
            .to_srv_request();

        // 4. Run Middleware
        let middleware = AuthMiddleware::new(key);
        let srv = middleware
            .new_transform(actix_web::test::ok_service())
            .await
            .unwrap();

        let resp = srv.call(req).await;

        // Should succeed
        assert!(resp.is_ok());
    }

    #[actix_web::test]
    async fn test_fail_invalid_origin() {
        let key = Key::generate();
        let user = SessionUser {
            id: Uuid::new_v4(),
            staff_id: Uuid::new_v4(),
            email: "hack@test.com".to_string(),
        };

        // Token bound to localhost
        let payload_str = create_session_token(user, "http://localhost".to_string());

        let mut jar = CookieJar::new();
        jar.private_mut(&key)
            .add(Cookie::new("auth-token", payload_str));
        let encrypted_value = jar.get("auth-token").unwrap().value().to_string();

        // Request coming from EVIL.COM
        let req = actix_web::test::TestRequest::default()
            .cookie(Cookie::build("auth-token", encrypted_value).finish())
            .insert_header(("Origin", "http://evil.com"))
            .to_srv_request();

        let middleware = AuthMiddleware::new(key);
        let srv = middleware
            .new_transform(actix_web::test::ok_service())
            .await
            .unwrap();

        let resp = srv.call(req).await;

        // Should Fail
        assert!(resp.is_err());
    }
}
