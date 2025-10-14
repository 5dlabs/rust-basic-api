use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // Database URL will be used in Task 2 for database connectivity
    #[allow(dead_code)]
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
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
