//! Rust Basic API
//!
//! A production-ready REST API built with Axum framework.

mod config;
mod error;
mod models;
mod repository;
mod routes;

use crate::config::Config;
use axum::{extract::State, routing::get, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub pool: PgPool,
}

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

    // Create database pool
    let pool = repository::create_pool(&config.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create database pool: {e}"))?;

    tracing::info!("Database connection pool created");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run database migrations: {e}"))?;

    tracing::info!("Database migrations completed successfully");

    // Create application state
    let state = AppState { pool };

    // Build application router with state
    let app = Router::new()
        .route("/health", get(health_check))
        .merge(routes::build_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

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
/// Also verifies database connectivity.
async fn health_check(State(state): State<AppState>) -> &'static str {
    // Verify database connection is alive
    if sqlx::query("SELECT 1").execute(&state.pool).await.is_ok() {
        "OK"
    } else {
        "Database connection failed"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_with_db() {
        // Setup test database
        dotenv::from_filename(".env.test").ok();

        // Skip test if DATABASE_URL is not configured
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            if !database_url.is_empty() {
                // Create pool for testing
                if let Ok(pool) = repository::create_pool(&database_url).await {
                    let state = AppState { pool };
                    let response = health_check(axum::extract::State(state)).await;
                    assert_eq!(response, "OK");
                }
            }
        }
        // Note: Test skipped if DATABASE_URL is not configured or database is not available
    }
}
