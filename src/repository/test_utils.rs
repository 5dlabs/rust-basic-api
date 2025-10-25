//! Test utilities for database testing
//!
//! This module provides helper functions for setting up and managing test databases,
//! ensuring proper isolation and cleanup between tests.

use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Once;

static INIT: Once = Once::new();

/// Set up a test database connection pool
///
/// This function:
/// - Initializes environment from `.env.test` (only once)
/// - Creates a connection pool to the test database
/// - Runs all pending migrations
///
/// # Panics
///
/// This function will panic if:
/// - `DATABASE_URL` environment variable is not set
/// - Database connection fails
/// - Migrations fail to run
///
/// # Examples
///
/// ```no_run
/// use rust_basic_api::repository::test_utils::setup_test_database;
///
/// #[tokio::test]
/// async fn test_database_operation() {
///     let pool = setup_test_database().await;
///     // Use pool for testing
/// }
/// ```
pub async fn setup_test_database() -> PgPool {
    INIT.call_once(|| {
        // Load test environment variables (only once per test run)
        // Use override to replace placeholder values from workflow
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok();
        if let Some(dir) = manifest_dir {
            let env_path = std::path::Path::new(&dir).join(".env.test");
            if env_path.exists() {
                dotenvy::from_path_override(&env_path).ok();
            }
        } else {
            // Fallback to current directory
            dotenvy::from_filename_override(".env.test").ok();
        }
    });

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set for database tests (see .env.test.example)");

    let pool = super::create_pool(&database_url).await.unwrap_or_else(|e| {
        panic!("Failed to create test database pool. URL: '{database_url}', Error: {e:?}")
    });

    // Run migrations to ensure schema is up to date
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

/// Begin a new database transaction for test isolation
///
/// Transactions provide test isolation by allowing changes to be rolled back
/// after the test completes, keeping the database clean.
///
/// # Arguments
///
/// * `pool` - Database connection pool
///
/// # Returns
///
/// Returns a transaction that can be used for database operations
///
/// # Panics
///
/// Panics if transaction creation fails
///
/// # Examples
///
/// ```no_run
/// use rust_basic_api::repository::test_utils::{setup_test_database, transaction};
///
/// #[tokio::test]
/// async fn test_with_transaction() {
///     let pool = setup_test_database().await;
///     let mut tx = transaction(&pool).await;
///     
///     // Perform database operations
///     // Changes will be rolled back after test
/// }
/// ```
pub async fn transaction(pool: &PgPool) -> Transaction<'_, Postgres> {
    pool.begin().await.expect("Failed to begin transaction")
}

/// Clean up all data from the database
///
/// This function truncates all tables and resets sequences, providing a clean slate
/// for tests. Use this when you need to ensure complete isolation between tests.
///
/// # Arguments
///
/// * `pool` - Database connection pool
///
/// # Panics
///
/// Panics if cleanup operations fail
///
/// # Examples
///
/// ```no_run
/// use rust_basic_api::repository::test_utils::{setup_test_database, cleanup_database};
///
/// #[tokio::test]
/// async fn test_requiring_clean_database() {
///     let pool = setup_test_database().await;
///     cleanup_database(&pool).await;
///     
///     // Database is now empty
/// }
/// ```
pub async fn cleanup_database(pool: &PgPool) {
    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(pool)
        .await
        .expect("Failed to cleanup database");
}

// Note: Tests for test_utils functions are in tests/database_integration.rs
// since they require an actual database connection
