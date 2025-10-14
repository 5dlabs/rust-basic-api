//! Configuration management module
//!
//! Loads application configuration from environment variables using dotenv.

use dotenv::dotenv;
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
    /// Reads `.env` file if present and loads:
    /// - `DATABASE_URL` (required)
    /// - `SERVER_PORT` (optional, defaults to 3000)
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are missing
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        Ok(Self {
            database_url,
            server_port,
        })
    }
}
