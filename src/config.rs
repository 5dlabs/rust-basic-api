use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;

/// Configuration for the application loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Database connection URL (currently unused but required for future database integration)
    /// Justification for `dead_code`: This field will be used when database connection pooling is implemented
    #[allow(dead_code)]
    pub database_url: String,
    /// Port number for the HTTP server
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").context("DATABASE_URL environment variable must be set")?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .context("SERVER_PORT must be a valid u16 value")?;

        Ok(Self {
            database_url,
            server_port,
        })
    }
}
