use dotenvy::dotenv;
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// `PostgreSQL` database connection URL
    pub database_url: String,
    /// HTTP server port
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if required or malformed environment variables are encountered
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").map_err(|err| ConfigError::MissingEnvVar {
            key: "DATABASE_URL",
            source: err,
        })?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|err| ConfigError::InvalidServerPort { source: err })?;

        Ok(Config {
            database_url,
            server_port,
        })
    }
}

/// Errors that can occur during configuration loading
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Missing required environment variable
    #[error("Missing required environment variable {key}")]
    MissingEnvVar {
        key: &'static str,
        #[source]
        source: env::VarError,
    },

    /// Invalid server port value
    #[error("Invalid SERVER_PORT value")]
    InvalidServerPort {
        #[source]
        source: std::num::ParseIntError,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_default_server_port() {
        let _guard = ENV_MUTEX.lock().unwrap();

        env::remove_var("SERVER_PORT");
        env::set_var("DATABASE_URL", "postgres://example");

        let config = Config::from_env().expect("config loads with defaults");
        assert_eq!(config.server_port, 3000);
    }

    #[test]
    fn test_invalid_server_port() {
        let _guard = ENV_MUTEX.lock().unwrap();

        env::set_var("SERVER_PORT", "invalid");
        env::set_var("DATABASE_URL", "postgres://example");

        let result = Config::from_env();
        assert!(matches!(result, Err(ConfigError::InvalidServerPort { .. })));
    }
}
