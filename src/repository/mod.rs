//! Database repository module
//!
//! Contains database interaction logic and query implementations.

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Create a database connection pool with optimized settings
///
/// # Arguments
///
/// * `database_url` - `PostgreSQL` connection string
///
/// # Errors
///
/// Returns an error if the pool cannot be created or connection fails
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
