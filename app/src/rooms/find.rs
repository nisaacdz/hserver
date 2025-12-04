use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct FindRoomOptions {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub class_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FindRoomSuccess {
    pub rooms: Vec<RoomSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RoomSummary {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
}

#[derive(Debug, Serialize)]
pub enum FindRoomError {
    InternalError,
    InvalidDateRange,
}

impl Display for FindRoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindRoomError::InternalError => write!(f, "Internal Server Error"),
            FindRoomError::InvalidDateRange => write!(f, "Invalid date range"),
        }
    }
}

impl ResponseError for FindRoomError {
    fn status_code(&self) -> StatusCode {
        match self {
            FindRoomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            FindRoomError::InvalidDateRange => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
