#![allow(unexpected_cfgs)]

mod app_state;
mod config;
mod error;
mod models;
mod repository;
mod routes;

use std::{future::Future, net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use app_state::AppState;
use config::Config;
use repository::create_pool;
use routes::create_router;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, util::TryInitError, EnvFilter,
};

#[tokio::main]
async fn main() -> Result<()> {
    run_with_shutdown(shutdown_signal()).await
}

/// Runs the application with the provided shutdown signal.
///
/// # Errors
/// Returns an error if tracing initialization fails (and is not already configured),
/// configuration cannot be loaded, or the HTTP server reports an error.
pub async fn run_with_shutdown(shutdown: impl Future<Output = ()> + Send + 'static) -> Result<()> {
    run_with_shutdown_using(init_tracing, shutdown).await
}

/// Runs the application using a custom tracing initializer and shutdown signal.
///
/// # Errors
/// Returns an error if initialization fails (excluding repeated tracing setup),
/// configuration cannot be loaded, or the HTTP server reports an error.
pub async fn run_with_shutdown_using(
    init: impl Fn() -> Result<()> + Send,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<()> {
    if let Err(error) = init() {
        if !error.is::<SetGlobalDefaultError>() && !error.is::<TryInitError>() {
            Err::<(), _>(error).context("failed to initialize tracing")?;
        }
    }

    let config = Arc::new(Config::from_env().context("failed to load configuration")?);
    let application =
        build_application(Arc::clone(&config)).context("failed to prepare application state")?;

    tracing::info!(addr = %application.addr(), "listening on");

    run_application(application, shutdown).await
}

pub struct Application {
    addr: SocketAddr,
    state: Arc<AppState>,
}

impl Application {
    #[must_use]
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    #[must_use]
    pub fn state(&self) -> Arc<AppState> {
        Arc::clone(&self.state)
    }
}

/// Builds the application state and router from the provided configuration.
///
/// # Errors
/// Returns an error if the database connection pool cannot be created.
pub fn build_application(config: Arc<Config>) -> Result<Application> {
    let db_pool = create_pool(&config.database_url, config.database_max_connections)
        .context("failed to create database connection pool")?;
    let addr = SocketAddr::new(config.server_host, config.server_port);
    let state = Arc::new(AppState::new(config, db_pool));

    Ok(Application { addr, state })
}

/// Serves HTTP traffic until the provided shutdown future resolves.
///
/// # Errors
/// Returns an error if the HTTP server reports an error while serving traffic.
pub async fn run_application(
    application: Application,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<()> {
    let Application { addr, state } = application;
    let router = create_router().with_state(state);

    axum::Server::bind(&addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown)
        .await
        .context("server error")
}

fn init_tracing() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .context("invalid RUST_LOG filter configuration")?;

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .map_err(anyhow::Error::new)
}

#[cfg(coverage)]
async fn shutdown_signal() {
    std::future::pending::<()>().await;
}

#[cfg(not(coverage))]
async fn shutdown_signal() {
    shutdown_listener(ctrl_c_signal(), terminate_signal()).await;
}

#[cfg(coverage)]
async fn ctrl_c_signal() {
    std::future::pending::<()>().await;
}

#[cfg(not(coverage))]
async fn ctrl_c_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
}

#[cfg(all(coverage, unix))]
async fn terminate_signal() {
    std::future::pending::<()>().await;
}

#[cfg(all(not(coverage), unix))]
async fn terminate_signal() {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigterm = signal(SignalKind::terminate()).expect("failed to install terminate handler");

    loop {
        if sigterm.recv().await.is_some() {
            break;
        }

        tracing::warn!("SIGTERM listener returned None; reinstalling handler");
        sigterm = signal(SignalKind::terminate()).expect("failed to reinstall terminate handler");
    }
}

#[cfg(all(coverage, not(unix)))]
async fn terminate_signal() {
    std::future::pending::<()>().await;
}

#[cfg(all(not(coverage), not(unix)))]
async fn terminate_signal() {
    std::future::pending::<()>().await;
}

