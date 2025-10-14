//! Test utilities for database testing
//!
//! Provides helper functions for setting up test databases and managing transactions.

use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Once;

static INIT: Once = Once::new();

/// Set up a test database pool with migrations applied
///
/// This function ensures the test database is initialized only once
/// and migrations are run before any tests execute.
///
/// # Panics
///
/// Panics if the test database cannot be created or migrations fail
#[must_use]
pub async fn setup_test_database() -> PgPool {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.test").ok();
    });

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test or environment");

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

/// Create a database transaction for test isolation
///
/// # Panics
///
/// Panics if transaction creation fails
#[must_use]
#[allow(dead_code)]
pub async fn transaction(pool: &PgPool) -> Transaction<'_, Postgres> {
    pool.begin().await.expect("Failed to begin transaction")
}

/// Clean up test database by truncating all tables
///
/// # Errors
///
/// Returns an error if the cleanup operation fails
#[allow(dead_code)]
pub async fn cleanup_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup_test_database() {
        // This test verifies that setup_test_database can be called
        // In real usage, DATABASE_URL must be set
        if std::env::var("DATABASE_URL").is_ok() {
            let pool = setup_test_database().await;
            assert!(pool.size() > 0);
        }
    }
}
