use std::{
    env::{self, VarError},
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use dotenv::dotenv;

use crate::error::ConfigError;

const DEFAULT_SERVER_PORT: u16 = 3000;
const DEFAULT_SERVER_HOST: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
const DEFAULT_DB_MAX_CONNECTIONS: u32 = 5;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub server_host: IpAddr,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").map_err(|source| ConfigError::EnvVar {
            key: "DATABASE_URL",
            source,
        })?;

        let database_max_connections =
            parse_env_or_default("DATABASE_MAX_CONNECTIONS", DEFAULT_DB_MAX_CONNECTIONS)?;

        let server_port = parse_env_or_default("SERVER_PORT", DEFAULT_SERVER_PORT)?;
        let server_host = match env::var("SERVER_HOST") {
            Ok(value) => parse_value("SERVER_HOST", &value, IpAddr::from_str)?,
            Err(VarError::NotPresent) => DEFAULT_SERVER_HOST,
            Err(source) => {
                return Err(ConfigError::EnvVar {
                    key: "SERVER_HOST",
                    source,
                })
            }
        };

        Ok(Self {
            database_url,
            database_max_connections,
            server_host,
            server_port,
        })
    }
}

fn parse_env_or_default<T>(key: &'static str, default: T) -> Result<T, ConfigError>
where
    T: FromStr,
    anyhow::Error: From<<T as FromStr>::Err>,
{
    match env::var(key) {
        Ok(value) => parse_value(key, &value, T::from_str),
        Err(VarError::NotPresent) => Ok(default),
        Err(source) => Err(ConfigError::EnvVar { key, source }),
    }
}

fn parse_value<T, F>(key: &'static str, value: &str, parser: F) -> Result<T, ConfigError>
where
    F: FnOnce(&str) -> Result<T, <T as FromStr>::Err>,
    T: FromStr,
    anyhow::Error: From<<T as FromStr>::Err>,
{
    parser(value).map_err(|source| ConfigError::InvalidEnvVar {
        key,
        source: source.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::net::Ipv4Addr;

    #[cfg(unix)]
    use std::os::unix::ffi::OsStringExt;

    #[test]
    #[serial_test::serial]
    fn config_defaults_are_applied_when_env_missing() {
        env::remove_var("DATABASE_URL");
        env::remove_var("SERVER_PORT");
        env::remove_var("SERVER_HOST");
        env::remove_var("DATABASE_MAX_CONNECTIONS");

        env::set_var("DATABASE_URL", "postgresql://example");

        let config = Config::from_env().expect("config should load");

        assert_eq!(config.server_port, DEFAULT_SERVER_PORT);
        assert_eq!(config.server_host, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        assert_eq!(config.database_max_connections, DEFAULT_DB_MAX_CONNECTIONS);
    }

    #[test]
    #[serial_test::serial]
    fn config_errors_on_missing_database_url() {
        env::remove_var("DATABASE_URL");

        let result = Config::from_env();

        assert!(result.is_err());
    }

    #[test]
    #[serial_test::serial]
    fn config_parses_custom_values() {
        env::set_var("DATABASE_URL", "postgresql://custom");
        env::set_var("SERVER_PORT", "8080");
        env::set_var("SERVER_HOST", "127.0.0.1");
        env::set_var("DATABASE_MAX_CONNECTIONS", "10");

        let config = Config::from_env().expect("config should load");

        assert_eq!(config.server_port, 8080);
        assert_eq!(config.server_host, IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(config.database_max_connections, 10);
        assert_eq!(config.database_url, "postgresql://custom");
    }

    #[test]
    #[serial_test::serial]
    fn config_rejects_invalid_server_port() {
        env::set_var("DATABASE_URL", "postgresql://example");
        env::set_var("SERVER_PORT", "invalid");

        let error = Config::from_env().expect_err("invalid port should cause error");

        if let ConfigError::InvalidEnvVar { key, .. } = error {
            assert_eq!(key, "SERVER_PORT");
        } else {
            panic!("unexpected error variant: {error:?}");
        }

        env::remove_var("SERVER_PORT");
        env::remove_var("DATABASE_URL");
    }

    #[test]
    #[serial_test::serial]
    fn config_rejects_invalid_host() {
        env::set_var("DATABASE_URL", "postgresql://example");
        env::set_var("SERVER_HOST", "not-a-host");

        let error = Config::from_env().expect_err("invalid host should cause error");

        if let ConfigError::InvalidEnvVar { key, .. } = error {
            assert_eq!(key, "SERVER_HOST");
        } else {
            panic!("unexpected error variant: {error:?}");
        }

        env::remove_var("SERVER_HOST");
        env::remove_var("DATABASE_URL");
    }

    #[test]
    #[serial_test::serial]
    fn config_rejects_invalid_max_connections() {
        env::set_var("DATABASE_URL", "postgresql://example");
        env::set_var("DATABASE_MAX_CONNECTIONS", "NaN");

        let error = Config::from_env().expect_err("invalid connection count should cause error");

        if let ConfigError::InvalidEnvVar { key, .. } = error {
            assert_eq!(key, "DATABASE_MAX_CONNECTIONS");
        } else {
            panic!("unexpected error variant: {error:?}");
        }

        env::remove_var("DATABASE_MAX_CONNECTIONS");
        env::remove_var("DATABASE_URL");
    }

    #[cfg(unix)]
    #[test]
    #[serial_test::serial]
    fn config_rejects_non_utf8_host() {
        use std::ffi::OsString;

        env::set_var("DATABASE_URL", "postgresql://example");
        env::set_var("SERVER_HOST", OsString::from_vec(vec![0xff]));

        let error = Config::from_env().expect_err("non-utf host should cause error");

        if let ConfigError::EnvVar { key, .. } = error {
            assert_eq!(key, "SERVER_HOST");
        } else {
            panic!("unexpected error variant: {error:?}");
        }

        env::remove_var("SERVER_HOST");
        env::remove_var("DATABASE_URL");
    }
}
