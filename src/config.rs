use std::{env, io::ErrorKind};

use dotenv::dotenv;
use tracing::warn;

use crate::error::{ConfigError, ConfigResult};

const DEFAULT_SERVER_PORT: u16 = 3000;
const DEFAULT_DATABASE_MAX_CONNECTIONS: u32 = 5;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub database_max_connections: u32,
}

impl Config {
    pub fn from_env() -> ConfigResult<Self> {
        match dotenv() {
            Ok(path) => {
                tracing::debug!(path = %path.display(), "Loaded environment variables from .env file");
            }
            Err(dotenv::Error::Io(ref io_err)) if io_err.kind() == ErrorKind::NotFound => {}
            Err(err) => warn!(%err, "Failed to load .env file"),
        }

        let database_url = match env::var("DATABASE_URL") {
            Ok(value) if value.trim().is_empty() => return Err(ConfigError::EmptyDatabaseUrl),
            Ok(value) => value,
            Err(env::VarError::NotPresent) => return Err(ConfigError::MissingDatabaseUrl),
            Err(err) => {
                return Err(ConfigError::EnvVar {
                    variable: "DATABASE_URL",
                    source: err,
                })
            }
        };

        let server_port = match env::var("SERVER_PORT") {
            Ok(value) => value
                .parse::<u16>()
                .map_err(|source| ConfigError::InvalidServerPort { value, source })?,
            Err(env::VarError::NotPresent) => DEFAULT_SERVER_PORT,
            Err(err) => {
                return Err(ConfigError::EnvVar {
                    variable: "SERVER_PORT",
                    source: err,
                })
            }
        };

        let database_max_connections = match env::var("DATABASE_MAX_CONNECTIONS") {
            Ok(value) => {
                let parsed = value.parse::<u32>().map_err(|source| {
                    ConfigError::InvalidDatabaseMaxConnections {
                        value: value.clone(),
                        source,
                    }
                })?;
                if parsed == 0 {
                    return Err(ConfigError::DatabaseMaxConnectionsZero { value: parsed });
                }
                parsed
            }
            Err(env::VarError::NotPresent) => DEFAULT_DATABASE_MAX_CONNECTIONS,
            Err(err) => {
                return Err(ConfigError::EnvVar {
                    variable: "DATABASE_MAX_CONNECTIONS",
                    source: err,
                })
            }
        };

        Ok(Self {
            database_url,
            server_port,
            database_max_connections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> &'static Mutex<()> {
        ENV_MUTEX.get_or_init(Mutex::default)
    }

    #[test]
    fn loads_required_configuration_values() {
        let _guard = env_lock().lock().unwrap();

        env::set_var(
            "DATABASE_URL",
            "postgres://tester:secret@localhost:5432/test_db",
        );
        env::set_var("SERVER_PORT", "8080");
        env::set_var("DATABASE_MAX_CONNECTIONS", "12");

        let config = Config::from_env().expect("configuration should load successfully");

        assert_eq!(
            config.database_url,
            "postgres://tester:secret@localhost:5432/test_db"
        );
        assert_eq!(config.server_port, 8080);
        assert_eq!(config.database_max_connections, 12);

        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
        env::remove_var("DATABASE_MAX_CONNECTIONS");
    }

    #[test]
    fn defaults_optional_values_and_errors_when_required_missing() {
        let _guard = env_lock().lock().unwrap();

        env::remove_var("SERVER_PORT");
        env::remove_var("DATABASE_MAX_CONNECTIONS");
        env::remove_var("DATABASE_URL");

        let err = Config::from_env().expect_err("missing DATABASE_URL should error");
        assert!(matches!(err, ConfigError::MissingDatabaseUrl));
    }

    #[test]
    fn rejects_zero_database_connections() {
        let _guard = env_lock().lock().unwrap();

        env::set_var(
            "DATABASE_URL",
            "postgres://tester:secret@localhost:5432/test_db",
        );
        env::set_var("DATABASE_MAX_CONNECTIONS", "0");

        let err = Config::from_env().expect_err("zero database connections should be rejected");
        assert!(matches!(
            err,
            ConfigError::DatabaseMaxConnectionsZero { .. }
        ));

        env::remove_var("DATABASE_URL");
        env::remove_var("DATABASE_MAX_CONNECTIONS");
    }
}
