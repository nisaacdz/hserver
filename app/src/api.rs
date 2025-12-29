use actix_web::{HttpResponse, ResponseError, body::BoxBody};
use serde::Serialize;

pub enum ApiResponse<S, E> {
    Success(HttpResponse<S>),
    Error(E),
}

impl<S: Serialize, E: ResponseError> From<Result<HttpResponse<S>, E>> for ApiResponse<S, E> {
    fn from(result: Result<HttpResponse<S>, E>) -> Self {
        match result {
            Ok(value) => ApiResponse::Success(value),
            Err(error) => ApiResponse::Error(error),
        }
    }
}

// impl<S: Serialize, E: ResponseError> From<ApiResponse<S, E>> for Result<HttpResponse<S>, E> {
//     fn from(value: ApiResponse<S, E>) -> Self {
//         match value {
//             ApiResponse::Success(value) => Ok(value),
//             ApiResponse::Error(error) => Err(error),
//         }
//     }
// }

impl<S: Serialize, E: ResponseError> ApiResponse<S, E> {
    pub fn success(value: HttpResponse<S>) -> Self {
        ApiResponse::Success(value)
    }

    pub fn error(error: E) -> Self {
        ApiResponse::Error(error)
    }
}

impl<S: Serialize, E: ResponseError> From<ApiResponse<S, E>> for HttpResponse<BoxBody> {
    fn from(api_response: ApiResponse<S, E>) -> Self {
        match api_response {
            ApiResponse::Success(value) => value.map_body(|_, v| serde_json::to_string(&v).expect("Failed to serialize response")).map_into_boxed_body(),
            ApiResponse::Error(error) => error.error_response(),
        }
    }
}

impl<S: Serialize, E: ResponseError> From<ApiResponse<S, E>> for Result<HttpResponse<BoxBody>, E> {
    fn from(api_response: ApiResponse<S, E>) -> Self {
        match api_response {
            ApiResponse::Success(value) => Ok(value.map_body(|_, v| serde_json::to_string(&v).expect("Failed to serialize response")).map_into_boxed_body()),
            ApiResponse::Error(error) => Err(error),
        }
    }
}
