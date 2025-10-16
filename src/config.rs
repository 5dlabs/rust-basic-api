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
    /// # Errors
    ///
    /// Returns an error if `DATABASE_URL` is not set or if `SERVER_PORT` is invalid
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        Ok(Config {
            database_url,
            server_port,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        // Tests that Config loads from environment
        // Note: .env file is loaded by dotenv() in from_env()
        let config = Config::from_env().expect("Failed to load config");
        assert!(!config.database_url.is_empty());
        assert!(config.server_port > 0);
    }

    #[test]
    fn test_config_port_defaults() {
        // When SERVER_PORT is not set or invalid, should default to 3000
        let original_port = env::var("SERVER_PORT").ok();
        env::remove_var("SERVER_PORT");

        let config = Config::from_env().expect("Failed to load config");
        assert_eq!(config.server_port, 3000);

        // Restore original value if it existed
        if let Some(port) = original_port {
            env::set_var("SERVER_PORT", port);
        }
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
