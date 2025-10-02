use anyhow::Result;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    /// Database connection URL - will be used in Task 2 (Database Setup)
    #[allow(dead_code)]
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").map_err(|e| anyhow::anyhow!("DATABASE_URL not set: {e}"))?;
        let server_port: u16 = env::var("SERVER_PORT")
            .map_err(|e| anyhow::anyhow!("SERVER_PORT not set: {e}"))?
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid SERVER_PORT: {e}"))?;
        Ok(Self {
            database_url,
            server_port,
        })
    }
}
