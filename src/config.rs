use dotenv::dotenv;
use std::env;

/// Configuration structure for the application
/// All values are loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Database connection URL (e.g., postgresql://user:password@host:port/database)
    pub database_url: String,
    /// Server port to listen on (default: 3000)
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing
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
        // Load .env file if present (optional in production)
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
    use std::env;

    #[test]
    fn test_config_from_env_with_database_url() {
        env::set_var(
            "DATABASE_URL",
            "postgresql://test:test@localhost:5432/testdb",
        );
        env::set_var("SERVER_PORT", "8080");

        let config = Config::from_env().expect("Failed to load config");

        assert_eq!(
            config.database_url,
            "postgresql://test:test@localhost:5432/testdb"
        );
        assert_eq!(config.server_port, 8080);

        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_default_port() {
        env::set_var(
            "DATABASE_URL",
            "postgresql://test:test@localhost:5432/testdb",
        );
        env::remove_var("SERVER_PORT");

        let config = Config::from_env().expect("Failed to load config");

        assert_eq!(config.server_port, 3000);

        env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_config_missing_database_url() {
        env::remove_var("DATABASE_URL");

        let result = Config::from_env();

        assert!(result.is_err());
    }
}
