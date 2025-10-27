mod config;
mod error;
mod models;
mod repository;
mod routes;

use axum::{routing::get, Json, Router};
use config::Config;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_basic_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config =
        Config::from_env().map_err(|e| anyhow::anyhow!("Failed to load configuration: {e}"))?;

    tracing::info!(
        "Starting server with configuration - Port: {}, Database: {}",
        config.server_port,
        mask_database_url(&config.database_url)
    );

    // Build application router with health check endpoint
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http());

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint that returns server status and current timestamp
///
/// # Returns
///
/// JSON response with status and timestamp
async fn health_check() -> Json<Value> {
    let now = chrono::Utc::now();
    Json(json!({
        "status": "OK",
        "timestamp": now.to_rfc3339(),
    }))
}

/// Mask sensitive parts of database URL for logging
///
/// # Arguments
///
/// * `url` - The database URL to mask
///
/// # Returns
///
/// A string with password and sensitive information masked
fn mask_database_url(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let mut masked = url.to_string();
            masked.replace_range((colon_pos + 1)..at_pos, "****");
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
        assert!(masked.contains("****"));
        assert!(!masked.contains("password"));
    }

    #[test]
    fn test_mask_database_url_no_password() {
        let url = "postgresql://localhost:5432/db";
        let masked = mask_database_url(url);
        assert_eq!(masked, url);
    }
}
