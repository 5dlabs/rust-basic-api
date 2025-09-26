//! Database access layer utilities and helpers.

use std::str::FromStr;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;

pub type DbPool = PgPool;

pub fn create_pool(database_url: &str, max_connections: u32) -> Result<DbPool, sqlx::Error> {
    let connect_options = PgConnectOptions::from_str(database_url)?;

    Ok(PgPoolOptions::new()
        .max_connections(max_connections)
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
}
