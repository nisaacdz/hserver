#[cfg(test)]
mod tests {
    use crate::config::Settings;
    use crate::error::AppError;
    use actix_web::http::StatusCode;

    #[test]
    fn test_app_error_fields() {
        let error = AppError::new(StatusCode::BAD_REQUEST, "Test error");
        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.message, Some("Test error".to_string()));
        assert!(error.cause.is_none());
    }

    #[test]
    fn test_app_error_with_cause() {
        let error = AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Test error")
            .cause(std::io::Error::new(std::io::ErrorKind::Other, "Root cause"));
        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.message, Some("Test error".to_string()));
        assert!(error.cause.is_some());
        assert_eq!(error.cause.as_ref().unwrap().to_string(), "Root cause");
    }

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.server.host, "127.0.0.1");
        assert_eq!(settings.server.port, 8080);
        assert_eq!(settings.database.max_connections, 10);
        assert_eq!(settings.application.name, "Hotel Management System");
    }

    #[test]
    fn test_api_response_success() {
        use crate::response::ApiResponse;
        
        let response = ApiResponse::success("test data");
        assert!(response.success);
        assert_eq!(response.data, Some("test data"));
        assert!(response.message.is_none());
    }

    #[test]
    fn test_api_response_success_with_message() {
        use crate::response::ApiResponse;
        
        let response = ApiResponse::success_with_message("test data", "Operation successful");
        assert!(response.success);
        assert_eq!(response.data, Some("test data"));
        assert_eq!(response.message, Some("Operation successful".to_string()));
    }

    #[test]
    fn test_api_response_error() {
        use crate::response::ApiResponse;
        
        let response: ApiResponse<()> = ApiResponse::error("Something went wrong");
        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.message, Some("Something went wrong".to_string()));
    }

    #[test]
    fn test_api_response_success_no_data() {
        use crate::response::ApiResponse;
        
        let response = ApiResponse::success_no_data();
        assert!(response.success);
        assert!(response.data.is_none());
        assert!(response.message.is_none());
    }

    #[test]
    fn test_api_response_success_message() {
        use crate::response::ApiResponse;
        
        let response = ApiResponse::success_message("Action completed");
        assert!(response.success);
        assert!(response.data.is_none());
        assert_eq!(response.message, Some("Action completed".to_string()));
    }
}
