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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    fn test_config_struct_creation() {
        let config = Config {
            database_url: "postgresql://test:test@localhost/testdb".to_string(),
            server_port: 8080,
        };
        assert_eq!(
            config.database_url,
            "postgresql://test:test@localhost/testdb"
        );
        assert_eq!(config.server_port, 8080);
    }

    #[test]
    #[serial]
    fn test_config_from_env_with_valid_vars() {
        // Set required environment variables
        env::set_var("DATABASE_URL", "postgresql://user:pass@localhost:5432/db");
        env::set_var("SERVER_PORT", "8080");

        let config = Config::from_env().expect("Should load config successfully");
        assert_eq!(
            config.database_url,
            "postgresql://user:pass@localhost:5432/db"
        );
        assert_eq!(config.server_port, 8080);

        // Clean up
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    #[serial]
    fn test_config_from_env_default_port() {
        // Set only required DATABASE_URL
        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::remove_var("SERVER_PORT");

        let config = Config::from_env().expect("Should use default port");
        assert_eq!(config.server_port, 3000);

        // Clean up
        env::remove_var("DATABASE_URL");
    }

    #[test]
    #[serial]
    fn test_config_from_env_missing_database_url() {
        // Ensure DATABASE_URL is not set
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");

        let result = Config::from_env();
        assert!(result.is_err());

        if let Err(ConfigError::MissingEnvVar(var)) = result {
            assert_eq!(var, "DATABASE_URL");
        } else {
            panic!("Expected MissingEnvVar error");
        }
    }

    #[test]
    #[serial]
    fn test_config_from_env_invalid_port() {
        // Set DATABASE_URL but invalid port
        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::set_var("SERVER_PORT", "invalid_port");

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::InvalidPort)));

        // Clean up
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    #[serial]
    fn test_config_from_env_port_out_of_range() {
        // Set DATABASE_URL but port > u16::MAX
        env::set_var("DATABASE_URL", "postgresql://localhost/test");
        env::set_var("SERVER_PORT", "99999");

        let result = Config::from_env();
        assert!(result.is_err());
        assert!(matches!(result, Err(ConfigError::InvalidPort)));

        // Clean up
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_error_display() {
        let error = ConfigError::MissingEnvVar("TEST_VAR");
        assert_eq!(
            format!("{error}"),
            "Missing required environment variable: TEST_VAR"
        );

        let error = ConfigError::InvalidPort;
        assert_eq!(format!("{error}"), "Invalid SERVER_PORT value");
    }

    #[test]
    fn test_config_error_debug() {
        let error = ConfigError::MissingEnvVar("DATABASE_URL");
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("MissingEnvVar"));
    }

    #[test]
    fn test_config_clone() {
        let config = Config {
            database_url: "postgresql://test:test@localhost/testdb".to_string(),
            server_port: 3000,
        };
        let cloned = config.clone();
        assert_eq!(cloned.database_url, config.database_url);
        assert_eq!(cloned.server_port, config.server_port);
    }

    #[test]
    fn test_config_debug() {
        let config = Config {
            database_url: "postgresql://test:test@localhost/testdb".to_string(),
            server_port: 3000,
        };
        let debug_str = format!("{config:?}");
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("3000"));
    }
}
