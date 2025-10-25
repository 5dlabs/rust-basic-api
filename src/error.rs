use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
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

/// API-level errors for HTTP responses
#[derive(Debug, Error)]
pub enum ApiError {
    /// Resource not found
    #[error("Resource not found")]
    NotFound,

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Database error
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    /// Internal server error
    #[error("Internal server error")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "Resource not found".to_string(),
            ),
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg),
            ApiError::Database(ref err) => {
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "Database error occurred".to_string(),
                )
            }
            ApiError::Internal(ref msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": error_code,
            "message": message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[test]
    fn test_app_error_display() {
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

    #[test]
    fn test_api_error_not_found() {
        let err = ApiError::NotFound;
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_api_error_validation() {
        let err = ApiError::Validation("Invalid email".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_api_error_database() {
        let err = ApiError::Database(sqlx::Error::RowNotFound);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_api_error_internal() {
        let err = ApiError::Internal("Something went wrong".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_api_error_display() {
        assert_eq!(ApiError::NotFound.to_string(), "Resource not found");
        assert_eq!(
            ApiError::Validation("test".to_string()).to_string(),
            "Validation error: test"
        );
    }
}
