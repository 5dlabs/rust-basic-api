use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::env;
use thiserror::Error;

/// Application-level errors
#[derive(Debug, Error)]
pub enum AppError {
    /// Database connection error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Internal server error
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),

    /// Missing required environment variable
    #[error("Missing required environment variable {key}")]
    MissingEnvVar {
        key: &'static str,
        #[source]
        source: env::VarError,
    },

    /// Invalid server port value
    #[error("Invalid SERVER_PORT value")]
    InvalidServerPort {
        #[source]
        source: std::num::ParseIntError,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::Internal(ref err) => {
                tracing::error!("Internal error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::MissingEnvVar { key, ref source } => {
                tracing::error!("Missing environment variable {}: {}", key, source);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
            AppError::InvalidServerPort { ref source } => {
                tracing::error!("Invalid server port: {}", source);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
        };

        (status, error_message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AppError::MissingEnvVar {
            key: "TEST_VAR",
            source: std::env::VarError::NotPresent,
        };
        assert_eq!(
            err.to_string(),
            "Missing required environment variable TEST_VAR"
        );
    }
}
