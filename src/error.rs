//! Error types and handling
//!
//! This module defines custom error types for the application using thiserror.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Application-level errors
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    /// Database operation errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Not found errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Self::Database(_) | Self::Config(_) | Self::Internal(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        (status, message).into_response()
    }
}
