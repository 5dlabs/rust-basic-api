mod config;
mod error;
mod models;
mod repository;
mod routes;
#[cfg(test)]
mod test_support;

use crate::config::Config;
use crate::repository as db;
use crate::routes::{router, AppState};
use anyhow::Context;
use axum::Router;
use std::net::SocketAddr;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = Config::from_env().context("failed to load configuration")?;

    let pool = db::create_pool(&config.database_url, &config.database)
        .context("failed to initialise database pool")?;
    db::ensure_connection(&pool)
        .await
        .context("database connectivity check failed")?;

    let state = AppState {
        db_pool: pool.clone(),
    };
    let app: Router<_> = router().with_state(state);

    let addr: SocketAddr = config.socket_addr();
    info!(address = %addr, "HTTP server listening");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

    Ok(())
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("rust_basic_api=info,axum=warn"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => info!("shutdown signal received"),
        Err(error) => error!(?error, "failed to listen for shutdown signal"),
    }
}
