//! Configuration module for loading environment variables

use crate::error::{AppError, AppResult};
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// `PostgreSQL` database connection URL
    pub database_url: String,
    /// Server port to bind to
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing or invalid
    pub fn from_env() -> AppResult<Self> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|e| AppError::config(format!("DATABASE_URL not set: {e}")))?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse()
            .map_err(|e| AppError::config(format!("Invalid SERVER_PORT: {e}")))?;

        Ok(Self {
            database_url,
            server_port,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_with_default_port() {
        let original_db = env::var("DATABASE_URL").ok();
        let original_port = env::var("SERVER_PORT").ok();

        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::remove_var("SERVER_PORT");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 3000);
        assert_eq!(config.database_url, "postgresql://localhost/test");

        // Restore
        if let Some(val) = original_db {
            env::set_var("DATABASE_URL", val);
        }
        if let Some(val) = original_port {
            env::set_var("SERVER_PORT", val);
        }
    }

    #[test]
    fn test_config_with_custom_port() {
        let original_db = env::var("DATABASE_URL").ok();
        let original_port = env::var("SERVER_PORT").ok();

        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::set_var("SERVER_PORT", "8080");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 8080);

        // Restore
        if let Some(val) = original_db {
            env::set_var("DATABASE_URL", val);
        }
        if let Some(val) = original_port {
            env::set_var("SERVER_PORT", val);
        } else {
            env::remove_var("SERVER_PORT");
        }
    }

    #[test]
    fn test_config_invalid_port() {
        // Save current values
        let original_db = env::var("DATABASE_URL").ok();
        let original_port = env::var("SERVER_PORT").ok();

        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::set_var("SERVER_PORT", "invalid");

        let result = Config::from_env();

        // Restore original values
        if let Some(val) = original_db {
            env::set_var("DATABASE_URL", val);
        }
        if let Some(val) = original_port {
            env::set_var("SERVER_PORT", val);
        } else {
            env::remove_var("SERVER_PORT");
        }

        assert!(
            result.is_err(),
            "Expected error for invalid port, got: {result:?}"
        );
    }
}
