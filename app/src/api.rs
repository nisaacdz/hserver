use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

pub enum ApiResponse<S, E> {
    Success(S),
    Error(E),
}

impl<S: Serialize, E: ResponseError> From<Result<S, E>> for ApiResponse<S, E> {
    fn from(result: Result<S, E>) -> Self {
        match result {
            Ok(value) => ApiResponse::Success(value),
            Err(error) => ApiResponse::Error(error),
        }
    }
}

impl<S: Serialize, E: ResponseError> From<ApiResponse<S, E>> for Result<S, E> {
    fn from(value: ApiResponse<S, E>) -> Self {
        match value {
            ApiResponse::Success(value) => Ok(value),
            ApiResponse::Error(error) => Err(error),
        }
    }
}

impl<S: Serialize, E: ResponseError> ApiResponse<S, E> {
    pub fn success(value: S) -> Self {
        ApiResponse::Success(value)
    }

    pub fn error(error: E) -> Self {
        ApiResponse::Error(error)
    }
}

impl<S: Serialize, E: ResponseError> From<ApiResponse<S, E>> for HttpResponse {
    fn from(api_response: ApiResponse<S, E>) -> Self {
        match api_response {
            ApiResponse::Success(value) => HttpResponse::Ok().json(value),
            ApiResponse::Error(error) => error.error_response(),
        }
    }
}
