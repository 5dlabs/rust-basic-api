//! Shared data models used across application layers.

use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository;

    #[tokio::test]
    async fn app_state_clone() {
        let pool = repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
            .expect("pool creation should succeed");

        let state = AppState { db_pool: pool };
        let cloned_state = state.clone();

        // Both states should reference the same pool
        assert!(!state.db_pool.is_closed());
        assert!(!cloned_state.db_pool.is_closed());
    }

    #[tokio::test]
    async fn app_state_debug_format() {
        let pool = repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
            .expect("pool creation should succeed");

        let state = AppState { db_pool: pool };
        let debug_str = format!("{state:?}");

        assert!(debug_str.contains("AppState"));
        assert!(debug_str.contains("db_pool"));
    }
}
