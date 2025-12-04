use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OnboardRequest {
    pub user_id: Uuid,
    pub otp: String,
    pub password: String,
}