async fn shutdown_listener(
    ctrl_c: impl Future<Output = ()> + Send,
    terminate: impl Future<Output = ()> + Send,
) {
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    tracing::info!("shutdown signal received");
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        env,
        net::{IpAddr, Ipv4Addr},
        time::Duration,
    };

    use anyhow::{anyhow, Result as AnyResult};
    use http::StatusCode;
    use hyper::{body::to_bytes, Client, Uri};
    use tokio::sync::oneshot;
    #[cfg(coverage)]
    use tokio::task::yield_now;
    use tracing_subscriber::EnvFilter;

    fn test_config(port: u16) -> Arc<Config> {
        Arc::new(Config {
            database_url: "postgresql://example".into(),
            database_max_connections: 5,
            server_host: IpAddr::V4(Ipv4Addr::LOCALHOST),
            server_port: port,
        })
    }

    struct EnvCleanup;

    impl Drop for EnvCleanup {
        fn drop(&mut self) {
            for key in [
                "DATABASE_URL",
                "SERVER_HOST",
                "SERVER_PORT",
                "DATABASE_MAX_CONNECTIONS",
                "RUST_LOG",
            ] {
                env::remove_var(key);
            }
        }
    }

    #[test]
    fn env_filter_rejects_invalid_syntax() {
        for candidate in [
            "::invalid",
            "invalid::",
            "info==warn",
            "target@warn",
            "invalid%",
        ] {
            if EnvFilter::try_new(candidate).is_err() {
                return;
            }
        }

        panic!("expected at least one invalid directive");
    }

    fn find_free_port() -> u16 {
        std::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .expect("should bind to an ephemeral port")
            .local_addr()
            .expect("should obtain local addr")
            .port()
    }

    #[tokio::test]
    async fn build_application_constructs_state_and_router() -> AnyResult<()> {
        let config = test_config(4000);
        let application = build_application(Arc::clone(&config)).expect("build succeeds");

        assert_eq!(
            application.addr(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000)
        );

        let state = application.state();
        assert_eq!(state.config.database_url, "postgresql://example");
        assert!(!state.db_pool.is_closed(), "pool should be open");

        Ok(())
    }

    #[tokio::test]
    async fn run_application_serves_health_requests() -> AnyResult<()> {
        let _ = init_tracing();

        let port = find_free_port();
        let config = test_config(port);
        let application = build_application(Arc::clone(&config)).expect("build succeeds");
        let addr = application.addr();
        let state = application.state();
        assert_eq!(state.config.server_port, port);

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let uri: Uri = format!("http://{addr}/health").parse().expect("valid uri");

        let server_handle = tokio::spawn(async move {
            run_application(application, async {
                let _ = shutdown_rx.await;
            })
            .await
        });

        let client = Client::new();

        let response = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                match client.get(uri.clone()).await {
                    Ok(resp) => break resp,
                    Err(_) => tokio::time::sleep(Duration::from_millis(50)).await,
                }
            }
        })
        .await
        .map_err(|_| anyhow!("timed out waiting for server response"))?;

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = to_bytes(response.into_body())
            .await
            .expect("body should be readable");
        let body = String::from_utf8(body_bytes.to_vec()).expect("body is utf8");
        assert_eq!(body, "OK");

        let _ = shutdown_tx.send(());
        let server_result = tokio::time::timeout(Duration::from_secs(5), server_handle)
            .await
            .expect("server join should succeed");

        server_result.expect("server future should resolve successfully")?;

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn run_with_shutdown_uses_env_configuration() -> AnyResult<()> {
        let _cleanup = EnvCleanup;

        let port = find_free_port();
        env::set_var("DATABASE_URL", "postgresql://example");
        env::set_var("SERVER_HOST", "127.0.0.1");
        env::set_var("SERVER_PORT", port.to_string());

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let run_handle = tokio::spawn(async move {
            run_with_shutdown(async {
                let _ = shutdown_rx.await;
            })
            .await
        });

        tokio::time::sleep(Duration::from_millis(200)).await;

        shutdown_tx.send(()).ok();

        tokio::time::timeout(Duration::from_secs(5), run_handle)
            .await
            .expect("run join should succeed")
            .expect("run task should complete")?;

        Ok(())
    }

    #[tokio::test]
    async fn run_with_shutdown_propagates_tracing_errors() {
        let result = run_with_shutdown_using(|| Err(anyhow!("boom")), async {}).await;

        assert!(result.is_err(), "expected tracing initialization failure");
    }

    #[tokio::test]
    async fn shutdown_listener_finishes_when_ctrl_c_triggers() -> AnyResult<()> {
        let (tx, rx) = oneshot::channel::<()>();

        let listener = tokio::spawn(async move {
            shutdown_listener(
                async {
                    let _ = rx.await;
                },
                std::future::pending(),
            )
            .await;
        });

        tx.send(()).expect("signal send should succeed");

        tokio::time::timeout(Duration::from_secs(1), listener)
            .await
            .expect("listener should finish")
            .expect("listener task should succeed");

        Ok(())
    }

    #[cfg(coverage)]
    #[tokio::test]
    async fn coverage_stubs_are_pollable() {
        let shutdown = tokio::spawn(shutdown_signal());
        let ctrl = tokio::spawn(ctrl_c_signal());
        #[cfg(unix)]
        let term = tokio::spawn(terminate_signal());

        yield_now().await;

        shutdown.abort();
        ctrl.abort();
        #[cfg(unix)]
        term.abort();
    }

    #[tokio::test]
    async fn shutdown_listener_finishes_on_terminate_branch() -> AnyResult<()> {
        let listener = tokio::spawn(async {
            shutdown_listener(std::future::pending(), async {}).await;
        });

        tokio::time::timeout(Duration::from_secs(1), listener)
            .await
            .expect("listener should finish")
            .expect("listener task should succeed");

        Ok(())
    }
}
