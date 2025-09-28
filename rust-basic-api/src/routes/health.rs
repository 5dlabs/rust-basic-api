use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::{app_state::AppState, error::AppResult};

pub async fn health_check(State(state): State<Arc<AppState>>) -> AppResult<impl IntoResponse> {
    tracing::debug!(
        server_port = state.config.server_port,
        "health check invoked"
    );

    if state.db_pool.is_closed() {
        tracing::warn!("database connection pool is closed");
        return Ok((StatusCode::SERVICE_UNAVAILABLE, "UNAVAILABLE"));
    }

    Ok((StatusCode::OK, "OK"))
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::{body::Body, http::Request, Router};
    use tower::ServiceExt;

    use crate::{
        app_state::AppState, config::Config, repository::create_pool, routes::create_router,
    };

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let config = Arc::new(Config {
            database_url: "postgresql://example".into(),
            database_max_connections: 5,
            server_host: std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
            server_port: 3000,
        });

        let pool = create_pool(&config.database_url, config.database_max_connections)
            .expect("lazy pool creation should succeed");

        let state = Arc::new(AppState::new(Arc::clone(&config), pool));
        let router: Router = create_router().with_state(state);

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn health_endpoint_returns_unavailable_when_pool_closed() {
        let config = Arc::new(Config {
            database_url: "postgresql://example".into(),
            database_max_connections: 5,
            server_host: std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
            server_port: 3000,
        });

        let pool = create_pool(&config.database_url, config.database_max_connections)
            .expect("lazy pool creation should succeed");
        pool.close().await;

        let state = Arc::new(AppState::new(Arc::clone(&config), pool));
        let router: Router = create_router().with_state(state);

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
