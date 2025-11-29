use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: AuthUser,
}

#[derive(Serialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}
