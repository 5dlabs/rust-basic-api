mod config;
#[allow(dead_code)]
mod error;
mod models;
mod repository;
mod routes;

use axum::Router;
use config::Config;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for structured logging
    init_tracing();

    // Load configuration from environment
    let config = Config::from_env()?;
    tracing::info!(
        server_port = config.server_port,
        "Loaded server configuration"
    );
    tracing::debug!(
        has_database_url = !config.database_url.is_empty(),
        "Database settings loaded"
    );

    // Build application router with routes
    let app = create_app();

    // Create socket address
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), config.server_port);
    tracing::info!("Listening on {}", addr);

    // Start the HTTP server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Initialize tracing subscriber for structured logging
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_basic_api=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Create the application router with all routes
///
/// # Returns
///
/// A configured `Router` with all application routes
pub fn create_app() -> Router {
    routes::register_routes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_create_app() {
        let app = create_app();

        // Verify the router is created successfully
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_health_endpoint_integration() {
        let app = create_app();

        // Test GET /health
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Read and verify response body
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"OK");
    }

    #[tokio::test]
    async fn test_unknown_route_returns_404() {
        let app = create_app();

        // Test unknown route
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/unknown")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_init_tracing() {
        // This test verifies that init_tracing doesn't panic
        // We can't call it directly as it would initialize the global subscriber
        // which can only be done once per process
        // Instead, we just verify the function exists and compiles
        let _ = init_tracing as fn();
    }
}
