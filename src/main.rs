mod config;
mod error;
mod models;
mod repository;
mod routes;

use axum::{routing::get, Router};
use config::Config;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing with environment-based filtering
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment
    let config = Config::from_env()?;
    info!("Configuration loaded: {:?}", config);

    // Build application router
    let app = Router::new().route("/health", get(health_check));

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = TcpListener::bind(&addr).await?;

    info!("Server listening on {}", addr);

    // Start server using Axum 0.7 API
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}
