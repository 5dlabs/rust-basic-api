mod config;
mod error;
mod models;
mod repository;
mod routes;

use std::net::SocketAddr;

use anyhow::Context;
use config::Config;
use models::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = Config::from_env().context("failed to load configuration")?;

    let db_pool = repository::create_pool(&config.database_url, config.database_max_connections)
        .context("failed to initialize database connection pool")?;

    let state = AppState { db_pool };
    let app = routes::router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!(%addr, "HTTP server listening");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

    Ok(())
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer());

    if subscriber.try_init().is_err() {
        tracing::debug!("Tracing subscriber already initialized");
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    tracing::info!("Shutdown signal received");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Once;
    use tokio::time::{sleep, Duration};

    static TRACING_INIT: Once = Once::new();

    fn init_test_tracing() {
        TRACING_INIT.call_once(|| {
            init_tracing();
        });
    }

    #[test]
    fn init_tracing_is_idempotent() {
        init_tracing();
        init_tracing();
    }

    #[test]
    fn init_tracing_handles_already_initialized() {
        init_test_tracing();
        // Should not panic even if already initialized
        init_tracing();
    }

    #[test]
    fn init_tracing_with_env_filter() {
        env::set_var("RUST_LOG", "debug");
        init_tracing();
        env::remove_var("RUST_LOG");
    }

    #[test]
    fn init_tracing_with_invalid_env_filter() {
        env::set_var("RUST_LOG", "");
        init_tracing();
        env::remove_var("RUST_LOG");
    }

    #[tokio::test]
    async fn shutdown_signal_receives_ctrl_c() {
        // Test that shutdown_signal completes when receiving a signal
        // This is hard to test directly, so we test the structure exists
        tokio::select! {
            () = shutdown_signal() => {
                // This branch will be taken if a signal is received
            }
            () = sleep(Duration::from_millis(10)) => {
                // Expected: timeout because no signal sent
            }
        }
    }

    #[tokio::test]
    async fn app_state_creation() {
        let pool = repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
            .expect("pool creation should succeed");

        let state = models::AppState { db_pool: pool };

        // Test that state can be cloned
        let _cloned_state = state.clone();

        // Test debug formatting
        let debug_str = format!("{state:?}");
        assert!(debug_str.contains("AppState"));
    }

    #[tokio::test]
    async fn repository_pool_management() {
        let pool = repository::create_pool("postgres://user:pass@localhost:5432/db", 10)
            .expect("pool creation should succeed");

        assert!(!pool.is_closed());
        assert_eq!(pool.num_idle(), 0); // No connections created yet in lazy pool

        // Test closing pool
        pool.close().await;
        assert!(pool.is_closed());
    }

    #[test]
    fn repository_invalid_database_url() {
        let result = repository::create_pool("invalid_url", 5);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn repository_zero_max_connections() {
        // Zero max connections gets normalized to 1 by our create_pool function
        let pool = repository::create_pool("postgres://user:pass@localhost:5432/db", 0)
            .expect("pool creation should succeed with normalized connections");
        assert!(!pool.is_closed());
    }

    // Test the main function path without running server
    #[test]
    fn config_loading_integration() {
        env::set_var("DATABASE_URL", "postgres://test:test@localhost:5432/testdb");
        env::set_var("SERVER_PORT", "8080");

        let config = config::Config::from_env().expect("config should load");
        assert_eq!(config.server_port, 8080);
        assert!(config.database_url.contains("testdb"));

        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn config_loading_failure() {
        env::remove_var("DATABASE_URL");
        let result = config::Config::from_env();
        assert!(result.is_err());
    }

    // Test router creation with different states
    #[tokio::test]
    async fn router_with_different_pool_configurations() {
        let configs = vec![
            ("postgres://user1:pass1@localhost:5432/db1", 1),
            ("postgres://user2:pass2@localhost:5432/db2", 10),
            ("postgres://user3:pass3@localhost:5432/db3", 100),
        ];

        for (url, max_conn) in configs {
            let pool =
                repository::create_pool(url, max_conn).expect("pool creation should succeed");
            let state = models::AppState { db_pool: pool };
            let _router = routes::router(state); // Should not panic
        }
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn shutdown_signal_unix_specific() {
        use tokio::time::{sleep, Duration};

        // Test that the shutdown signal handler exists and can be constructed
        // This tests the Unix-specific SIGTERM handler code path
        tokio::select! {
            () = shutdown_signal() => {
                // Signal received (unlikely in test)
            }
            () = sleep(Duration::from_millis(5)) => {
                // Expected: timeout because no signal sent
            }
        }
    }

    #[cfg(not(unix))]
    #[tokio::test]
    async fn shutdown_signal_non_unix() {
        use tokio::time::{sleep, Duration};

        // Test the non-Unix shutdown signal path
        tokio::select! {
            _ = shutdown_signal() => {
                // Signal received (unlikely in test)
            }
            _ = sleep(Duration::from_millis(5)) => {
                // Expected: timeout because no signal sent
            }
        }
    }
}
