//! Rust Basic API
//!
//! A production-ready REST API built with Axum framework.

mod config;
mod error;
mod models;
mod repository;
mod routes;

use crate::config::Config;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_basic_api=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment
    let config =
        Config::from_env().map_err(|e| anyhow::anyhow!("Failed to load configuration: {e}"))?;

    tracing::info!(
        database_url_configured = !config.database_url.is_empty(),
        port = config.server_port,
        "Configuration loaded"
    );

    // Build application router
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(routes::build_routes())
        .layer(TraceLayer::new_for_http());

    // Create socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on {addr}");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint handler
///
/// Returns a simple "OK" status to indicate the server is running.
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
