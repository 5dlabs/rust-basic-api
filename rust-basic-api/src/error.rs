use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::env;
use thiserror::Error;

/// Convenience result type used across the application.
pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("environment variable `{key}` is not available")]
    EnvVar {
        key: &'static str,
        #[source]
        source: env::VarError,
    },
    #[error("environment variable `{key}` is not valid: {source}")]
    InvalidEnvVar {
        key: &'static str,
        #[source]
        source: anyhow::Error,
    },
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Config(_) | Self::Database(_) | Self::Unexpected(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = axum::Json(ErrorBody {
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::anyhow;
    use hyper::body::to_bytes;
    use serde_json::Value;

    #[tokio::test]
    async fn into_response_returns_internal_server_error() {
        let error = AppError::Config(ConfigError::InvalidEnvVar {
            key: "SERVER_PORT",
            source: anyhow!("invalid"),
        });

        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body_bytes = to_bytes(response.into_body())
            .await
            .expect("body should be readable");
        let value: Value = serde_json::from_slice(&body_bytes).expect("json body");

        assert_eq!(
            value["message"],
            "environment variable `SERVER_PORT` is not valid: invalid"
        );
    }

    #[test]
    fn config_error_env_var_variant_includes_key_name() {
        let error = ConfigError::EnvVar {
            key: "DATABASE_URL",
            source: env::VarError::NotPresent,
        };

        assert!(
            error.to_string().contains("DATABASE_URL"),
            "error message should include key name"
        );
    }
}
