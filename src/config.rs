//! Configuration management module
//!
//! This module handles loading and managing application configuration from environment variables.

use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// `PostgreSQL` database connection URL
    pub database_url: String,
    /// Server port for HTTP listener
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Environment Variables
    ///
    /// - `DATABASE_URL` (required): `PostgreSQL` connection string
    /// - `SERVER_PORT` (optional): HTTP server port, defaults to 3000
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Mutex to prevent parallel test execution that interferes with env vars
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_config_default_port() {
        let _lock = TEST_LOCK.lock().unwrap();

        env::set_var(
            "DATABASE_URL",
            "postgresql://testuser:testpass@localhost/testdb",
        );
        env::remove_var("SERVER_PORT");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 3000);

        // Cleanup
        env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_config_custom_port() {
        let _lock = TEST_LOCK.lock().unwrap();

        env::set_var(
            "DATABASE_URL",
            "postgresql://testuser:testpass@localhost/testdb",
        );
        env::set_var("SERVER_PORT", "8080");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 8080);

        // Cleanup
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_missing_database_url() {
        let _lock = TEST_LOCK.lock().unwrap();

        env::remove_var("DATABASE_URL");
        let result = Config::from_env();
        assert!(result.is_err());
    }
}
