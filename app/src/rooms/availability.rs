use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Bound;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct GetAvailabilityOptions {
    pub room_id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetAvailabilitySuccess {
    pub room_id: Uuid,
    #[schema(value_type = Vec<String>, example = json!(["2023-01-01T00:00:00Z", "2023-01-02T00:00:00Z"]))]
    pub period: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    pub blocks: Vec<CalendarBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarBlock {
    pub id: Uuid,
    #[schema(value_type = Vec<String>)]
    pub period: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    #[serde(rename = "type")]
    pub kind: BlockKind,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockKind {
    Booking,
    Maintenance,
    Unknown,
}

#[derive(Debug, Serialize)]
pub enum GetAvailabilityError {
    Unauthorized,
    InternalError,
    NotFound,
}

impl Display for GetAvailabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetAvailabilityError::Unauthorized => write!(f, "Unauthorized"),
            GetAvailabilityError::InternalError => write!(f, "Internal Server Error"),
            GetAvailabilityError::NotFound => write!(f, "Room not found"),
        }
    }
}

impl ResponseError for GetAvailabilityError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetAvailabilityError::Unauthorized => StatusCode::UNAUTHORIZED,
            GetAvailabilityError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            GetAvailabilityError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
