//! Database repository module
//!
//! This module contains all database interaction logic and queries,
//! including connection pool initialization and migrations.

use crate::config::Config;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Initialize a `PostgreSQL` connection pool using configuration parameters.
///
/// Runs database migrations on startup for secure, up-to-date schema.
///
/// # Errors
///
/// Returns an error if the database connection cannot be established or
/// if migrations fail to run successfully.
pub async fn init_pool_and_migrate(config: &Config) -> anyhow::Result<PgPool> {
    let options = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .acquire_timeout(Duration::from_secs(config.db_acquire_timeout_secs))
        .idle_timeout(Some(Duration::from_secs(config.db_idle_timeout_secs)));

    let connect_fut = options.connect(&config.database_url);
    let pool = tokio::time::timeout(
        Duration::from_secs(config.db_connect_timeout_secs),
        connect_fut,
    )
    .await
    .map_err(|e| anyhow::anyhow!("Timed out connecting to database: {e}"))??;

    // Run embedded migrations from the migrations/ folder
    // Safety: uses compile-time embedding to avoid runtime path traversal risks
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

/// Simple health check to verify the database connection is alive.
///
/// # Errors
///
/// Returns a `sqlx::Error` if the query cannot be executed.
pub async fn db_ping(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Avoid selecting any user data, prevents leaking information
    let _ = sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}

// Test utilities are included directly in integration tests.

#[cfg(test)]
mod tests {
    use std::env;

    #[tokio::test]
    async fn test_pool_init_env_missing_fails() {
        // Ensure DATABASE_URL is missing for this test
        env::remove_var("DATABASE_URL");
        let result = crate::config::Config::from_env();
        assert!(result.is_err());
    }
}
