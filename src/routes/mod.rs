//! API route handlers module
//!
//! This module contains HTTP request handlers for all API endpoints

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::error::AppResult;

/// Health check endpoint handler
///
/// Returns a JSON response indicating the service is healthy
///
/// # Errors
///
/// This endpoint never returns an error
pub async fn health_check() -> AppResult<(StatusCode, Json<Value>)> {
    Ok((StatusCode::OK, Json(json!({"status": "OK"}))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let result = health_check().await;
        assert!(result.is_ok());
        let (status, _body) = result.expect("health check should succeed");
        assert_eq!(status, StatusCode::OK);
    }
}
