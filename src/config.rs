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
