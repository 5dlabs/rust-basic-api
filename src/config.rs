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
    /// Database pool: maximum connections
    pub db_max_connections: u32,
    /// Database pool: minimum idle connections
    pub db_min_connections: u32,
    /// Database pool: connect timeout in seconds
    pub db_connect_timeout_secs: u64,
    /// Database pool: idle timeout in seconds
    pub db_idle_timeout_secs: u64,
    /// Database pool: acquire timeout in seconds
    pub db_acquire_timeout_secs: u64,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Environment Variables
    ///
    /// - `DATABASE_URL` (required): `PostgreSQL` connection string
    /// - `SERVER_PORT` (optional): HTTP server port, defaults to 3000
    /// - `DB_MAX_CONNECTIONS` (optional): max connections, default 10
    /// - `DB_MIN_CONNECTIONS` (optional): min idle connections, default 1
    /// - `DB_CONNECT_TIMEOUT_SECS` (optional): connect timeout, default 5
    /// - `DB_IDLE_TIMEOUT_SECS` (optional): idle timeout, default 300
    /// - `DB_ACQUIRE_TIMEOUT_SECS` (optional): acquire timeout, default 30
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing
    pub fn from_env() -> Result<Self, env::VarError> {
        // Load environment variables from a .env file if present (dev/test only)
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
        let db_max_connections = env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);
        let db_min_connections = env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .unwrap_or(1);
        let db_connect_timeout_secs = env::var("DB_CONNECT_TIMEOUT_SECS")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .unwrap_or(5);
        let db_idle_timeout_secs = env::var("DB_IDLE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "300".to_string())
            .parse()
            .unwrap_or(300);
        let db_acquire_timeout_secs = env::var("DB_ACQUIRE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30);

        Ok(Self {
            database_url,
            server_port,
            db_max_connections,
            db_min_connections,
            db_connect_timeout_secs,
            db_idle_timeout_secs,
            db_acquire_timeout_secs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Mutex to prevent parallel test execution that interferes with env vars
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn sample_database_url() -> String {
        format!(
            "{scheme}://{user}:{pass}@{host}/{db}",
            scheme = "postgresql",
            user = "testuser",
            pass = "testpass",
            host = "localhost:5432",
            db = "testdb"
        )
    }

    #[test]
    fn test_config_default_port() {
        let _lock = TEST_LOCK.lock().unwrap();

        env::set_var("DATABASE_URL", sample_database_url());
        env::remove_var("SERVER_PORT");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 3000);

        // Cleanup
        env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_config_custom_port() {
        let _lock = TEST_LOCK.lock().unwrap();

        env::set_var("DATABASE_URL", sample_database_url());
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
