mod config;
#[allow(dead_code)]
mod error;
mod models;
mod repository;
mod routes;

use axum::{routing::get, Router};
use config::Config;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for structured logging
    init_tracing();

    // Load configuration and run server
    run_application().await
}

/// Run the application with configuration loaded from environment
///
/// # Errors
///
/// Returns an error if configuration loading fails or server startup fails
pub async fn run_application() -> anyhow::Result<()> {
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

    // Create socket address and start server
    start_server(app, config.server_port).await
}

/// Start the HTTP server on the specified port
///
/// # Errors
///
/// Returns an error if the server fails to bind or start
pub async fn start_server(app: Router, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    tracing::info!("Listening on {}", addr);

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
    Router::new().route("/health", get(health_check))
}

/// Health check endpoint handler
///
/// Returns "OK" to indicate the service is running
async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "OK");
    }

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

    #[tokio::test]
    async fn test_health_endpoint_post_method_not_allowed() {
        let app = create_app();

        // Test POST to /health (should be method not allowed)
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_health_endpoint_returns_text() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Verify response is successful
        assert!(response.status().is_success());

        // Verify body content
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();
        assert_eq!(body_str, "OK");
    }

    #[tokio::test]
    async fn test_multiple_health_checks() {
        let app = create_app();

        // Make multiple requests to verify idempotency
        for _ in 0..3 {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri("/health")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            assert_eq!(&body[..], b"OK");
        }
    }

    #[tokio::test]
    async fn test_router_has_health_route() {
        let app = create_app();

        // Verify that /health route exists and is accessible
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should not be 404
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_init_tracing_function_exists() {
        // Verify the function signature is correct
        let _: fn() = init_tracing;
    }

    #[tokio::test]
    async fn test_run_application_with_missing_env() {
        // Remove DATABASE_URL to test error handling
        std::env::remove_var("DATABASE_URL");

        let result = run_application().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_run_application_config_validation() {
        // Test that run_application properly propagates config errors
        std::env::remove_var("DATABASE_URL");

        let result = run_application().await;
        assert!(result.is_err(), "Should fail with missing DATABASE_URL");
    }

    #[test]
    fn test_socket_addr_creation() {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        // Test socket address creation with different ports
        let port = 3000;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
        assert_eq!(addr.port(), 3000);
        assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::UNSPECIFIED));

        let port = 8080;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_create_app_returns_router() {
        let app = create_app();
        // Verify we can create a router without panicking
        let _ = format!("{app:?}");
    }
}
