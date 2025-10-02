use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL").map_err(anyhow::Error::msg)?;
        let server_port: u16 = std::env::var("SERVER_PORT")
            .map_err(anyhow::Error::msg)?
            .parse()
            .map_err(anyhow::Error::msg)?;
        Ok(Self {
            database_url,
            server_port,
        })
    }
}
