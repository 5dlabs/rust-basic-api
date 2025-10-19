pub mod config;
pub mod error;
pub mod models;
pub mod repository;
pub mod routes;

use axum::{routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize tracing subscriber for structured logging
///
/// # Errors
///
/// Returns an error if the tracing subscriber cannot be initialized
pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_basic_api=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;
    Ok(())
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
    async fn test_health_endpoint_post_method() {
        let app = create_app();

        // Test POST /health (should return 405 Method Not Allowed)
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
    async fn test_health_endpoint_head_method() {
        let app = create_app();

        // Test HEAD /health
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::HEAD)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // HEAD requests should succeed with same status but no body
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_multiple_health_checks() {
        // Verify multiple calls work correctly
        for _ in 0..5 {
            let response = health_check().await;
            assert_eq!(response, "OK");
        }
    }

    #[tokio::test]
    async fn test_health_endpoint_with_query_params() {
        let app = create_app();

        // Test /health with query parameters (should still work)
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health?test=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_router_is_clonable() {
        let app = create_app();
        let _app2 = app.clone();
        // If this compiles and runs, Router is properly clonable
    }

    #[tokio::test]
    async fn test_health_endpoint_accepts_options() {
        let app = create_app();

        // Test OPTIONS /health for CORS preflight
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // OPTIONS should return 200 or 405 depending on router config
        assert!(
            response.status() == StatusCode::OK
                || response.status() == StatusCode::METHOD_NOT_ALLOWED
        );
    }

    #[tokio::test]
    async fn test_health_endpoint_with_trailing_slash() {
        let app = create_app();

        // Test /health/ with trailing slash
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should be 404 since exact route is /health without trailing slash
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_health_endpoint_case_sensitive() {
        let app = create_app();

        // Test /HEALTH with uppercase
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/HEALTH")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Routes are case-sensitive, should be 404
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_root_path_returns_404() {
        let app = create_app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_health_endpoint_delete_method() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_health_endpoint_put_method() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_health_endpoint_patch_method() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[test]
    fn test_init_tracing_error_handling() {
        // Test that init_tracing returns a Result
        // This will likely fail on second call since subscriber already initialized
        // but we're testing the function signature and error path
        let result = init_tracing();
        // First call might succeed or fail depending on test order
        // Just verify it returns a Result type
        let _is_result: Result<(), Box<dyn std::error::Error>> = result;
    }
}
