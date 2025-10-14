//! Configuration management module
//!
//! Loads application configuration from environment variables using dotenvy.

use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// `PostgreSQL` database connection URL
    ///
    /// This field will be used in future tasks for database connectivity
    #[allow(dead_code)]
    pub database_url: String,
    /// HTTP server port
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing or invalid
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL"))?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidPort)?;

        Ok(Self {
            database_url,
            server_port,
        })
    }
}

/// Configuration-related errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Required environment variable is missing
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(&'static str),

    /// Server port value is invalid
    #[error("Invalid SERVER_PORT value")]
    InvalidPort,
}
