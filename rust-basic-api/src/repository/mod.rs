use crate::config::DatabaseConfig;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::instrument;

pub type DbPool = Pool<Postgres>;

pub fn init_pool(database_url: &str, config: &DatabaseConfig) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.acquire_timeout)
        .connect_lazy(database_url)
}

#[instrument(skip(pool))]
pub async fn ensure_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    let mut connection = pool.acquire().await?;
    sqlx::query("SELECT 1")
        .execute(&mut connection)
        .await
        .map(|_| ())
}
