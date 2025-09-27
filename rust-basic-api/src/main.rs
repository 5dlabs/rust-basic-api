mod config;
mod error;
mod models;
mod repository;
mod routes;

use axum::Router;
use config::Config;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Health check endpoint handler
///
/// Returns a simple "OK" response to indicate the server is running
async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for structured logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment variables
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully");

    // Build application router with health check endpoint
    let app = Router::new().route("/health", axum::routing::get(health_check));

    // Create socket address for the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Starting server on {}", addr);

    // Start the HTTP server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
