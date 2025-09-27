mod config;
mod error;
mod models;
mod repository;
mod routes;

use std::{future::Future, net::SocketAddr, sync::OnceLock};

use anyhow::Context;
use axum::Router;
use config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app_main().await
}

async fn app_main() -> anyhow::Result<()> {
    init_tracing();

    let config = Config::from_env().context("failed to load configuration")?;

    run(config).await
}

async fn run(config: Config) -> anyhow::Result<()> {
    run_with_shutdown(config, shutdown_signal()).await
}

async fn run_with_shutdown<F>(config: Config, shutdown: F) -> anyhow::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    tracing::debug!(
        has_database_url = !config.database_url.is_empty(),
        "configuration loaded"
    );

    let (addr, app) = build_app(&config);
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await
        .context("server error")
}

fn build_app(config: &Config) -> (SocketAddr, Router) {
    let app = routes::create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    (addr, app)
}

fn init_tracing() {
    static INITIALIZED: OnceLock<()> = OnceLock::new();

    INITIALIZED.get_or_init(|| {
        tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
            .with(tracing_subscriber::fmt::layer())
            .init();
    });
}

#[cfg(not(test))]
async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::warn!(?error, "failed to listen for shutdown signal");
    }
}

#[cfg(test)]
fn shutdown_signal() -> impl Future<Output = ()> + Send + 'static {
    std::future::ready(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        sync::{Mutex, OnceLock},
    };

    fn env_lock() -> &'static Mutex<()> {
        static ENV_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_GUARD.get_or_init(|| Mutex::new(()))
    }

    fn clear_runtime_env() {
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn build_app_uses_configured_port() {
        let config = Config {
            database_url: "postgres://example".into(),
            server_port: 4242,
        };

        let (addr, _) = build_app(&config);

        assert_eq!(addr, SocketAddr::from(([0, 0, 0, 0], 4242)));
    }

    #[test]
    fn init_tracing_runs_only_once() {
        init_tracing();
        init_tracing();
    }

    #[tokio::test]
    async fn run_with_shutdown_completes() {
        init_tracing();
        let config = Config {
            database_url: "postgres://example".into(),
            server_port: 0,
        };

        run_with_shutdown(config, std::future::ready(()))
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn app_main_runs_with_env_configuration() {
        let _guard = env_lock().lock().unwrap();
        clear_runtime_env();
        env::set_var("DATABASE_URL", "postgres://example");
        env::set_var("SERVER_PORT", "0");

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(app_main())
                .expect("app main runs");
        });

        clear_runtime_env();
    }

    #[test]
    fn main_runs_with_env_configuration() {
        let _guard = env_lock().lock().unwrap();
        clear_runtime_env();
        env::set_var("DATABASE_URL", "postgres://example");
        env::set_var("SERVER_PORT", "0");

        super::main().expect("main runs");

        clear_runtime_env();
    }
}
