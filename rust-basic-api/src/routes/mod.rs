mod health;

use axum::{routing::get, Router};

use crate::repository::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

impl AppState {
    #[must_use]
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

pub fn create_router() -> Router<AppState> {
    Router::new().route("/health", get(health::health_check))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_is_cloneable() {
        fn assert_clone<T: Clone>() {}
        assert_clone::<AppState>();
    }
}
