mod config;
#[allow(dead_code)]
mod error;
mod models;
mod repository;
mod routes;

use axum::{routing::get, Router};
use config::Config;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
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
    let config = Config::from_env()?;
    tracing::info!(
        server_port = config.server_port,
        "Loaded server configuration"
    );
    tracing::debug!(
        has_database_url = !config.database_url.is_empty(),
        "Database settings loaded"
    );

    // Build application router with routes
    let app = Router::new().route("/health", get(health_check));

    // Create socket address
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), config.server_port);
    tracing::info!("Listening on {}", addr);

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
