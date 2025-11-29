use actix_web::{
    Error, HttpMessage,
    cookie::{Cookie, SameSite},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::{ErrorUnauthorized, InternalError},
    http::StatusCode,
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use bincode::{Decode, Encode, config};
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use chrono::Utc;
use domain::SecurityConfig;
use futures_util::{
    future::{Ready, ok},
    task::{Context, Poll},
};
use rand::RngCore;
use std::{future::Future, pin::Pin, rc::Rc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct SessionUser {
    pub id: Uuid,
    pub staff_id: Option<Uuid>,
    pub email: String,
}

// Bincode implementation for efficient binary serialization
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

/// Internal wrapper to include expiration in the encrypted payload
#[derive(Encode, Decode, Debug)]
pub struct AuthSession {
    pub exp: i64,
    pub user: SessionUser,
}

#[derive(Clone)]
pub struct TokenEngine {
    cipher: XChaCha20Poly1305,
    duration: i64,
}

impl TokenEngine {
    pub fn new(config: &SecurityConfig) -> Self {
        let key_bytes = config.key.as_bytes();
        if key_bytes.len() != 32 {
            panic!("Security key must be exactly 32 bytes long for XChaCha20Poly1305");
        }

        let key = chacha20poly1305::Key::from_slice(key_bytes);

        Self {
            cipher: XChaCha20Poly1305::new(key),
            duration: config.session_duration as i64,
        }
    }

    /// Serializes and Encrypts user data into a Base64 string.
    /// Optimized for minimum allocations.
    pub fn create_token(&self, user: SessionUser) -> Result<String, Error> {
        let session = AuthSession {
            exp: Utc::now().timestamp() + self.duration,
            user,
        };

        let config = config::standard();
        let payload_bytes = bincode::encode_to_vec(session, config)
            .map_err(|_| ErrorUnauthorized("Serialization error"))?;

        let mut nonce = XNonce::default();
        rand::rng().fill_bytes(&mut nonce);

        let ciphertext = self
            .cipher
            .encrypt(&nonce, payload_bytes.as_ref())
            .map_err(|_| ErrorUnauthorized("Encryption failed"))?;

        let mut final_buffer = Vec::with_capacity(nonce.len() + ciphertext.len());
        final_buffer.extend_from_slice(&nonce);
        final_buffer.extend_from_slice(&ciphertext);
        Ok(BASE64.encode(final_buffer))
    }

    /// Decrypts a Base64 string back into SessionUser
    pub fn verify_token(&self, token_str: &str) -> Result<SessionUser, Error> {
        let encrypted_data = BASE64
            .decode(token_str)
            .map_err(|_| ErrorUnauthorized("Invalid token encoding"))?;

        if encrypted_data.len() < 24 {
            return Err(ErrorUnauthorized("Token too short"));
        }
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(24);
        let nonce = XNonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| ErrorUnauthorized("Invalid token signature or data"))?;
        let config = config::standard();
        let (session, _): (AuthSession, usize) = bincode::decode_from_slice(&plaintext, config)
            .map_err(|_| ErrorUnauthorized("Invalid session data"))?;
        if session.exp < Utc::now().timestamp() {
            return Err(ErrorUnauthorized("Token expired"));
        }

        Ok(session.user)
    }
}

/// Helper to generate the HTTP Cookie
pub fn generate_auth_cookie(
    token_engine: &TokenEngine,
    user: SessionUser,
) -> Result<Cookie<'static>, Error> {
    let token_str = token_engine.create_token(user)?;

    Ok(Cookie::build("auth-token", token_str)
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .finish())
}

pub fn hash_password(plain_password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(plain_password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;

    Ok(password_hash.to_string())
}

/// Used during Login: Verifies a plain password against a stored Argon2 hash.
pub fn verify_password(plain_password: &str, stored_hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|e| e.to_string())?;

    let argon2 = Argon2::default();

    let is_valid = argon2
        .verify_password(plain_password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}

pub struct AuthMiddleware;

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
            let token_engine =
                req.app_data::<actix_web::web::Data<TokenEngine>>()
                    .ok_or(InternalError::new(
                        "Internal Error: TokenEngine not configured",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))?;

            if let Some(cookie) = req.cookie("auth-token") {
                let token_str = cookie.value();

                match token_engine.verify_token(token_str) {
                    Ok(user) => {
                        req.extensions_mut().insert(Rc::new(user));
                        return srv.call(req).await;
                    }
                    Err(e) => return Err(e),
                }
            }

            Err(ErrorUnauthorized("Missing authentication token"))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::{self, TestRequest};
    use actix_web::web::Data;

    fn default_user() -> SessionUser {
        SessionUser {
            id: Uuid::new_v4(),
            staff_id: None,
            email: "perf_test@test.com".to_string(),
        }
    }

    fn get_test_service() -> TokenEngine {
        let config = SecurityConfig {
            key: "01234567890123456789012345678901".to_string(),
            session_duration: 3600,
        };
        TokenEngine::new(&config)
    }

    #[test]
    fn test_token_round_trip() {
        let service = get_test_service();
        let user = default_user();

        let token = service
            .create_token(user.clone())
            .expect("Encryption failed");

        let decoded_user = service.verify_token(&token).expect("Decryption failed");

        assert_eq!(user, decoded_user);
    }

    #[test]
    fn test_tampered_token_fails() {
        let service = get_test_service();
        let token = service.create_token(default_user()).unwrap();

        let mut raw = BASE64.decode(&token).unwrap();
        let last_idx = raw.len() - 1;
        raw[last_idx] ^= 0xFF;

        let bad_token = BASE64.encode(raw);

        let result = service.verify_token(&bad_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_expired_token() {
        let config = SecurityConfig {
            key: "01234567890123456789012345678901".to_string(),
            session_duration: 0,
        };

        let service = TokenEngine::new(&config);

        let token = service.create_token(default_user()).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        let result = service.verify_token(&token);

        assert!(result.is_err());
    }

    #[actix_web::test]
    async fn test_middleware_integration() {
        let service = get_test_service();
        let user = default_user();
        let cookie = generate_auth_cookie(&service, user).unwrap();

        let req = TestRequest::default()
            .app_data(Data::new(service))
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
}
