//! Test utilities for database integration tests
#![cfg_attr(not(test), allow(dead_code))]
//!
//! Provides helpers to create an isolated database context using transactions
//! and to verify schema existence. Tests using these utilities should be
//! marked with `#[ignore]` by default unless a test database is available.

use sqlx::query_scalar::query_scalar;
use sqlx::transaction::Transaction;
use sqlx_postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Create a connection pool for tests using `DATABASE_URL`.
///
/// The pool uses conservative defaults to avoid exhausting local resources.
///
/// # Errors
///
/// Returns an error if `DATABASE_URL` is not set or if a connection
/// to the database cannot be established.
#[cfg_attr(not(test), allow(dead_code))]
pub async fn test_pool() -> anyhow::Result<PgPool> {
    // Load test env if present
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Some(Duration::from_secs(60)))
        .connect(&database_url)
        .await?;

    // Ensure migrations are applied for the test database
    sqlx::migrate::Migrator::new(std::path::Path::new("migrations"))
        .await?
        .run(&pool)
        .await?;

    Ok(pool)
}

/// Start a transaction for a test and return it. Drop will rollback by default.
///
/// # Errors
///
/// Returns a `sqlx::Error` if a transaction cannot be started.
#[cfg_attr(not(test), allow(dead_code))]
pub async fn begin_tx(
    pool: &PgPool,
) -> Result<Transaction<'_, sqlx_postgres::Postgres>, sqlx::Error> {
    pool.begin().await
}

/// Check whether the `users` table exists.
///
/// # Errors
///
/// Returns a `sqlx::Error` if the metadata query fails.
#[cfg_attr(not(test), allow(dead_code))]
pub async fn users_table_exists(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let rec = query_scalar::<_, i64>(
        "
        SELECT COUNT(*)
        FROM information_schema.tables
        WHERE table_schema = 'public'
          AND table_name = 'users'
        ",
    )
    .fetch_one(pool)
    .await?;
    Ok(rec > 0)
}
