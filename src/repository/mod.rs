use sqlx::{postgres::PgPoolOptions, PgPool};

pub type DatabasePool = PgPool;

/// Initialise a lazily connected `PostgreSQL` connection pool.
pub fn init_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_lazy(database_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init_pool_respects_max_connections() {
        let pool = init_pool(
            "postgresql://postgres:postgres@localhost:5432/repository_test",
            7,
        )
        .expect("pool should initialise");

        assert_eq!(pool.size(), 0);
    }
}
