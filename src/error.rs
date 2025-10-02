use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

// AppError will be used in Tasks 2-4 (Database, API, Authentication)
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Config(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            AppError::Database(_) | AppError::Internal => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
