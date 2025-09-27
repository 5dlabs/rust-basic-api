use std::env;

use dotenv::dotenv;

use crate::error::ConfigError;

const DEFAULT_SERVER_PORT: u16 = 3000;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").map_err(|source| ConfigError::EnvVar {
            name: "DATABASE_URL",
            source,
        })?;

        let server_port = match env::var("SERVER_PORT") {
            Ok(raw) => raw
                .parse::<u16>()
                .map_err(|source| ConfigError::InvalidPort { value: raw, source })?,
            Err(env::VarError::NotPresent) => DEFAULT_SERVER_PORT,
            Err(source) => {
                return Err(ConfigError::EnvVar {
                    name: "SERVER_PORT",
                    source,
                })
            }
        };

        Ok(Self {
            database_url,
            server_port,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use std::ffi::OsString;
    #[cfg(unix)]
    use std::os::unix::ffi::OsStringExt;
    use std::{
        env,
        sync::{Mutex, OnceLock},
    };

    fn env_lock() -> &'static Mutex<()> {
        static ENV_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_GUARD.get_or_init(|| Mutex::new(()))
    }

    fn clear_env() {
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
    }

    #[test]
    fn loads_defaults_when_optional_port_missing() {
        let _guard = env_lock().lock().unwrap();
        clear_env();
        env::set_var("DATABASE_URL", "postgres://localhost/test");

        let config = Config::from_env().expect("configuration loads");

        assert_eq!(config.database_url, "postgres://localhost/test");
        assert_eq!(config.server_port, DEFAULT_SERVER_PORT);
    }

    #[test]
    fn errors_when_database_url_missing() {
        let _guard = env_lock().lock().unwrap();
        clear_env();

        let error = Config::from_env().expect_err("missing database URL should error");

        assert!(matches!(
            error,
            ConfigError::EnvVar { name, .. } if name == "DATABASE_URL"
        ));
    }

    #[test]
    fn errors_on_invalid_port() {
        let _guard = env_lock().lock().unwrap();
        clear_env();
        env::set_var("DATABASE_URL", "postgres://localhost/test");
        env::set_var("SERVER_PORT", "not-a-number");

        let error = Config::from_env().expect_err("invalid server port should error");

        assert!(matches!(error, ConfigError::InvalidPort { .. }));
    }

    #[test]
    fn loads_explicit_port_when_set() {
        let _guard = env_lock().lock().unwrap();
        clear_env();
        env::set_var("DATABASE_URL", "postgres://localhost/test");
        env::set_var("SERVER_PORT", "4000");

        let config = Config::from_env().expect("configuration loads with explicit port");

        assert_eq!(config.server_port, 4000);
    }

    #[cfg(unix)]
    #[test]
    fn errors_on_non_utf8_port() {
        let _guard = env_lock().lock().unwrap();
        clear_env();
        env::set_var("DATABASE_URL", "postgres://localhost/test");
        let invalid = OsString::from_vec(vec![0xff, 0xfe, 0xfd]);
        env::set_var("SERVER_PORT", &invalid);

        let error = Config::from_env().expect_err("non UTF-8 server port should error");

        assert!(matches!(
            error,
            ConfigError::EnvVar { name, .. } if name == "SERVER_PORT"
        ));
    }
}
