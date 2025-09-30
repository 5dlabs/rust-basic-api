use crate::config::DatabaseConfig;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub fn create_pool(database_url: &str, settings: &DatabaseConfig) -> Result<DbPool, sqlx::Error> {
    let connect_options: PgConnectOptions = database_url.parse()?;

    Ok(PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .min_connections(settings.min_connections)
        .acquire_timeout(settings.acquire_timeout)
        .connect_lazy_with(connect_options))
}

pub async fn ensure_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
}
