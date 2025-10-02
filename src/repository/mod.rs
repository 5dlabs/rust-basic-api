use sqlx::PgPool;

#[allow(dead_code)]
pub async fn connect_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    #[allow(dead_code)]
    PgPool::connect(database_url).await
}
