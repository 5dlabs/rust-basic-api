use std::{env::VarError, num::ParseIntError};

use thiserror::Error;

/// Errors that can occur while loading application configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("DATABASE_URL environment variable is not set")]
    MissingDatabaseUrl,
    #[error("environment variable {variable} could not be read")]
    EnvVar {
        variable: &'static str,
        #[source]
        source: VarError,
    },
    #[error("SERVER_PORT value '{value}' is not a valid unsigned 16-bit integer")]
    InvalidServerPort {
        value: String,
        #[source]
        source: ParseIntError,
    },
    #[error("DATABASE_MAX_CONNECTIONS value '{value}' is not a valid positive integer")]
    InvalidDatabaseMaxConnections {
        value: String,
        #[source]
        source: ParseIntError,
    },
    #[error("DATABASE_MAX_CONNECTIONS must be greater than zero (received {value})")]
    DatabaseMaxConnectionsZero { value: u32 },
    #[error("DATABASE_URL cannot be empty")]
    EmptyDatabaseUrl,
}

/// Convenient result alias for configuration loading operations.
pub type ConfigResult<T> = Result<T, ConfigError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::error::Error;

    #[test]
    fn config_error_display_messages() {
        assert_eq!(
            ConfigError::MissingDatabaseUrl.to_string(),
            "DATABASE_URL environment variable is not set"
        );

        assert_eq!(
            ConfigError::EmptyDatabaseUrl.to_string(),
            "DATABASE_URL cannot be empty"
        );

        let env_error = ConfigError::EnvVar {
            variable: "SERVER_PORT",
            source: env::VarError::NotPresent,
        };
        assert_eq!(
            env_error.to_string(),
            "environment variable SERVER_PORT could not be read"
        );

        let parse_error = ConfigError::InvalidServerPort {
            value: "invalid".to_string(),
            source: "invalid".parse::<u16>().unwrap_err(),
        };
        assert!(parse_error
            .to_string()
            .contains("SERVER_PORT value 'invalid' is not a valid unsigned 16-bit integer"));

        let db_parse_error = ConfigError::InvalidDatabaseMaxConnections {
            value: "negative".to_string(),
            source: "negative".parse::<u32>().unwrap_err(),
        };
        assert!(db_parse_error
            .to_string()
            .contains("DATABASE_MAX_CONNECTIONS value 'negative' is not a valid positive integer"));

        let zero_error = ConfigError::DatabaseMaxConnectionsZero { value: 0 };
        assert_eq!(
            zero_error.to_string(),
            "DATABASE_MAX_CONNECTIONS must be greater than zero (received 0)"
        );
    }

    #[test]
    fn config_error_sources() {
        let env_error = ConfigError::EnvVar {
            variable: "TEST_VAR",
            source: env::VarError::NotPresent,
        };
        assert!(env_error.source().is_some());

        let parse_error = ConfigError::InvalidServerPort {
            value: "invalid".to_string(),
            source: "invalid".parse::<u16>().unwrap_err(),
        };
        assert!(parse_error.source().is_some());

        let db_parse_error = ConfigError::InvalidDatabaseMaxConnections {
            value: "invalid".to_string(),
            source: "invalid".parse::<u32>().unwrap_err(),
        };
        assert!(db_parse_error.source().is_some());

        // These errors don't have sources
        assert!(ConfigError::MissingDatabaseUrl.source().is_none());
        assert!(ConfigError::EmptyDatabaseUrl.source().is_none());
        assert!(ConfigError::DatabaseMaxConnectionsZero { value: 0 }
            .source()
            .is_none());
    }
}
