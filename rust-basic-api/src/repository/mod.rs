use sqlx::{postgres::PgPoolOptions, PgPool};

pub type DbPool = PgPool;

/// Create a lazy `PostgreSQL` connection pool.
///
/// The pool defers the initial connection until the first query to avoid
/// requiring a live database during application startup.
pub fn create_pool(database_url: &str, max_connections: u32) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_lazy(database_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_pool_uses_lazy_connections() {
        let result = create_pool("postgresql://invalid", 1);

        assert!(
            result.is_ok(),
            "lazy pool creation should succeed even without a live database"
        );
    }
}
