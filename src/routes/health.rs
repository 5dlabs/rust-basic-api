use axum::{extract::State, routing::get, Router};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(handler))
}

async fn handler(State(state): State<AppState>) -> &'static str {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    tracing::debug!(
        port = state.config().server_port,
        pool_size = state.database_pool().size(),
        ?timestamp,
        "health check succeeded",
    );

    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, repository, AppState};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use hyper::body::to_bytes;
    use std::net::IpAddr;
    use tower::ServiceExt as _;

    #[tokio::test]
    async fn health_endpoint_exposes_runtime_status() {
        let config = Config {
            database_url: "postgresql://postgres:postgres@localhost:5432/health_test".to_string(),
            database_max_connections: 3,
            server_host: IpAddr::from([127, 0, 0, 1]),
            server_port: 4001,
            rust_log: "info".to_string(),
        };

        let db_pool = repository::init_pool(&config.database_url, config.database_max_connections)
            .expect("pool should initialise lazily");

        let state = AppState::new(config, db_pool);
        let app = router().with_state(state.clone());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("request builds"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body())
            .await
            .expect("body should read");
        assert_eq!(body.as_ref(), b"OK");
    }
}
