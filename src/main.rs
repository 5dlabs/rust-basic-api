mod config;
mod error;
mod models;
mod repository;
mod routes;

use axum::{routing::get, Router};
use config::Config;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_basic_api=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment
    let config =
        Config::from_env().map_err(|e| anyhow::anyhow!("Failed to load configuration: {e}"))?;

    tracing::info!("Starting rust-basic-api server");
    let db_status = if config.database_url.is_empty() {
        "Not set"
    } else {
        "Set"
    };
    tracing::info!("Database URL configured: {db_status}");

    // Build application router with health check endpoint
    let app = Router::new().route("/health", get(health_check));

    // Configure server address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on {addr}");

    // Start the HTTP server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Health check endpoint handler
///
/// Returns "OK" to indicate the service is running
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
