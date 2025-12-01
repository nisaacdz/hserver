use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserResponse {
    pub user: UserDetails,
}

#[derive(Serialize)]
pub struct UserDetails {
    pub id: Uuid,
    pub email: String,
}
