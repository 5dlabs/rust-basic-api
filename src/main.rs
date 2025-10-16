//! REST API server built with Axum
//!
//! This application provides a production-ready REST API with database connectivity,
//! structured logging, and proper error handling.

mod config;
mod error;
mod models;
mod repository;
mod routes;

use config::Config;
use std::net::SocketAddr;

use axum::{routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment variables
    let config = Config::from_env()?;
    tracing::info!(
        "Configuration loaded: port={}, database={}",
        config.server_port,
        &config.database_url[..config
            .database_url
            .find('@')
            .unwrap_or(config.database_url.len())]
    );

    // Build application router with routes
    let app = Router::new().route("/health", get(routes::health_check));

    // Start the HTTP server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
