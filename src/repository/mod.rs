//! Database repository module
//!
//! This module contains all database interaction logic and queries.

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Creates a `PostgreSQL` connection pool with optimal settings
///
/// # Arguments
///
/// * `database_url` - `PostgreSQL` connection string
///
/// # Returns
///
/// A configured `PgPool` instance or an error if connection fails
///
/// # Errors
///
/// Returns `sqlx::Error` if:
/// - Database URL is invalid
/// - Database is unreachable
/// - Authentication fails
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
}

#[cfg(test)]
pub mod test_utils;
