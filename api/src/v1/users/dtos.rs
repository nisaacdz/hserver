use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub user: UserDetails,
}

#[derive(Serialize, ToSchema)]
pub struct UserDetails {
    pub id: Uuid,
    pub email: String,
}
