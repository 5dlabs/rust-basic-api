use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            Self::Database(_) => (StatusCode::SERVICE_UNAVAILABLE, "database_unavailable"),
            Self::Unexpected(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        };

        error!(error = ?self, code, status = status.as_u16(), "request failed");

        let body = ErrorBody {
            code,
            message: self.to_string(),
        };

        (status, Json(body)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
