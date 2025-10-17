use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    #[allow(dead_code)] // Will be used in database setup task
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Serialize tests to avoid environment variable conflicts
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_default_port() {
        let _guard = TEST_MUTEX.lock().unwrap();
        env::remove_var("SERVER_PORT");
        env::set_var("DATABASE_URL", "postgresql://localhost/test");

        let config = Config::from_env().unwrap();
        assert_eq!(config.server_port, 3000);
    }

    #[test]
    fn test_custom_port() {
        let _guard = TEST_MUTEX.lock().unwrap();
        env::set_var("SERVER_PORT", "8080");
        env::set_var("DATABASE_URL", "postgresql://localhost/test");

        let config = Config::from_env().unwrap();
        assert_eq!(config.server_port, 8080);
    }
}
