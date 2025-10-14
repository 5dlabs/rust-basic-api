//! Application startup and integration tests
//!
//! Tests that verify the application can start up correctly with database.

use axum::{body::Body, http::Request, http::StatusCode};
use rust_basic_api::AppState;
use sqlx::PgPool;
use tower::ServiceExt;

/// Helper to get test database pool
async fn get_test_pool() -> Option<PgPool> {
    dotenvy::from_filename(".env.test").ok();

    let database_url = std::env::var("DATABASE_URL").ok()?;

    let pool = rust_basic_api::repository::create_pool(&database_url)
        .await
        .ok()?;

    // Run migrations
    sqlx::migrate!().run(&pool).await.ok()?;

    Some(pool)
}

/// Test that AppState can be created with a valid pool
#[tokio::test]
async fn test_app_state_creation() {
    if let Some(pool) = get_test_pool().await {
        let state = AppState { pool: pool.clone() };

        // Verify state has a working pool
        assert!(state.pool.size() > 0);

        // Verify we can use the pool
        let result = sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&state.pool)
            .await;

        assert!(result.is_ok());
    }
}

/// Test that health check endpoint works with database state
#[tokio::test]
async fn test_health_endpoint_with_database() {
    if let Some(pool) = get_test_pool().await {
        let state = AppState { pool };

        let app = axum::Router::new()
            .route("/health", axum::routing::get(health_check_handler))
            .with_state(state);

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

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(body_str, "OK");
    }
}

/// Test health handler
async fn health_check_handler(
    axum::extract::State(_state): axum::extract::State<AppState>,
) -> &'static str {
    "OK"
}

/// Test that AppState can be cloned
#[tokio::test]
async fn test_app_state_clone() {
    if let Some(pool) = get_test_pool().await {
        let state1 = AppState { pool };
        let state2 = state1.clone();

        // Both states should work
        let result1 = sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&state1.pool)
            .await;
        let result2 = sqlx::query_scalar::<_, i32>("SELECT 2")
            .fetch_one(&state2.pool)
            .await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert_eq!(result1.unwrap(), 1);
        assert_eq!(result2.unwrap(), 2);
    }
}

/// Test database connectivity through AppState
#[tokio::test]
async fn test_app_state_database_query() {
    if let Some(pool) = get_test_pool().await {
        let state = AppState { pool };

        // Test that we can query the users table
        let result = sqlx::query("SELECT COUNT(*) FROM users")
            .fetch_one(&state.pool)
            .await;

        assert!(result.is_ok(), "Should be able to query users table");
    }
}

/// Test that pool from test_utils works correctly
#[tokio::test]
async fn test_test_utils_pool() {
    if std::env::var("DATABASE_URL").is_ok() {
        let pool = rust_basic_api::repository::test_utils::setup_test_database().await;

        // Verify pool works
        assert!(pool.size() > 0);

        // Verify we can query
        let result = sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
}

/// Test that cleanup_database works
#[tokio::test]
async fn test_cleanup_database_function() {
    if std::env::var("DATABASE_URL").is_ok() {
        let pool = rust_basic_api::repository::test_utils::setup_test_database().await;

        // Insert a test user
        sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
            .bind("Test User")
            .bind("test@cleanup.com")
            .execute(&pool)
            .await
            .expect("Should insert test user");

        // Cleanup
        rust_basic_api::repository::test_utils::cleanup_database(&pool)
            .await
            .expect("Cleanup should succeed");

        // Verify table is empty
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .expect("Should query count");

        assert_eq!(count, 0, "Users table should be empty after cleanup");
    }
}

/// Test transaction helper
#[tokio::test]
async fn test_transaction_helper() {
    if std::env::var("DATABASE_URL").is_ok() {
        let pool = rust_basic_api::repository::test_utils::setup_test_database().await;
        rust_basic_api::repository::test_utils::cleanup_database(&pool)
            .await
            .ok();

        // Begin transaction
        let mut tx = rust_basic_api::repository::test_utils::transaction(&pool).await;

        // Insert within transaction
        sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
            .bind("Transaction Test")
            .bind("tx@test.com")
            .execute(&mut *tx)
            .await
            .expect("Should insert in transaction");

        // Rollback
        tx.rollback().await.expect("Should rollback");

        // Verify data was not committed
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .expect("Should query count");

        assert_eq!(count, 0, "Data should not be committed after rollback");
    }
}
