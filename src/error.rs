//! Error types and handling for the application

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Application-level errors
#[derive(Error, Debug)]
pub enum AppError {
    /// Database operation errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            Self::Config(ref message) => {
                tracing::error!("Configuration error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
        };

        (status, error_message).into_response()
    }
}

/// Convenience type alias for Results with `AppError`
pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    /// Create a configuration error
    #[must_use]
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error() {
        let error = AppError::config("test config error");
        assert!(matches!(error, AppError::Config(_)));
    }

    #[test]
    fn test_database_error() {
        let sql_error = sqlx::Error::RowNotFound;
        let error = AppError::from(sql_error);
        assert!(matches!(error, AppError::Database(_)));
    }
}
