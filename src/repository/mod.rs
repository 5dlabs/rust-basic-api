// Repository module - database interaction layer
// This module contains database queries and connection pool management

pub mod user_repository;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

// Re-export commonly used types for convenience
pub use user_repository::{SqlxUserRepository, UserRepository};

/// Create a `PostgreSQL` connection pool with optimized settings
///
/// # Arguments
///
/// * `database_url` - `PostgreSQL` connection string
///
/// # Returns
///
/// Returns a configured `PgPool` on success, or an error if connection fails
///
/// # Errors
///
/// This function will return an error if:
/// - The database URL is invalid
/// - The database is unreachable
/// - Connection timeout is exceeded
/// - Authentication fails
///
/// # Pool Configuration
///
/// - `max_connections`: 10 - Maximum number of concurrent connections
/// - `min_connections`: 2 - Minimum connections kept alive
/// - `acquire_timeout`: 3s - Time to wait for an available connection
/// - `idle_timeout`: 600s (10 minutes) - Time before idle connections are closed
/// - `max_lifetime`: 1800s (30 minutes) - Maximum connection lifetime
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

// Test utilities - always compiled but only used in tests
pub mod test_utils;
