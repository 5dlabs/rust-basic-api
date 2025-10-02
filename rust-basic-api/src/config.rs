use anyhow;
use dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok(); // Load .env file if present
        let database_url =
            env::var("DATABASE_URL").map_err(|e: std::env::VarError| anyhow::anyhow!(e))?;
        let server_port: u16 = env::var("SERVER_PORT")
            .map_err(|e: std::env::VarError| anyhow::anyhow!(e))?
            .parse()
            .map_err(|e: std::num::ParseIntError| anyhow::anyhow!(e))?;
        Ok(Self {
            database_url,
            server_port,
        })
    }
}
