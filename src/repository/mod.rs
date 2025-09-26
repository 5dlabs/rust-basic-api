use sqlx::{postgres::PgPoolOptions, PgPool};

pub type DatabasePool = PgPool;

/// Initialise a lazily connected `PostgreSQL` connection pool.
pub fn init_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_lazy(database_url)
}
