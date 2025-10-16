mod config;
mod error;
mod models;
mod repository;
mod routes;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use config::Config;
use error::AppError;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().map_err(AppError::from)?;
    tracing::info!("configuration loaded successfully");

    // Configure database pool
    let pool = PgPoolOptions::new()
        .max_connections(config.database_pool.max_connections)
        .acquire_timeout(config.database_pool.acquire_timeout)
        .idle_timeout(Some(config.database_pool.idle_timeout))
        .connect(&config.database_url)
        .await?;

    tracing::info!("database pool established");

    let state = AppState { pool };

    // Build application router
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(state);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!(%addr, "listening on");

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("server shutdown complete");

    Ok(())
}

async fn health_check(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    // Verify database connectivity by acquiring a connection
    state.pool.acquire().await.map_err(|error| {
        tracing::error!(error = %error, "database connectivity check failed");
        AppError::Database(error)
    })?;

    Ok((StatusCode::OK, "OK"))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install terminate handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {}
        () = terminate => {}
    }
}
