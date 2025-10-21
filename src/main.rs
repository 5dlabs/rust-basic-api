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
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rust Basic API server");

    // Load configuration from environment variables
    let config = Config::from_env()?;
    tracing::info!(
        "Configuration loaded: database_url={}, server_port={}",
        mask_database_url(&config.database_url),
        config.server_port
    );

    // Build application router with health check endpoint
    let app = Router::new().route("/health", get(health_check));

    // Create socket address from configuration
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on {}", addr);

    // Start the HTTP server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Health check endpoint handler
///
/// Returns a simple "OK" response to indicate the service is running
async fn health_check() -> &'static str {
    tracing::debug!("Health check endpoint called");
    "OK"
}

/// Mask sensitive information in database URL for logging
///
/// Replaces password with asterisks in postgresql:// URLs
fn mask_database_url(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let mut masked = url.to_string();
            masked.replace_range(colon_pos + 1..at_pos, "****");
            return masked;
        }
    }
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_database_url() {
        let url = "postgresql://user:password@localhost:5432/db";
        let masked = mask_database_url(url);
        assert_eq!(masked, "postgresql://user:****@localhost:5432/db");
    }

    #[test]
    fn test_mask_database_url_no_password() {
        let url = "postgresql://localhost:5432/db";
        let masked = mask_database_url(url);
        assert_eq!(masked, "postgresql://localhost:5432/db");
    }

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "OK");
    }
}
