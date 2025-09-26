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
