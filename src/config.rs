use dotenv::dotenv;
use std::env;
use thiserror::Error;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// `PostgreSQL` database connection URL
    pub database_url: String,
    /// HTTP server port
    pub server_port: u16,
}

/// Errors that can occur while loading configuration
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConfigError {
    /// The `DATABASE_URL` environment variable was missing or invalid
    #[error("DATABASE_URL environment variable must be set: {0}")]
    DatabaseUrl(#[source] env::VarError),
    /// The `DATABASE_URL` environment variable was provided but empty
    #[error("DATABASE_URL environment variable cannot be empty")]
    EmptyDatabaseUrl,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if `DATABASE_URL` is not set or empty
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").map_err(ConfigError::DatabaseUrl)?;
        if database_url.trim().is_empty() {
            return Err(ConfigError::EmptyDatabaseUrl);
        }

        let server_port = match env::var("SERVER_PORT") {
            Ok(value) => value.parse().unwrap_or_else(|error| {
                tracing::warn!(
                    %value,
                    %error,
                    "Invalid SERVER_PORT provided; defaulting to 3000"
                );
                3000
            }),
            Err(_) => 3000,
        };

        Ok(Config {
            database_url,
            server_port,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn restore_var(key: &str, original: Option<String>) {
        if let Some(value) = original {
            env::set_var(key, value);
        } else {
            env::remove_var(key);
        }
    }

    #[test]
    fn test_config_from_env() {
        let _guard = env_guard();

        let original_database = env::var("DATABASE_URL").ok();
        let original_port = env::var("SERVER_PORT").ok();

        env::set_var("DATABASE_URL", "postgresql://localhost/test_db");
        env::set_var("SERVER_PORT", "8080");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.database_url, "postgresql://localhost/test_db");
        assert_eq!(config.server_port, 8080);

        restore_var("DATABASE_URL", original_database);
        restore_var("SERVER_PORT", original_port);
    }

    #[test]
    fn test_config_port_defaults() {
        let _guard = env_guard();

        let original_database = env::var("DATABASE_URL").ok();
        let original_port = env::var("SERVER_PORT").ok();

        env::set_var("DATABASE_URL", "postgresql://localhost/test_db");
        env::set_var("SERVER_PORT", "invalid");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 3000);

        restore_var("DATABASE_URL", original_database);
        restore_var("SERVER_PORT", original_port);
    }

    #[test]
    fn test_config_missing_database_url_errors() {
        let _guard = env_guard();

        let original_database = env::var("DATABASE_URL").ok();
        let original_port = env::var("SERVER_PORT").ok();

        env::set_var("DATABASE_URL", "   ");
        env::remove_var("SERVER_PORT");

        let result = Config::from_env();
        assert!(matches!(result, Err(ConfigError::EmptyDatabaseUrl)));

        restore_var("DATABASE_URL", original_database);
        restore_var("SERVER_PORT", original_port);
    }

    #[test]
    fn test_config_struct_creation() {
        // Test that Config can be created with valid values
        let config = Config {
            database_url: "postgresql://localhost/test".to_string(),
            server_port: 8080,
        };
        assert_eq!(config.database_url, "postgresql://localhost/test");
        assert_eq!(config.server_port, 8080);
    }
}
