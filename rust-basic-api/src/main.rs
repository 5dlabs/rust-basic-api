use crate::config::Config;
use axum::{routing::get, Router};
use std::net::SocketAddr;

mod config;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let config = Config::from_env()?;
    let app = Router::new().route("/health", get(health_check));
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app.into_make_service(),
    )
    .await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
