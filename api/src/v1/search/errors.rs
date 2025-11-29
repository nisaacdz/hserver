use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use serde::Serialize;

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

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
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
