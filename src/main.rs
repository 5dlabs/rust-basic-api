use rust_basic_api::{config::Config, create_app, init_tracing};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber for structured logging
    init_tracing().ok();

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
    let app = create_app();

    // Create socket address
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), config.server_port);
    tracing::info!("Listening on {}", addr);

    // Start the HTTP server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
