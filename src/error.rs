//! Error handling module
//!
//! Defines custom error types for the application.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Application-level errors
///
/// This error type will be used in future tasks for request handlers
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Database connection or query errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[from] crate::config::ConfigError),

    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Database(ref e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            Self::Config(ref e) => {
                tracing::error!("Configuration error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
            Self::Internal(ref msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;

    #[test]
    fn test_app_error_display() {
        let error = AppError::Internal("test error".to_string());
        assert_eq!(format!("{error}"), "Internal server error: test error");
    }

    #[test]
    fn test_app_error_from_config_error() {
        let config_error = crate::config::ConfigError::InvalidPort;
        let app_error: AppError = config_error.into();
        assert!(matches!(app_error, AppError::Config(_)));
    }

    #[tokio::test]
    async fn test_internal_error_response() {
        async fn handler() -> Result<String, AppError> {
            Err(AppError::Internal("test internal error".to_string()))
        }

        let app = Router::new().route("/test", get(handler));

        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Internal server error"));
    }

    #[tokio::test]
    async fn test_config_error_response() {
        async fn handler() -> Result<String, AppError> {
            Err(AppError::Config(crate::config::ConfigError::InvalidPort))
        }

        let app = Router::new().route("/test", get(handler));

        let response = app
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Configuration error"));
    }
}
