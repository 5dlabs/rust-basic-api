use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::repository;

use super::AppState;

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    if let Err(error) = repository::ensure_connection(&state.pool).await {
        tracing::warn!(error = %error, "database connectivity check failed during health probe");
    }

    (StatusCode::OK, "OK")
}
