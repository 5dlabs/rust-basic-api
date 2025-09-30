#![allow(dead_code)] // Utility helpers consumed by future integration tests.

use crate::config::DatabaseConfig;
use crate::repository::{self, DbPool};
use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::sync::{LazyLock, Mutex, MutexGuard};
use std::time::Duration;
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use testcontainers::{Container, GenericImage};
use tokio::time::sleep;

static ENV_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

pub struct EnvGuard {
    _lock: MutexGuard<'static, ()>,
    originals: HashMap<String, Option<OsString>>,
}

impl EnvGuard {
    pub fn new() -> Self {
        Self {
            _lock: ENV_MUTEX.lock().expect("environment mutex poisoned"),
            originals: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: impl AsRef<str>) {
        self.originals
            .entry(key.to_string())
            .or_insert_with(|| env::var_os(key));
        env::set_var(key, value.as_ref());
    }

    pub fn set_os(&mut self, key: &str, value: &OsStr) {
        self.originals
            .entry(key.to_string())
            .or_insert_with(|| env::var_os(key));
        env::set_var(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.originals
            .entry(key.to_string())
            .or_insert_with(|| env::var_os(key));
        env::remove_var(key);
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in self.originals.drain() {
            match value {
                Some(original) => env::set_var(&key, original),
                None => env::remove_var(&key),
            }
        }
    }
}

#[must_use]
pub fn default_database_config() -> DatabaseConfig {
    DatabaseConfig {
        max_connections: 5,
        min_connections: 1,
        acquire_timeout: Duration::from_secs(10),
    }
}

pub struct PostgresInstance<'a> {
    _container: Container<'a, GenericImage>,
    pub connection_string: String,
}

pub fn spawn_postgres(docker: &Cli) -> PostgresInstance<'_> {
    let image = GenericImage::new("postgres", "15-alpine")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_DB", "postgres")
        .with_exposed_port(5432)
        .with_wait_for(WaitFor::message_on_stdout(
            "database system is ready to accept connections",
        ));

    let container = docker.run(image);
    let host_port = container.get_host_port_ipv4(5432);
    let connection_string = format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres");

    PostgresInstance {
        _container: container,
        connection_string,
    }
}

pub async fn wait_for_pool(pool: &DbPool) -> Result<(), sqlx::Error> {
    const MAX_ATTEMPTS: usize = 10;
    const PAUSE: Duration = Duration::from_millis(100);

    for attempt in 0..MAX_ATTEMPTS {
        match repository::ensure_connection(pool).await {
            Ok(()) => return Ok(()),
            Err(error) if attempt + 1 == MAX_ATTEMPTS => return Err(error),
            Err(_) => sleep(PAUSE).await,
        }
    }

    Ok(())
}
