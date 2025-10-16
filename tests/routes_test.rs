//! Comprehensive route handler tests

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn build_app() -> axum::Router {
    axum::Router::new().route(
        "/health",
        axum::routing::get(rust_basic_api::routes::health_check),
    )
}

#[tokio::test]
async fn test_health_check_response_structure() {
    let app = build_app();

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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json.is_object());
    assert!(json.get("status").is_some());
    assert_eq!(json["status"].as_str().unwrap(), "OK");
}

#[tokio::test]
async fn test_health_check_idempotency() {
    let app = build_app();

    // Call health check multiple times
    for i in 0..10 {
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

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Call {} should return OK",
            i
        );

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "OK", "Call {} should have OK status", i);
    }
}

#[tokio::test]
async fn test_invalid_routes() {
    let app = build_app();

    let invalid_paths = vec![
        "/",
        "/api",
        "/health/check",
        "/healthz",
        "/ready",
        "/status",
        "/ping",
    ];

    for path in invalid_paths {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "Path {} should return NOT_FOUND",
            path
        );
    }
}

#[tokio::test]
async fn test_health_endpoint_case_sensitivity() {
    let app = build_app();

    // Only lowercase /health should work
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

    // Uppercase should not work
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/HEALTH")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_health_endpoint_with_query_params() {
    let app = build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health?foo=bar&baz=qux")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Query params should not affect the health check
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_endpoint_headers() {
    let app = build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .header("user-agent", "test-client/1.0")
                .header("x-custom-header", "custom-value")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_check_concurrent_requests() {
    let app = build_app();

    let mut handles = vec![];

    for _ in 0..5 {
        let app_clone = app.clone();
        let handle = tokio::spawn(async move {
            app_clone
                .oneshot(
                    Request::builder()
                        .uri("/health")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap()
        });
        handles.push(handle);
    }

    for handle in handles {
        let response = handle.await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
