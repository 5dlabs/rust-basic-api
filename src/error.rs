use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

/// Custom error type for API operations
#[derive(Debug)]
#[allow(dead_code)] // Variants will be used in future tasks
pub enum ApiError {
    /// Database-related errors
    Database(String),
    /// Configuration errors
    Configuration(String),
    /// Not found errors
    NotFound(String),
    /// Bad request errors
    BadRequest(String),
    /// Internal server errors
    Internal(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Database(msg) => write!(f, "Database error: {msg}"),
            ApiError::Configuration(msg) => write!(f, "Configuration error: {msg}"),
            ApiError::NotFound(msg) => write!(f, "Not found: {msg}"),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {msg}"),
            ApiError::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Database(msg)
            | ApiError::Configuration(msg)
            | ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            _ => ApiError::Database(err.to_string()),
        }
    }
}

impl From<std::env::VarError> for ApiError {
    fn from(err: std::env::VarError) -> Self {
        ApiError::Configuration(format!("Environment variable error: {err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let error = ApiError::NotFound("User not found".to_string());
        assert_eq!(error.to_string(), "Not found: User not found");

        let error = ApiError::Database("Connection failed".to_string());
        assert_eq!(error.to_string(), "Database error: Connection failed");
    }

    #[test]
    fn test_api_error_from_sqlx() {
        let sqlx_error = sqlx::Error::RowNotFound;
        let api_error: ApiError = sqlx_error.into();

        match api_error {
            ApiError::NotFound(_) => (),
            _ => panic!("Expected NotFound error"),
        }
    }
}
