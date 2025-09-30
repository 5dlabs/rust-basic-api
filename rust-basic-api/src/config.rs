use anyhow::{anyhow, bail, Context, Result};
use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: IpAddr,
    pub server_port: u16,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").context("DATABASE_URL environment variable must be set")?;

        let default_host = IpAddr::from_str("0.0.0.0").expect("valid default host");
        let server_host = parse_env_var("SERVER_HOST", default_host)?;
        let server_port = parse_env_var("SERVER_PORT", 3000u16)?;

        let database = DatabaseConfig::from_env()?;

        Ok(Self {
            database_url,
            server_host,
            server_port,
            database,
        })
    }

    #[must_use]
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.server_host, self.server_port)
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
}

impl DatabaseConfig {
    fn from_env() -> Result<Self> {
        let max_connections = parse_env_var("DATABASE_MAX_CONNECTIONS", 5u32)?;
        let min_connections = parse_env_var("DATABASE_MIN_CONNECTIONS", 1u32)?;
        if min_connections == 0 {
            bail!("DATABASE_MIN_CONNECTIONS must be greater than zero");
        }
        if min_connections > max_connections {
            bail!(
                "DATABASE_MIN_CONNECTIONS ({min_connections}) cannot exceed DATABASE_MAX_CONNECTIONS ({max_connections})"
            );
        }

        let acquire_timeout_secs = parse_env_var("DATABASE_ACQUIRE_TIMEOUT_SECS", 10u64)?;

        Ok(Self {
            max_connections,
            min_connections,
            acquire_timeout: Duration::from_secs(acquire_timeout_secs),
        })
    }
}

fn parse_env_var<T>(key: &str, default: T) -> Result<T>
where
    T: FromStr + Copy,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    match env::var(key) {
        Ok(value) => value
            .parse::<T>()
            .with_context(|| format!("failed to parse environment variable `{key}`")),
        Err(env::VarError::NotPresent) => Ok(default),
        Err(env::VarError::NotUnicode(_)) => Err(anyhow!(
            "environment variable `{key}` contains invalid unicode"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::EnvGuard;
    use std::net::IpAddr;
    use std::time::Duration;

    #[test]
    fn config_uses_defaults_when_optional_variables_missing() {
        let mut guard = EnvGuard::new();
        guard.set(
            "DATABASE_URL",
            "postgres://postgres:postgres@localhost:5432/postgres",
        );
        guard.remove("SERVER_HOST");
        guard.remove("SERVER_PORT");
        guard.remove("DATABASE_MAX_CONNECTIONS");
        guard.remove("DATABASE_MIN_CONNECTIONS");
        guard.remove("DATABASE_ACQUIRE_TIMEOUT_SECS");

        let config = Config::from_env().expect("config should load with defaults");

        assert_eq!(
            config.server_host,
            IpAddr::from_str("0.0.0.0").expect("valid ipv4")
        );
        assert_eq!(config.server_port, 3000);
        assert_eq!(config.database.max_connections, 5);
        assert_eq!(config.database.min_connections, 1);
        assert_eq!(config.database.acquire_timeout, Duration::from_secs(10));
    }

    #[test]
    fn config_uses_custom_values_when_provided() {
        let mut guard = EnvGuard::new();
        guard.set(
            "DATABASE_URL",
            "postgres://app:secret@db.internal:5432/service",
        );
        guard.set("SERVER_HOST", "127.0.0.1");
        guard.set("SERVER_PORT", "8080");
        guard.set("DATABASE_MAX_CONNECTIONS", "15");
        guard.set("DATABASE_MIN_CONNECTIONS", "5");
        guard.set("DATABASE_ACQUIRE_TIMEOUT_SECS", "30");

        let config = Config::from_env().expect("config should load with custom values");

        assert_eq!(
            config.server_host,
            IpAddr::from_str("127.0.0.1").expect("valid ipv4")
        );
        assert_eq!(config.server_port, 8080);
        assert_eq!(config.database.max_connections, 15);
        assert_eq!(config.database.min_connections, 5);
        assert_eq!(config.database.acquire_timeout, Duration::from_secs(30));
        assert_eq!(
            config.database_url,
            "postgres://app:secret@db.internal:5432/service"
        );
    }

    #[test]
    fn config_rejects_invalid_pool_bounds() {
        let mut guard = EnvGuard::new();
        guard.set(
            "DATABASE_URL",
            "postgres://postgres:postgres@localhost:5432/postgres",
        );
        guard.set("DATABASE_MAX_CONNECTIONS", "4");
        guard.set("DATABASE_MIN_CONNECTIONS", "5");

        let error = Config::from_env().unwrap_err();
        assert!(error
            .to_string()
            .contains("DATABASE_MIN_CONNECTIONS (5) cannot exceed"));
    }
}
