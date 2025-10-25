mod config;
pub mod error;
mod models;
mod repository;
mod routes;

use axum::Router;
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for structured logging
    init_tracing();

    // Load configuration and run server
    run_application().await
}

/// Run the application with configuration loaded from environment
///
/// # Errors
///
/// Returns an error if configuration loading fails or server startup fails
pub async fn run_application() -> anyhow::Result<()> {
    // Load configuration from environment
    let config = Config::from_env()?;
    tracing::info!(
        server_port = config.server_port,
        "Loaded server configuration"
    );

    // Connect to database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Successfully connected to database");

    // Build application router with routes
    let app = create_app(pool);

    // Create socket address and start server
    start_server(app, config.server_port).await
}

/// Start the HTTP server on the specified port
///
/// # Errors
///
/// Returns an error if the server fails to bind or start
pub async fn start_server(app: Router, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Initialize tracing subscriber for structured logging
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_basic_api=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Create the application router with all routes
///
/// # Returns
///
/// A configured `Router` with all application routes and database pool
pub fn create_app(pool: sqlx::PgPool) -> Router {
    routes::register_routes(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tracing_function_exists() {
        // Verify the function signature is correct
        let _: fn() = init_tracing;
    }

    #[test]
    fn test_socket_addr_creation() {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        // Test socket address creation with different ports
        let port = 3000;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
        assert_eq!(addr.port(), 3000);
        assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::UNSPECIFIED));

        let port = 8080;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
        assert_eq!(addr.port(), 8080);
    }

    // Note: Integration tests with database connection would go in tests/ directory
    // These are unit tests for non-database logic only
}
