use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Application-level errors with proper HTTP status code mapping
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Internal server error
    #[error("Internal server error: {0}")]
    Internal(String),

    /// Not found error
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Bad request error
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Unauthorized error
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {err}"),
            ),
            Self::Config(msg) | Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

/// Convert environment variable errors to configuration errors
impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        Self::Config(format!("Environment variable error: {err}"))
    }
}

/// Result type alias for application errors
pub type AppResult<T> = Result<T, AppError>;
