use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use serde::Serialize;

#[derive(Debug)]
pub enum RoomError {
    NotFound,
    InternalError,
    Unauthorized,
}

impl std::fmt::Display for RoomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomError::NotFound => write!(f, "Room not found"),
            RoomError::InternalError => write!(f, "Internal Server Error"),
            RoomError::Unauthorized => write!(f, "Unauthorized: Staff access required"),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl ResponseError for RoomError {
    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            RoomError::NotFound => StatusCode::NOT_FOUND,
            RoomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            RoomError::Unauthorized => StatusCode::FORBIDDEN,
        }
    }
}

#[derive(Debug)]
pub enum SearchError {
    InternalError,
    InvalidDateRange,
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::InternalError => write!(f, "Internal Server Error"),
            SearchError::InvalidDateRange => write!(f, "Invalid date range"),
        }
    }
}

impl ResponseError for SearchError {
    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SearchError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            SearchError::InvalidDateRange => StatusCode::BAD_REQUEST,
        }
    }
}
