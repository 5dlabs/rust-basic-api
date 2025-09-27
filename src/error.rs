use std::{env::VarError, num::ParseIntError};

use thiserror::Error;

/// Errors that can occur while loading application configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing or invalid environment variable `{name}`: {source}")]
    EnvVar {
        name: &'static str,
        #[source]
        source: VarError,
    },
    #[error("failed to parse server port `{value}`: {source}")]
    InvalidPort {
        value: String,
        #[source]
        source: ParseIntError,
    },
}
