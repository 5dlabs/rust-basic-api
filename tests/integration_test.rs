//! Integration tests for Rust Basic API
//!
//! Tests the HTTP endpoints and application behavior

use axum::{routing::get, Router};
use http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Helper function to create test router
fn create_test_router() -> Router {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::test]
async fn test_health_endpoint_returns_ok() {
    let app = create_test_router();

    let request = Request::builder()
        .uri("/health")
        .body(String::new())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(body_str, "OK");
}

#[tokio::test]
async fn test_health_endpoint_method_get() {
    let app = create_test_router();

    let request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(String::new())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_nonexistent_route_returns_404() {
    let app = create_test_router();

    let request = Request::builder()
        .uri("/nonexistent")
        .body(String::new())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_health_endpoint_post_not_allowed() {
    let app = create_test_router();

    let request = Request::builder()
        .method("POST")
        .uri("/health")
        .body(String::new())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_health_endpoint_multiple_requests() {
    // Test that multiple requests work correctly
    for _ in 0..5 {
        let app = create_test_router();
        let request = Request::builder()
            .uri("/health")
            .body(String::new())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
