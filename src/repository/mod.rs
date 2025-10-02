use sqlx::PgPool;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn connect() -> Result<PgPool, sqlx::Error> {
        let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;
        Ok(pool)
    }
}
