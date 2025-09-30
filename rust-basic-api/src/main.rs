mod config;
mod error;
mod models;
mod repository;
mod routes;

use crate::config::Config;
use crate::repository::{ensure_connection, init_pool};
use crate::routes::AppState;
use anyhow::Context;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = Config::from_env().context("failed to load configuration")?;
    let pool = init_pool(&config.database_url, &config.database)
        .context("failed to initialize database pool")?;

    if let Err(error) = ensure_connection(&pool).await {
        tracing::warn!(error = %error, "database connectivity check failed");
    } else {
        tracing::info!("database connectivity check succeeded");
    }

    let state = AppState::new(pool.clone());
    let app = routes::create_router().with_state(state);
    let addr = config.socket_addr();

    tracing::info!(%addr, "starting HTTP server");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("server encountered an unrecoverable error")?;

    Ok(())
}

fn init_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
