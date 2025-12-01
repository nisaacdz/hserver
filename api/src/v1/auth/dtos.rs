use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub user: AuthUser,
}

#[derive(Serialize, ToSchema)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OnboardRequest {
    pub user_id: Uuid,
    pub otp: String,
    pub password: String,
}
