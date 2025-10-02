mod config;
mod error;
mod models;
mod repository;
mod routes;

use crate::routes::user_routes; // Add routes module
use anyhow::Result;
use axum::{routing::get, Router};
use config::Config;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing with environment-based filtering
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration from environment
    let config = Config::from_env()?;
    tracing::info!(
        database_url = %config.database_url,
        server_port = %config.server_port,
        "Configuration loaded"
    );

    // Build application router with health check endpoint
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(user_routes()); // Merge user routes

    // Start HTTP server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!(address = %addr, "Starting server");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Health check endpoint returning "OK" status
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
