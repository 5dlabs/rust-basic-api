//! Test utilities for database testing
//!
//! Provides helper functions for setting up and tearing down test databases.

use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Once;

static INIT: Once = Once::new();

/// Sets up a test database pool with migrations
///
/// This function ensures that the test database is initialized only once
/// and runs all migrations before returning the pool.
///
/// # Panics
///
/// Panics if:
/// - `DATABASE_URL` environment variable is not set in `.env.test`
/// - Database connection fails
/// - Migrations fail to run
///
/// # Example
///
/// ```no_run
/// # use rust_basic_api::repository::test_utils::setup_test_database;
/// # async {
/// let pool = setup_test_database().await;
/// // Use pool for testing
/// # };
/// ```
pub async fn setup_test_database() -> PgPool {
    INIT.call_once(|| {
        dotenv::from_filename(".env.test").ok();
    });

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test for testing");

    let pool = super::create_pool(&database_url)
        .await
        .expect("Failed to create test database pool");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

/// Creates a new transaction for test isolation
///
/// # Panics
///
/// Panics if transaction creation fails
///
/// # Example
///
/// ```no_run
/// # use rust_basic_api::repository::test_utils::{setup_test_database, transaction};
/// # async {
/// let pool = setup_test_database().await;
/// let mut tx = transaction(&pool).await;
/// // Use transaction for testing - will be rolled back
/// tx.rollback().await.unwrap();
/// # };
/// ```
#[allow(dead_code)]
pub async fn transaction(pool: &PgPool) -> Transaction<'_, Postgres> {
    pool.begin().await.expect("Failed to begin transaction")
}

/// Cleans up the database by truncating all tables
///
/// This function truncates the users table and resets the ID sequence,
/// effectively clearing all test data.
///
/// # Panics
///
/// Panics if the cleanup query fails
///
/// # Example
///
/// ```no_run
/// # use rust_basic_api::repository::test_utils::{setup_test_database, cleanup_database};
/// # async {
/// let pool = setup_test_database().await;
/// // Run tests...
/// cleanup_database(&pool).await;
/// # };
/// ```
pub async fn cleanup_database(pool: &PgPool) {
    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(pool)
        .await
        .expect("Failed to cleanup database");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup_and_cleanup() {
        let pool = setup_test_database().await;

        // Use a transaction for test isolation
        let mut tx = pool.begin().await.expect("Failed to begin transaction");

        // Insert a test user
        let id = sqlx::query_scalar::<_, i32>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
        )
        .bind("Test User")
        .bind("test_utils_test@example.com")
        .fetch_one(&mut *tx)
        .await
        .expect("Failed to insert test user");

        // Verify user exists
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .expect("Failed to count users");
        assert_eq!(count, 1);

        // Commit the transaction
        tx.commit().await.expect("Failed to commit transaction");

        // Clean up
        cleanup_database(&pool).await;

        // Verify cleanup worked
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .expect("Failed to count users after cleanup");
        assert_eq!(count, 0);
    }
}
