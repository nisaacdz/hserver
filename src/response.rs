use serde::Serialize;
use utoipa::ToSchema;

/// Standard API response structure
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    /// Create a successful response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    /// Create a successful response with data and a custom message
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.into()),
        }
    }
}

impl ApiResponse<()> {
    /// Create a successful response without data
    pub fn success_no_data() -> Self {
        Self {
            success: true,
            data: None,
            message: None,
        }
    }

    /// Create a successful response without data but with a message
    pub fn success_message(message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: None,
            message: Some(message.into()),
        }
    }

    /// Create an error response with a message
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message.into()),
        }
    }
}
