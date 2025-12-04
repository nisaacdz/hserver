use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct GetDetailsOptions {
    pub room_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetDetailsSuccess {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
    pub class: RoomClassSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomClassSummary {
    pub id: Uuid,
    pub name: String,
    #[schema(value_type = String)]
    pub base_price: BigDecimal,
}

#[derive(Debug, Serialize)]
pub enum GetDetailsError {
    InternalError,
    NotFound,
}

impl Display for GetDetailsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetDetailsError::InternalError => write!(f, "Internal Server Error"),
            GetDetailsError::NotFound => write!(f, "Room not found"),
        }
    }
}

impl ResponseError for GetDetailsError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetDetailsError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            GetDetailsError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
