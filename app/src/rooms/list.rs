use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListedRoom {
    pub id: Uuid,
    pub label: String,
    pub class_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ListRoomOptions {
    pub search: Option<String>,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRoomSuccess {
    pub rooms: Vec<ListedRoom>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub enum ListRoomError {
    Unauthorized,
    InternalError,
    DatabaseError(String),
}

impl Display for ListRoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListRoomError::Unauthorized => write!(f, "Unauthorized"),
            ListRoomError::InternalError => write!(f, "Internal Server Error"),
            ListRoomError::DatabaseError(e) => write!(f, "Database Error: {}", e),
        }
    }
}

impl ResponseError for ListRoomError {
    fn status_code(&self) -> StatusCode {
        match self {
            ListRoomError::Unauthorized => StatusCode::UNAUTHORIZED,
            ListRoomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ListRoomError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
