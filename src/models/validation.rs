//! Validation utilities for request DTOs
//!
//! This module provides generic validation functions that can be used
//! across different request types.

use crate::error::ApiError;
use validator::Validate;

/// Validates a request DTO and converts validation errors to `ApiError`
///
/// This function takes any type that implements the `Validate` trait
/// and returns either `Ok(())` if validation passes or an `ApiError::Validation`
/// containing the validation error details.
///
/// # Errors
///
/// Returns `ApiError::Validation` if the request fails validation checks.
///
/// # Examples
///
/// ```
/// use rust_basic_api::models::{CreateUserRequest, validate_request};
///
/// let request = CreateUserRequest {
///     name: "John Doe".to_string(),
///     email: "john@example.com".to_string(),
/// };
///
/// assert!(validate_request(&request).is_ok());
/// ```
pub fn validate_request<T: Validate>(request: &T) -> Result<(), ApiError> {
    request.validate().map_err(ApiError::Validation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::{CreateUserRequest, UpdateUserRequest};

    #[test]
    fn test_validate_request_valid_create_user() {
        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        let result = validate_request(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_request_invalid_email() {
        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "not-an-email".to_string(),
        };

        let result = validate_request(&request);
        assert!(result.is_err());
        assert!(matches!(result, Err(ApiError::Validation(_))));
    }

    #[test]
    fn test_validate_request_empty_name() {
        let request = CreateUserRequest {
            name: String::new(),
            email: "john@example.com".to_string(),
        };

        let result = validate_request(&request);
        assert!(result.is_err());
        assert!(matches!(result, Err(ApiError::Validation(_))));
    }

    #[test]
    fn test_validate_request_valid_update_partial() {
        let request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: None,
        };

        let result = validate_request(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_request_invalid_update_email() {
        let request = UpdateUserRequest {
            name: None,
            email: Some("invalid".to_string()),
        };

        let result = validate_request(&request);
        assert!(result.is_err());
        assert!(matches!(result, Err(ApiError::Validation(_))));
    }

    #[test]
    fn test_validate_request_update_empty_name() {
        let request = UpdateUserRequest {
            name: Some(String::new()),
            email: None,
        };

        let result = validate_request(&request);
        assert!(result.is_err());
        assert!(matches!(result, Err(ApiError::Validation(_))));
    }

    #[test]
    fn test_validate_request_name_too_long() {
        let request = CreateUserRequest {
            name: "a".repeat(256),
            email: "john@example.com".to_string(),
        };

        let result = validate_request(&request);
        assert!(result.is_err());
        assert!(matches!(result, Err(ApiError::Validation(_))));
    }
}
