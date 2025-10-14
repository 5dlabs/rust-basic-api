//! Repository module tests
//!
//! Tests for database repository functions and pool management.

use std::time::Duration;

/// Test that `create_pool` returns a valid pool with correct configuration
#[tokio::test]
async fn test_create_pool_with_valid_url() {
    dotenvy::from_filename(".env.test").ok();

    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        // DATABASE_URL not set, skip test
        return;
    };

    let pool = rust_basic_api::repository::create_pool(&database_url)
        .await
        .expect("Should create pool with valid URL");

    // Verify pool configuration
    assert!(pool.size() > 0, "Pool should have connections");
}

/// Test that `create_pool` fails with invalid `URL`
#[tokio::test]
async fn test_create_pool_with_invalid_url() {
    let invalid_url = "postgresql://invalid_user:invalid_pass@nonexistent_host:9999/nonexistent_db";

    let result = rust_basic_api::repository::create_pool(invalid_url).await;

    // Should fail to connect
    assert!(result.is_err(), "Should fail with invalid database URL");
}

/// Test that `create_pool` fails with malformed `URL`
#[tokio::test]
async fn test_create_pool_with_malformed_url() {
    let malformed_url = "not-a-valid-url";

    let result = rust_basic_api::repository::create_pool(malformed_url).await;

    assert!(result.is_err(), "Should fail with malformed URL");
}

/// Test pool connection timeout behavior
#[tokio::test]
async fn test_pool_connection_timeout() {
    dotenvy::from_filename(".env.test").ok();

    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        // DATABASE_URL not set, skip test
        return;
    };

    let pool = rust_basic_api::repository::create_pool(&database_url)
        .await
        .expect("Should create pool");

    // Verify we can acquire a connection within timeout
    let start = std::time::Instant::now();
    let conn = pool.acquire().await;
    let elapsed = start.elapsed();

    assert!(conn.is_ok(), "Should acquire connection");
    assert!(
        elapsed < Duration::from_secs(5),
        "Should acquire connection quickly"
    );
}

/// Test that pool can handle multiple concurrent connections
#[tokio::test]
async fn test_pool_concurrent_connections() {
    dotenvy::from_filename(".env.test").ok();

    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        // DATABASE_URL not set, skip test
        return;
    };

    let pool = rust_basic_api::repository::create_pool(&database_url)
        .await
        .expect("Should create pool");

    // Spawn multiple concurrent tasks that use the pool
    let mut handles = vec![];

    for i in 0..5 {
        let pool_clone = pool.clone();
        let handle = tokio::spawn(async move {
            let conn = pool_clone.acquire().await;
            assert!(conn.is_ok(), "Connection {} should succeed", i);
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task should complete successfully");
    }
}

/// Test that pool connection can execute a simple query
#[tokio::test]
async fn test_pool_connection_query() {
    dotenvy::from_filename(".env.test").ok();

    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        // DATABASE_URL not set, skip test
        return;
    };

    let pool = rust_basic_api::repository::create_pool(&database_url)
        .await
        .expect("Should create pool");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations should succeed");

    // Execute a simple query
    let result = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await;

    assert!(result.is_ok(), "Simple query should succeed");
    assert_eq!(result.unwrap(), 1, "Query should return 1");
}
