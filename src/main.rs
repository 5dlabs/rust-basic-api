mod config;
mod error;
mod models;
mod repository;
mod routes;

use std::{net::SocketAddr, sync::Arc};

use config::Config;
use error::{AppError, Result};
use repository::DatabasePool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Clone)]
pub(crate) struct AppState {
    config: Arc<Config>,
    db_pool: DatabasePool,
}

impl AppState {
    pub(crate) fn config(&self) -> &Config {
        &self.config
    }

    pub(crate) fn database_pool(&self) -> &DatabasePool {
        &self.db_pool
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;
    init_tracing(&config);

    let db_pool = repository::init_pool(&config.database_url, config.database_max_connections)?;
    let state = AppState {
        config: Arc::new(config),
        db_pool,
    };

    let router = routes::create_router().with_state(state.clone());

    let addr: SocketAddr = state.config().socket_addr();
    tracing::info!(%addr, "starting HTTP server");

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|error| AppError::Other(error.into()))
}

fn init_tracing(config: &Config) {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &config.rust_log);
    }

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.rust_log)),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
