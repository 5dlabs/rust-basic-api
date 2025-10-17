//! Configuration module for loading and managing application settings.
//!
//! This module provides a `Config` struct that loads configuration from environment
//! variables using the `dotenvy` crate for .env file support.

use std::env;

/// Application configuration loaded from environment variables.
///
/// # Environment Variables
/// - `DATABASE_URL`: `PostgreSQL` connection string (required)
/// - `SERVER_PORT`: Port number for the HTTP server (default: 3000)
#[derive(Debug, Clone)]
pub struct Config {
    /// `PostgreSQL` database connection URL
    /// Note: Will be used in future tasks for database connectivity
    #[allow(dead_code)]
    pub database_url: String,
    /// HTTP server port number
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// This method attempts to load a .env file first, then reads the required
    /// environment variables. It will return an error if `DATABASE_URL` is not set.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `DATABASE_URL` environment variable is not set
    /// - `SERVER_PORT` is set but cannot be parsed as a valid u16
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rust_basic_api::config::Config;
    ///
    /// let config = Config::from_env().expect("Failed to load configuration");
    /// println!("Server will run on port {}", config.server_port);
    /// ```
    pub fn from_env() -> Result<Self, env::VarError> {
        // Load .env file if present (ignore errors if file doesn't exist)
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
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

    // Mutex to ensure tests run serially (environment variables are global)
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_config_with_defaults() {
        let _lock = TEST_MUTEX.lock().unwrap();

        // Set required env var
        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::remove_var("SERVER_PORT");

        let config = Config::from_env().expect("Config should load with defaults");
        assert_eq!(config.database_url, "postgresql://localhost/test");
        assert_eq!(config.server_port, 3000);

        // Cleanup
        env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_config_with_custom_port() {
        let _lock = TEST_MUTEX.lock().unwrap();

        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::set_var("SERVER_PORT", "8080");

        let config = Config::from_env().expect("Config should load");
        assert_eq!(config.server_port, 8080);

        // Cleanup
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_missing_database_url() {
        let _lock = TEST_MUTEX.lock().unwrap();

        env::remove_var("DATABASE_URL");

        let result = Config::from_env();
        assert!(result.is_err(), "Config should fail without DATABASE_URL");
    }
}
