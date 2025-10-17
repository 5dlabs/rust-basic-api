//! Main application entry point.
//!
//! This module initializes the HTTP server with Axum framework, sets up tracing,
//! and configures the application routes.

mod config;
mod error;
mod models;
mod repository;
mod routes;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rust_basic_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment
    let config = Config::from_env().map_err(error::AppError::from)?;
    tracing::info!("Configuration loaded successfully");
    tracing::debug!("Database URL configured: {}", &config.database_url);

    // Build application router with health check endpoint
    let app = Router::new().route("/health", get(health_check));

    // Configure server address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Starting server on {}", addr);

    // Start the HTTP server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint handler.
///
/// Returns a simple "OK" response to indicate the service is running.
async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "OK");
    }
}
