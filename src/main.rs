mod config;
mod error;
use axum::{Router, Server, routing::get};
use config::Config;
use error::AppError;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env().map_err(|e| AppError::Config(e.to_string()))?;

    info!("Starting server on port {}", config.server_port);

    let app: Router = Router::new().route("/health", get(health_check));

    let addr: SocketAddr = format!("0.0.0.0:{}", config.server_port).parse().unwrap();

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
