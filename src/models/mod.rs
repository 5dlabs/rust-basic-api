//! Shared data models used across application layers.

use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: PgPool,
}
