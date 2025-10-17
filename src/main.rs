//! Rust Basic API - Main application entry point.
//!
//! This REST API server is built with Axum and provides a foundation for
//! building production-ready services with database connectivity, structured
//! logging, and proper error handling.

mod config;
mod error;
mod models;
mod repository;
mod routes;

use axum::{routing::get, Router};
use config::Config;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_basic_api=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rust Basic API");

    // Load configuration from environment variables
    let config = Config::from_env().map_err(|e| {
        tracing::error!("Failed to load configuration: {e}");
        anyhow::anyhow!("Configuration error: {e}")
    })?;

    tracing::info!("Configuration loaded successfully");

    // Build the application router with health check endpoint
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http());

    // Create socket address for the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on {addr}");

    // Start the HTTP server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint handler.
///
/// Returns a simple "OK" response to indicate the service is running.
/// This endpoint can be used by load balancers, container orchestrators,
/// and monitoring systems to verify service health.
///
/// # Returns
///
/// A static string "OK" with HTTP 200 status.
async fn health_check() -> &'static str {
    tracing::debug!("Health check requested");
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
