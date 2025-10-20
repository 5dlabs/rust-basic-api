use axum::{routing::get, Router};

async fn health_check() -> &'static str {
    "OK"
}

pub fn register_routes() -> Router {
    Router::new().route("/health", get(health_check))
}
