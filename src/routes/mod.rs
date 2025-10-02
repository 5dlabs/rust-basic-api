use axum::{routing::get, Router};

#[allow(dead_code)]
pub fn health_routes() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}
