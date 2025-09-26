use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    timestamp: u64,
    port: u16,
    pool_size: u32,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(handler))
}

async fn handler(State(state): State<AppState>) -> Json<HealthResponse> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    let port = state.config().server_port;
    let pool_size = state.database_pool().size();

    Json(HealthResponse {
        status: "OK",
        timestamp,
        port,
        pool_size,
    })
}
