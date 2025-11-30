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

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OnboardRequest {
    pub user_id: Uuid,
    pub otp: String,
    pub password: String,
}
