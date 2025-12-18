use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

use utoipa::ToSchema;

pub mod login;
pub mod onboard;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SessionUser {
    pub id: Uuid,
    pub staff_id: Option<Uuid>,
    pub email: String,
}

#[derive(Debug)]
pub enum AuthError {
    InternalError,
    InvalidCredentials,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InternalError => write!(f, "Internal Server Error"),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_string())
    }
}

pub fn hash_password(plain_password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(plain_password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?;

    Ok(password_hash.to_string())
}

pub fn verify_password(plain_password: &str, stored_hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|e| e.to_string())?;

    let argon2 = Argon2::default();

    let is_valid = argon2
        .verify_password(plain_password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}
