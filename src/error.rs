use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
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

/// API-level errors with structured responses
///
/// These errors are returned from API handlers and automatically converted
/// to appropriate HTTP responses with structured error messages.
#[derive(Debug, Error)]
pub enum ApiError {
    /// Request validation failed
    #[error("Validation error")]
    Validation(#[from] validator::ValidationErrors),

    /// Requested resource was not found
    #[error("Resource not found")]
    NotFound,

    /// Database operation failed
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    /// Internal server error
    #[error("Internal server error")]
    Internal(String),
}

/// Structured error response sent to API clients
///
/// This struct provides a consistent error format across all API endpoints.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Machine-readable error code
    pub error_code: String,

    /// Human-readable error message
    pub message: String,
}

/// Type alias for Results in API handlers
pub type ApiResult<T> = Result<T, ApiError>;

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

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            ApiError::Validation(ref errors) => {
                // Log validation errors for debugging
                tracing::warn!("Validation error: {:?}", errors);

                // Format validation errors into a readable message
                let error_messages: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            format!(
                                "{}: {}",
                                field,
                                error.message.as_ref().map_or("Invalid value", |m| m)
                            )
                        })
                    })
                    .collect();

                (
                    StatusCode::BAD_REQUEST,
                    "VALIDATION_ERROR",
                    error_messages.join(", "),
                )
            }
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                "Resource not found".to_string(),
            ),
            ApiError::Database(ref err) => {
                // Log database errors but don't expose details to clients
                tracing::error!("Database error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "A database error occurred".to_string(),
                )
            }
            ApiError::Internal(ref msg) => {
                // Log internal errors
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occurred".to_string(),
                )
            }
        };

        let error_response = ErrorResponse {
            error_code: error_code.to_string(),
            message,
        };

        (status, Json(error_response)).into_response()
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

    // ApiError tests
    #[test]
    fn test_api_error_not_found_response() {
        let error = ApiError::NotFound;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_api_error_database_response() {
        let error = ApiError::Database(sqlx::Error::RowNotFound);
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_api_error_internal_response() {
        let error = ApiError::Internal("test error".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_api_error_not_found_display() {
        let error = ApiError::NotFound;
        assert_eq!(error.to_string(), "Resource not found");
    }

    #[test]
    fn test_api_error_database_display() {
        let error = ApiError::Database(sqlx::Error::RowNotFound);
        assert_eq!(error.to_string(), "Database error");
    }

    #[test]
    fn test_api_error_internal_display() {
        let error = ApiError::Internal("test".to_string());
        assert_eq!(error.to_string(), "Internal server error");
    }

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse {
            error_code: "TEST_ERROR".to_string(),
            message: "Test message".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        assert!(json.contains("TEST_ERROR"));
        assert!(json.contains("Test message"));
    }
}
