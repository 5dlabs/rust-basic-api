use std::sync::Arc;

use crate::{config::Config, repository::DbPool};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: DbPool,
}

impl AppState {
    pub fn new(config: Arc<Config>, db_pool: DbPool) -> Self {
        Self { config, db_pool }
    }
}
