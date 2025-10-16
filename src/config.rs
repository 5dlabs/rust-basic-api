use dotenv::dotenv;
use std::{env, num::ParseIntError, time::Duration};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub database_pool: DatabasePoolConfig,
}

#[derive(Debug, Clone)]
pub struct DatabasePoolConfig {
    pub max_connections: u32,
    pub idle_timeout: Duration,
    pub acquire_timeout: Duration,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing environment variable: {key}")]
    MissingEnv {
        key: &'static str,
        #[source]
        source: env::VarError,
    },
    #[error("invalid server port value `{value}`: {source}")]
    InvalidPort {
        value: String,
        #[source]
        source: ParseIntError,
    },
    #[error("invalid numeric value for `{key}`: `{value}` ({source})")]
    InvalidNumber {
        key: &'static str,
        value: String,
        #[source]
        source: ParseIntError,
    },
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing or invalid
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").map_err(|source| ConfigError::MissingEnv {
            key: "DATABASE_URL",
            source,
        })?;

        let server_port = parse_port("SERVER_PORT", 3000)?;
        let database_pool = DatabasePoolConfig::from_env()?;

        Ok(Self {
            database_url,
            server_port,
            database_pool,
        })
    }
}

impl DatabasePoolConfig {
    fn from_env() -> Result<Self, ConfigError> {
        let max_connections = parse_u32("DATABASE_POOL_MAX_CONNECTIONS", 10)?;
        let idle_timeout_secs = parse_u64("DATABASE_POOL_IDLE_TIMEOUT_SECS", 30)?;
        let acquire_timeout_secs = parse_u64("DATABASE_POOL_ACQUIRE_TIMEOUT_SECS", 10)?;

        Ok(Self {
            max_connections,
            idle_timeout: Duration::from_secs(idle_timeout_secs),
            acquire_timeout: Duration::from_secs(acquire_timeout_secs),
        })
    }
}

fn parse_port(key: &'static str, default: u16) -> Result<u16, ConfigError> {
    match env::var(key) {
        Ok(value) => value
            .parse::<u16>()
            .map_err(|source| ConfigError::InvalidPort { value, source }),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(source) => Err(ConfigError::MissingEnv { key, source }),
    }
}

fn parse_u32(key: &'static str, default: u32) -> Result<u32, ConfigError> {
    match env::var(key) {
        Ok(value) => value
            .parse::<u32>()
            .map_err(|source| ConfigError::InvalidNumber { key, value, source }),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(source) => Err(ConfigError::MissingEnv { key, source }),
    }
}

fn parse_u64(key: &'static str, default: u64) -> Result<u64, ConfigError> {
    match env::var(key) {
        Ok(value) => value
            .parse::<u64>()
            .map_err(|source| ConfigError::InvalidNumber { key, value, source }),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(source) => Err(ConfigError::MissingEnv { key, source }),
    }
}
