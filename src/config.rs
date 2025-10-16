use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

// Allow dead_code for database_url as it's a placeholder for future database integration
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
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
