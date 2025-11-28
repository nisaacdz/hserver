use actix_web::{
    Error, HttpMessage,
    cookie::{CookieJar, Key},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use bincode::{Decode, Encode, config};
use chrono::Utc;
use domain::SecurityConfig;
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
    pub exp: i64,
    pub user: SessionUser,
}

// ==========================================
// 2. Token Logic (Helper)
// ==========================================

/// Helper to generate the cookie value during Login
/// Helper to generate the cookie value during Login
pub fn generate_auth_cookie(
    user: SessionUser,
    config: &SecurityConfig,
) -> actix_web::cookie::Cookie<'static> {
    let session = AuthSession {
        exp: Utc::now().timestamp() + config.session_duration as i64,
        user,
    };

    let config = config::standard();
    let encoded_bytes = bincode::encode_to_vec(session, config).unwrap();
    let token = BASE64.encode(&encoded_bytes);

    actix_web::cookie::Cookie::build("auth-token", token)
        .secure(true)
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .path("/")
        .finish()
}

// ==========================================
// 3. The Middleware
// ==========================================

pub struct AuthConfig {
    key: Key,
}

impl AuthConfig {
    pub fn new(config: &SecurityConfig) -> Self {
        Self {
            key: Key::from(config.key.as_bytes()),
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
            let auth_config = req
                .app_data::<AuthConfig>()
                .expect("AuthConfig not in app data");

            if let Some(cookie) = req.cookie("auth-token") {
                let mut jar = CookieJar::new();
                jar.add_original(cookie);

                if let Some(private_cookie) = jar.private(&auth_config.key).get("auth-token") {
                    let value_str = private_cookie.value();

                    if let Ok(decoded_bytes) = BASE64.decode(value_str) {
                        let config = config::standard();
                        if let Ok((session, _size)) =
                            bincode::decode_from_slice::<AuthSession, _>(&decoded_bytes, config)
                        {
                            if session.exp < Utc::now().timestamp() {
                                return Err(ErrorUnauthorized("Token expired"));
                            }

                            req.extensions_mut().insert(Rc::new(session.user));
                            return srv.call(req).await;
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

    fn create_test_cookie(key: &Key, user: SessionUser, exp_offset: i64) -> Cookie<'static> {
        let session = AuthSession {
            exp: Utc::now().timestamp() + exp_offset,
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
        let cookie = create_test_cookie(&key, default_user(), 3600);

        let req = TestRequest::default()
            .app_data(AuthConfig::with_key(key))
            .cookie(cookie)
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
        let cookie = create_test_cookie(&key, default_user(), -10);

        let req = TestRequest::default()
            .app_data(AuthConfig::with_key(key))
            .cookie(cookie)
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
    async fn test_auth_fails_tampered_cookie() {
        let key = Key::generate();
        let mut cookie = create_test_cookie(&key, default_user(), 3600);

        // Maliciously modify the encrypted string
        let mut bad_value = cookie.value().to_string();
        bad_value.push('a'); // Corrupt the signature/data
        cookie.set_value(bad_value);

        let req = TestRequest::default()
            .app_data(AuthConfig::with_key(key))
            .cookie(cookie)
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
            .app_data(AuthConfig::with_key(Key::generate()))
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
