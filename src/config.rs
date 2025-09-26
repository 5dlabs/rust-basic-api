use dotenv::dotenv;
use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use thiserror::Error;

/// Application configuration loaded from the environment.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub server_host: IpAddr,
    pub server_port: u16,
    pub rust_log: String,
}

impl Config {
    /// Load configuration by reading environment variables.
    ///
    /// Falls back to sensible defaults when optional variables are missing.
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = read_required("DATABASE_URL")?;
        let database_max_connections = read_optional("DATABASE_MAX_CONNECTIONS", 5u32)?;
        let server_host = read_optional("SERVER_HOST", IpAddr::from([0, 0, 0, 0]))?;
        let server_port = read_optional("SERVER_PORT", 3000u16)?;
        let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(Self {
            database_url,
            database_max_connections,
            server_host,
            server_port,
            rust_log,
        })
    }

    /// Convenience helper to build the socket address the server should bind to.
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.server_host, self.server_port)
    }
}

fn read_required(var: &str) -> Result<String, ConfigError> {
    env::var(var).map_err(|source| ConfigError::MissingEnv {
        var: var.to_string(),
        source,
    })
}

fn read_optional<T>(var: &str, default: T) -> Result<T, ConfigError>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    match env::var(var) {
        Ok(value) => value.parse().map_err(|source| ConfigError::InvalidEnv {
            var: var.to_string(),
            source: Box::new(source),
        }),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(source) => Err(ConfigError::MissingEnv {
            var: var.to_string(),
            source,
        }),
    }
}

/// Errors that can occur while loading configuration from the environment.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable {var}")]
    MissingEnv {
        var: String,
        #[source]
        source: env::VarError,
    },
    #[error("invalid value for environment variable {var}")]
    InvalidEnv {
        var: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        net::{IpAddr, SocketAddr},
        sync::{Mutex, OnceLock},
    };

    fn env_mutex() -> &'static Mutex<()> {
        static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_MUTEX.get_or_init(|| Mutex::new(()))
    }

    fn with_env<R>(vars: &[(&str, Option<&str>)], test: impl FnOnce() -> R) -> R {
        let guard = env_mutex().lock().expect("env mutex poisoned");

        let snapshot: Vec<(String, Option<String>)> = vars
            .iter()
            .map(|(key, _)| ((*key).to_string(), env::var(key).ok()))
            .collect();

        for (key, value) in vars {
            match value {
                Some(val) => env::set_var(key, val),
                None => env::remove_var(key),
            }
        }

        let result = test();

        for (key, value) in snapshot {
            match value {
                Some(val) => env::set_var(&key, val),
                None => env::remove_var(&key),
            }
        }

        drop(guard);
        result
    }

    #[test]
    fn from_env_reads_all_configured_values() {
        with_env(
            &[
                (
                    "DATABASE_URL",
                    Some("postgresql://postgres:postgres@localhost:5432/test"),
                ),
                ("DATABASE_MAX_CONNECTIONS", Some("12")),
                ("SERVER_HOST", Some("127.0.0.1")),
                ("SERVER_PORT", Some("4000")),
                ("RUST_LOG", Some("debug")),
            ],
            || {
                let config = Config::from_env().expect("config should load");
                assert_eq!(
                    config.database_url,
                    "postgresql://postgres:postgres@localhost:5432/test"
                );
                assert_eq!(config.database_max_connections, 12);
                assert_eq!(config.server_host, IpAddr::from([127, 0, 0, 1]));
                assert_eq!(config.server_port, 4000);
                assert_eq!(config.rust_log, "debug");
                assert_eq!(
                    config.socket_addr(),
                    SocketAddr::from(([127, 0, 0, 1], 4000))
                );
            },
        );
    }

    #[test]
    fn from_env_uses_defaults_for_optional_values() {
        with_env(
            &[
                (
                    "DATABASE_URL",
                    Some("postgresql://postgres:postgres@localhost:5432/defaults"),
                ),
                ("DATABASE_MAX_CONNECTIONS", None),
                ("SERVER_HOST", None),
                ("SERVER_PORT", None),
                ("RUST_LOG", None),
            ],
            || {
                let config = Config::from_env().expect("config should load with defaults");
                assert_eq!(config.database_max_connections, 5);
                assert_eq!(config.server_host, IpAddr::from([0, 0, 0, 0]));
                assert_eq!(config.server_port, 3000);
                assert_eq!(config.rust_log, "info");
            },
        );
    }

    #[test]
    fn from_env_requires_database_url() {
        with_env(
            &[
                ("DATABASE_URL", None),
                ("DATABASE_MAX_CONNECTIONS", None),
                ("SERVER_HOST", None),
                ("SERVER_PORT", None),
                ("RUST_LOG", None),
            ],
            || {
                let error = Config::from_env().expect_err("missing DATABASE_URL should error");
                assert!(matches!(
                    error,
                    ConfigError::MissingEnv { ref var, .. } if var == "DATABASE_URL"
                ));
            },
        );
    }

    #[test]
    fn from_env_rejects_invalid_numbers() {
        with_env(
            &[
                (
                    "DATABASE_URL",
                    Some("postgresql://postgres:postgres@localhost:5432/invalid"),
                ),
                ("SERVER_PORT", Some("not-a-number")),
                ("DATABASE_MAX_CONNECTIONS", None),
                ("SERVER_HOST", None),
                ("RUST_LOG", None),
            ],
            || {
                let error = Config::from_env().expect_err("invalid server port should error");
                assert!(matches!(
                    error,
                    ConfigError::InvalidEnv { ref var, .. } if var == "SERVER_PORT"
                ));
            },
        );
    }
}
