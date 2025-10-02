use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok(); // Load .env file if present
        let server_port: u16 = env::var("SERVER_PORT")
            .map_err(|e: std::env::VarError| anyhow::anyhow!(e))?
            .parse()
            .map_err(|e: std::num::ParseIntError| anyhow::anyhow!(e))?;
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| anyhow::anyhow!("DATABASE_URL not set"))?,
            server_port,
        })
    }
}
