use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct GetUserDetailsOptions {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDetailsSuccess {
    pub user: UserDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub enum GetUserDetailsError {
    InternalError,
    NotFound,
}

impl Display for GetUserDetailsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetUserDetailsError::InternalError => write!(f, "Internal Server Error"),
            GetUserDetailsError::NotFound => write!(f, "User not found"),
        }
    }
}

impl ResponseError for GetUserDetailsError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetUserDetailsError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            GetUserDetailsError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
