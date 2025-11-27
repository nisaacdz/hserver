use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct FindRoom {
    pub class_id: String,
    pub period: (i32, i32),
}
