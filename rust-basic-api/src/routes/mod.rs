use crate::error::Result;
use crate::repository::{self, DbPool};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tracing::instrument;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

#[instrument(skip(state))]
async fn health_check(State(state): State<AppState>) -> Result<impl IntoResponse> {
    repository::ensure_connection(&state.db_pool).await?;
    Ok((StatusCode::OK, "OK"))
}
