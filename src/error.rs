use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

// Allow dead_code for ApiError as it's a placeholder for future error handling
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            Self::Internal(ref e) => {
                tracing::error!("Internal error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (status, error_message).into_response()
    }
}
