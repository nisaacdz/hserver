use actix_web::{http::StatusCode, HttpResponseBuilder};
use bcrypt::BcryptError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use std::convert::From;
use uuid::Error as UuidError;

use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub status: StatusCode,
}

#[derive(Debug, Serialize)]
struct AppErrorBody {
    pub error: String,
}

impl AppError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        AppError {
            status,
            message: Some(message.into()),
            cause: None,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn cause<E: std::error::Error + 'static>(mut self, cause: E) -> Self {
        self.cause = Some(Box::new(cause));
        self
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

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.cause, &self.message) {
            (Some(cause), Some(message)) => write!(f, "{}: {}", message, cause),
            (Some(cause), None) => write!(f, "{}", cause),
            (None, Some(message)) => write!(f, "{}", message),
            (None, None) => write!(f, "{}", self.status.canonical_reason().unwrap()),
        }
    }
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.status
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(AppErrorBody {
            error: format!("{}", self),
        })
    }
}

// REMOVED: This is the first, simpler, and conflicting implementation
// impl From<diesel::result::Error> for AppError {
//     fn from(err: diesel::result::Error) -> Self {
//         match err {
//             diesel::result::Error::NotFound => Self::not_found("Resource not found"),
//             _ => Self::new(StatusCode::INTERNAL_SERVER_ERROR, "Database error").cause(err),
//         }
//     }
// }

impl From<deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>> for AppError {
    fn from(err: deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "Database pool error").cause(err)
    }
}

impl From<BcryptError> for AppError {
    fn from(_err: BcryptError) -> Self {
        AppError::internal_error("An error occurred")
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            JwtErrorKind::InvalidToken => AppError::unauthorized("Token is invalid"),
            JwtErrorKind::InvalidIssuer => AppError::unauthorized("Issuer is invalid"),
            _ => AppError::unauthorized("An issue was found with the token provided"),
        }
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    AppError {
                        status: StatusCode::BAD_REQUEST,
                        message: Some(message.into()),
                        cause: None,
                    }
                } else {
                    let full_error = DieselError::DatabaseError(kind, info);
                    AppError::internal_error("Internal server error").cause(full_error)
                }
            }
            DieselError::NotFound => AppError::not_found("record not found"),
            _ => AppError::internal_error("Internal server error").cause(err),
        }
    }
}

impl From<UuidError> for AppError {
    fn from(_err: UuidError) -> Self {
        AppError::not_found("Uuid is invalid.")
    }
}