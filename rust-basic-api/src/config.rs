use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

use dotenv::dotenv;

use crate::error::{ConfigError, ConfigResult};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub server_host: IpAddr,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
}

impl Config {
    pub fn from_env() -> ConfigResult<Self> {
        dotenv().ok();

        let database_url = get_required_env("DATABASE_URL")?;
        let server_port = parse_env_or_default("SERVER_PORT", 3000u16)?;
        let server_host = parse_ip_or_default("SERVER_HOST", IpAddr::from(Ipv4Addr::UNSPECIFIED))?;

        let max_connections = parse_env_or_default("DATABASE_POOL_MAX_CONNECTIONS", 5u32)?;
        let min_connections = parse_env_or_default("DATABASE_POOL_MIN_CONNECTIONS", 1u32)?;
        let acquire_timeout_seconds =
            parse_env_or_default("DATABASE_POOL_ACQUIRE_TIMEOUT_SECONDS", 10u64)?;

        if min_connections > max_connections {
            return Err(ConfigError::invalid(
                "DATABASE_POOL_MIN_CONNECTIONS",
                min_connections.to_string(),
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "min connections ({min_connections}) cannot exceed max connections ({max_connections})"
                    ),
                ),
            ));
        }

        let database = DatabaseConfig {
            max_connections,
            min_connections,
            acquire_timeout: Duration::from_secs(acquire_timeout_seconds),
        };

        Ok(Self {
            database_url,
            server_port,
            server_host,
            database,
        })
    }

    #[must_use]
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.server_host, self.server_port)
    }
}

fn get_required_env(key: &'static str) -> ConfigResult<String> {
    match env::var(key) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Err(ConfigError::Missing(key)),
        Err(env::VarError::NotUnicode(_)) => Err(ConfigError::non_unicode(key)),
    }
}

fn parse_env_or_default<T>(key: &'static str, default: T) -> ConfigResult<T>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    match env::var(key) {
        Ok(value) => {
            let trimmed = value.trim();
            trimmed
                .parse::<T>()
                .map_err(|source| ConfigError::invalid(key, value, source))
        }
        Err(env::VarError::NotPresent) => Ok(default),
        Err(env::VarError::NotUnicode(_)) => Err(ConfigError::non_unicode(key)),
    }
}

fn parse_ip_or_default(key: &'static str, default: IpAddr) -> ConfigResult<IpAddr> {
    parse_env_or_default::<IpAddr>(key, default)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use std::ffi::OsString;

    fn with_env<F>(vars: &[(&str, &str)], test: F)
    where
        F: FnOnce(),
    {
        let backups: Vec<(&str, Option<OsString>)> = vars
            .iter()
            .map(|(key, value)| {
                let previous = env::var_os(key);
                env::set_var(key, value);
                (*key, previous)
            })
            .collect();

        test();

        for (key, previous) in backups {
            match previous {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
        }
    }

    #[test]
    fn loads_minimal_configuration() {
        with_env(
            &[
                ("DATABASE_URL", "postgres://example"),
                ("SERVER_PORT", "4000"),
            ],
            || {
                let config = Config::from_env().expect("configuration should load");

                assert_eq!(config.database_url, "postgres://example");
                assert_eq!(config.server_port, 4000);
                assert_eq!(config.server_host, IpAddr::from(Ipv4Addr::UNSPECIFIED));
            },
        );
    }
}
