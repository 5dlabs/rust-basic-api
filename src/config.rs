//! Configuration management module
//!
//! This module handles loading and managing application configuration
//! from environment variables using dotenv.

use dotenv::dotenv;
use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// `PostgreSQL` database connection URL
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
    pub fn from_env() -> Result<Self, env::VarError> {
        // Load .env file if it exists
        dotenv().ok();

        // Load required database URL
        let database_url = env::var("DATABASE_URL")?;

        // Load server port with default fallback
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse()
            .unwrap_or(3000);

        Ok(Self {
            database_url,
            server_port,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Ensure tests don't run in parallel since they modify env vars
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_config_default_port() {
        let _lock = TEST_MUTEX.lock().unwrap();

        // Set required DATABASE_URL for test
        env::set_var("DATABASE_URL", "postgresql://test:test@localhost/test");
        // Ensure SERVER_PORT is not set
        env::remove_var("SERVER_PORT");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 3000);
    }

    #[test]
    fn test_config_custom_port() {
        let _lock = TEST_MUTEX.lock().unwrap();

        env::set_var("DATABASE_URL", "postgresql://test:test@localhost/test");
        env::set_var("SERVER_PORT", "8080");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 8080);

        // Clean up
        env::remove_var("SERVER_PORT");
    }
}
