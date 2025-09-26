use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::AppState;

#[derive(Debug, Serialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, repository, AppState};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use hyper::body::to_bytes;
    use std::{convert::TryFrom, net::IpAddr};
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

        let bytes = to_bytes(response.into_body())
            .await
            .expect("body should read");
        let payload: serde_json::Value = serde_json::from_slice(&bytes).expect("valid json");

        assert_eq!(payload["status"], "OK");

        let port_value = payload["port"].as_u64().expect("port should be number");
        let port = u16::try_from(port_value).expect("port should fit into u16");
        assert_eq!(port, state.config().server_port);

        let pool_size_value = payload["pool_size"]
            .as_u64()
            .expect("pool_size should be number");
        let pool_size = u32::try_from(pool_size_value).expect("pool_size should fit into u32");
        assert_eq!(pool_size, state.database_pool().size());

        assert!(payload["timestamp"].as_u64().expect("timestamp present") > 0);
    }
}
