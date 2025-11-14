use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub message: String,
    pub cause: Option<String>,
}

impl AppError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            cause: None,
        }
    }

    pub fn with_cause(
        status: StatusCode,
        message: impl Into<String>,
        cause: impl Into<String>,
    ) -> Self {
        Self {
            status,
            message: message.into(),
            cause: Some(cause.into()),
        }
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(cause) = &self.cause {
            write!(f, " - Cause: {}", cause)?;
        }
        Ok(())
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        let body = serde_json::json!({
            "error": self.message,
            "cause": self.cause,
        });
        HttpResponse::build(self.status).json(body)
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Self::not_found("Resource not found"),
            _ => Self::with_cause(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
                err.to_string(),
            ),
        }
    }
}

impl From<deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>> for AppError {
    fn from(err: deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>) -> Self {
        Self::with_cause(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database pool error",
            err.to_string(),
        )
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        Self::with_cause(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Configuration error",
            err.to_string(),
        )
    }
}
