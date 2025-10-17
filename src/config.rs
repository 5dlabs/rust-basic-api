use anyhow::{Context, Result};
#[cfg(not(test))]
use dotenvy::dotenv;
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// `PostgreSQL` database connection URL
    pub database_url: String,
    /// `HTTP` server port
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing or invalid
    pub fn from_env() -> Result<Self> {
        // Load .env file if present (optional, won't fail if missing)
        // Skip loading .env in tests to avoid interference with test environment
        #[cfg(not(test))]
        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").context("DATABASE_URL environment variable must be set")?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()
            .context("SERVER_PORT must be a valid port number")?;

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

    // Use a lock to prevent tests from running in parallel
    // This is necessary because env vars are process-global
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_config_from_env_with_defaults() {
        let _lock = TEST_LOCK.lock().unwrap();
        
        // Clear any existing env vars that might interfere
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
        
        env::set_var("DATABASE_URL", "postgresql://localhost:5432/test");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.database_url, "postgresql://localhost:5432/test");
        assert_eq!(config.server_port, 3000);
        
        // Clean up
        env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_config_from_env_custom_port() {
        let _lock = TEST_LOCK.lock().unwrap();
        
        // Clear any existing env vars that might interfere
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
        
        env::set_var("DATABASE_URL", "postgresql://localhost:5432/test");
        env::set_var("SERVER_PORT", "8080");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 8080);
        
        // Clean up
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_missing_database_url() {
        let _lock = TEST_LOCK.lock().unwrap();
        
        // Clear any existing env vars
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
        
        let result = Config::from_env();
        assert!(result.is_err());
    }
}
