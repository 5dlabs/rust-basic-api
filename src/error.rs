use crate::config::ConfigError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Application-level errors
/// This error type will be used in future tasks for proper error handling
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal server error: {0}")]
    #[allow(dead_code)] // Will be used in future tasks
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            Self::Database(err) => {
                tracing::error!(error = %err, "Database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error occurred".to_string(),
                )
            }
            Self::Config(msg) => {
                tracing::error!(message = %msg, "Configuration error");
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            Self::Internal(msg) => {
                tracing::error!(message = %msg, "Internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
        };

        (status, error_message).into_response()
    }
}

impl From<ConfigError> for AppError {
    fn from(error: ConfigError) -> Self {
        Self::Config(error.to_string())
    }
}
