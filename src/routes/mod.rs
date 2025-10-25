pub mod user_routes;

use axum::{routing::get, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Register all application routes
pub fn register_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route(
            "/users",
            get(user_routes::get_users).post(user_routes::create_user),
        )
        .route(
            "/users/:id",
            get(user_routes::get_user)
                .put(user_routes::update_user)
                .delete(user_routes::delete_user),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "OK");
    }
}
