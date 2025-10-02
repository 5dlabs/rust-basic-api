use anyhow::Result;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    // database_url will be used in Task 2 (Database Setup)
    #[allow(dead_code)]
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    /// Automatically loads from .env file if present
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists (ok if it doesn't)
        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").map_err(|e| anyhow::anyhow!("DATABASE_URL not set: {e}"))?;

        let server_port: u16 = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid SERVER_PORT: {e}"))?;

        Ok(Self {
            database_url,
            server_port,
        })
    }
}
