//! Integration tests for the Rust Basic API
//!
//! These tests verify the complete application behavior including:
//! - Server startup and configuration
//! - Health endpoint functionality
//! - Error handling and edge cases

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serial_test::serial;
use std::env;
use tower::ServiceExt;

/// Test helper to build the application router
fn build_app() -> axum::Router {
    axum::Router::new().route(
        "/health",
        axum::routing::get(rust_basic_api::routes::health_check),
    )
}

#[tokio::test]
#[serial]
async fn test_health_endpoint_returns_ok() {
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
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("\"status\""));
    assert!(body_str.contains("\"OK\""));
}

#[tokio::test]
#[serial]
async fn test_health_endpoint_json_response() {
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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "OK");
}

#[tokio::test]
#[serial]
async fn test_not_found_route() {
    let app = build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
#[serial]
async fn test_health_endpoint_multiple_calls() {
    let app = build_app();

    for _ in 0..5 {
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
    }
}

#[tokio::test]
#[serial]
async fn test_config_loading_success() {
    // Save original values
    let original_db = env::var("DATABASE_URL").ok();
    let original_port = env::var("SERVER_PORT").ok();

    // Set valid configuration
    env::set_var(
        "DATABASE_URL",
        "postgresql://test:test@localhost:5432/testdb",
    );
    env::set_var("SERVER_PORT", "8080");

    let result = rust_basic_api::config::Config::from_env();
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.server_port, 8080);
    assert!(config.database_url.contains("postgresql"));

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
    if let Some(val) = original_port {
        env::set_var("SERVER_PORT", val);
    } else {
        env::remove_var("SERVER_PORT");
    }
}

#[tokio::test]
#[serial]
async fn test_config_missing_database_url() {
    // Save original values
    let original_db = env::var("DATABASE_URL").ok();

    // Remove DATABASE_URL
    env::remove_var("DATABASE_URL");

    let result = rust_basic_api::config::Config::from_env();
    assert!(result.is_err());

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    }
}

#[tokio::test]
#[serial]
async fn test_server_port_validation() {
    // Save original values
    let original_db = env::var("DATABASE_URL").ok();
    let original_port = env::var("SERVER_PORT").ok();

    env::set_var(
        "DATABASE_URL",
        "postgresql://test:test@localhost:5432/testdb",
    );

    // Test valid ports
    for port in &["80", "3000", "8080", "65535"] {
        env::set_var("SERVER_PORT", port);
        let result = rust_basic_api::config::Config::from_env();
        assert!(result.is_ok(), "Port {port} should be valid");
    }

    // Test invalid ports (note: 0 is valid in Rust - means "any available port")
    for port in &["70000", "-1", "abc", "999999"] {
        env::set_var("SERVER_PORT", port);
        let result = rust_basic_api::config::Config::from_env();
        assert!(result.is_err(), "Port {port} should be invalid");
    }

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
    if let Some(val) = original_port {
        env::set_var("SERVER_PORT", val);
    } else {
        env::remove_var("SERVER_PORT");
    }
}

#[tokio::test]
#[serial]
async fn test_health_endpoint_with_different_methods() {
    let app = build_app();

    // GET should work
    let response = app
        .clone()
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

    // POST should not be allowed
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);

    // PUT should not be allowed
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}
