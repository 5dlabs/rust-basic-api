//! Database access layer utilities and helpers.

use std::str::FromStr;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;

pub type DbPool = PgPool;

pub fn create_pool(database_url: &str, max_connections: u32) -> Result<DbPool, sqlx::Error> {
    let connect_options = PgConnectOptions::from_str(database_url)?;

    // Ensure at least 1 connection to prevent SQLx internal panics
    let actual_max_connections = if max_connections == 0 {
        1
    } else {
        max_connections
    };

    Ok(PgPoolOptions::new()
        .max_connections(actual_max_connections)
        .connect_lazy_with(connect_options))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn creates_lazy_pool_from_database_url() {
        let pool = create_pool("postgres://user:pass@localhost:5432/db", 5)
            .expect("expected lazy pool creation to succeed");
        assert!(!pool.is_closed());
    }

    #[tokio::test]
    async fn create_pool_with_various_max_connections() {
        // Test different max_connections values
        for max_conn in [1, 10, 100] {
            let pool = create_pool("postgres://user:pass@localhost:5432/db", max_conn)
                .expect("pool creation should succeed");
            assert!(!pool.is_closed());
        }
    }

    #[test]
    fn create_pool_with_invalid_url_format() {
        let result = create_pool("not_a_postgres_url", 5);
        assert!(result.is_err());
    }

    #[test]
    fn create_pool_with_empty_url() {
        let result = create_pool("", 5);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_pool_with_zero_max_connections() {
        // Zero max connections gets normalized to 1 to prevent SQLx panics
        let pool = create_pool("postgres://user:pass@localhost:5432/db", 0)
            .expect("pool creation should succeed with normalized connections");
        assert!(!pool.is_closed());
    }

    #[tokio::test]
    async fn create_pool_with_special_characters_in_url() {
        let pool = create_pool(
            "postgres://user%40example.com:p%40ss@localhost:5432/test-db",
            5,
        )
        .expect("should handle URL-encoded characters");
        assert!(!pool.is_closed());
    }

    #[tokio::test]
    async fn pool_can_be_closed() {
        let pool = create_pool("postgres://user:pass@localhost:5432/db", 5)
            .expect("pool creation should succeed");

        assert!(!pool.is_closed());

        pool.close().await;
        assert!(pool.is_closed());
    }

    #[tokio::test]
    async fn db_pool_type_alias() {
        let pool: DbPool = create_pool("postgres://user:pass@localhost:5432/db", 5)
            .expect("pool creation should succeed");
        assert!(!pool.is_closed());
    }
}
