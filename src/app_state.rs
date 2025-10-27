//! Application shared state

use sqlx_postgres::PgPool;

/// Global application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
