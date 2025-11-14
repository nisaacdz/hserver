#[cfg(test)]
mod tests {
    use crate::config::Settings;
    use crate::error::AppError;
    use actix_web::http::StatusCode;

    #[test]
    fn test_app_error_fields() {
        let error = AppError::new(StatusCode::BAD_REQUEST, "Test error");
        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.message, "Test error");
        assert!(error.cause.is_none());
    }

    #[test]
    fn test_app_error_with_cause() {
        let error = AppError::with_cause(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Test error",
            "Root cause",
        );
        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.message, "Test error");
        assert_eq!(error.cause, Some("Root cause".to_string()));
    }

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.server.host, "127.0.0.1");
        assert_eq!(settings.server.port, 8080);
        assert_eq!(settings.database.max_connections, 10);
        assert_eq!(settings.application.name, "Hotel Management System");
    }
}
