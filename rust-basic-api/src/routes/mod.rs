use std::sync::Arc;

use axum::{routing::get, Router};

pub mod health;

use crate::app_state::AppState;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health::health_check))
}
