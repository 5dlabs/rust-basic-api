use anyhow::{anyhow, Error as AnyhowError};
use thiserror::Error;

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("environment variable `{0}` is required but missing")]
    Missing(&'static str),
    #[error("environment variable `{key}` contains invalid value `{value}`: {source}")]
    Invalid {
        key: &'static str,
        value: String,
        #[source]
        source: AnyhowError,
    },
}

impl ConfigError {
    #[must_use]
    pub fn invalid<E>(key: &'static str, value: String, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Invalid {
            key,
            value,
            source: AnyhowError::new(source),
        }
    }

    #[must_use]
    pub fn non_unicode(key: &'static str) -> Self {
        Self::Invalid {
            key,
            value: "<non-unicode>".to_string(),
            source: anyhow!("value contains invalid unicode"),
        }
    }
}
