//! Error types and handling for the application.
//!
//! This module defines custom error types using `thiserror` and provides
//! conversions to HTTP responses for Axum.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Application-level errors.
///
/// These errors represent various failure modes that can occur during
/// request processing and are automatically converted to appropriate
/// HTTP responses.
///
/// Note: Some error variants are defined for future use and may not be used yet.
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    /// Database operation failed
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Generic internal server error
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Bad request / validation error
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Database(ref err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database operation failed".to_string(),
                )
            }
            Self::Config(ref msg) => {
                tracing::error!("Configuration error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            Self::Internal(ref err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            Self::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            Self::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

/// Convenience type alias for Results that can return `AppError`.
///
/// Note: Defined for future use in API endpoints.
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::NotFound("User".to_string());
        assert_eq!(err.to_string(), "Resource not found: User");

        let err = AppError::BadRequest("Invalid input".to_string());
        assert_eq!(err.to_string(), "Bad request: Invalid input");
    }
}
