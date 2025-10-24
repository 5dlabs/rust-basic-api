use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Application-level errors
#[derive(Debug, Error)]
pub enum AppError {
    /// Database connection error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Internal server error
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),

    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::Config(ref err) => {
                tracing::error!("Configuration error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
            AppError::Internal(ref err) => {
                tracing::error!("Internal error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::EnvVar(ref err) => {
                tracing::error!("Environment variable error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
        };

        (status, error_message).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn test_error_display() {
        let err = AppError::Config("test error".to_string());
        assert_eq!(err.to_string(), "Configuration error: test error");
    }

    #[test]
    fn test_database_error_display() {
        let err = AppError::Database(sqlx::Error::RowNotFound);
        let err_string = err.to_string();
        assert!(err_string.contains("Database error"));
    }

    #[test]
    fn test_internal_error_display() {
        let err = AppError::Internal(anyhow::anyhow!("internal error"));
        let err_string = err.to_string();
        assert!(err_string.contains("Internal server error"));
    }

    #[test]
    fn test_env_var_error_display() {
        let err = AppError::EnvVar(std::env::VarError::NotPresent);
        let err_string = err.to_string();
        assert!(err_string.contains("Environment variable error"));
    }

    #[test]
    fn test_database_error_into_response() {
        let err = AppError::Database(sqlx::Error::RowNotFound);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_config_error_into_response() {
        let err = AppError::Config("config error".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_internal_error_into_response() {
        let err = AppError::Internal(anyhow::anyhow!("test"));
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_env_var_error_into_response() {
        let err = AppError::EnvVar(std::env::VarError::NotPresent);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_from_sqlx() {
        let sqlx_err = sqlx::Error::RowNotFound;
        let app_err: AppError = sqlx_err.into();
        assert!(matches!(app_err, AppError::Database(_)));
    }

    #[test]
    fn test_error_from_anyhow() {
        let anyhow_err = anyhow::anyhow!("test error");
        let app_err: AppError = anyhow_err.into();
        assert!(matches!(app_err, AppError::Internal(_)));
    }

    #[test]
    fn test_error_from_env_var() {
        let env_err = std::env::VarError::NotPresent;
        let app_err: AppError = env_err.into();
        assert!(matches!(app_err, AppError::EnvVar(_)));
    }
}
