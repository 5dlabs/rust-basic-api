use axum::{Router, routing::get, Server};
use std::net::SocketAddr;
use tracing_subscriber;
use crate::config::Config;

mod config;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let config = Config::from_env()?;
    let app = Router::new().route("/health", get(health_check));
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
