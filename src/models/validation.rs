use super::user::{CreateUserRequest, UpdateUserRequest};
use crate::error::ApiError;

/// Validate a create user request
///
/// # Errors
///
/// Returns `ApiError::Validation` if validation fails
pub fn validate_create_request(req: &CreateUserRequest) -> Result<(), ApiError> {
    if req.name.trim().is_empty() {
        return Err(ApiError::Validation("Name cannot be empty".to_string()));
    }

    if req.name.len() > 255 {
        return Err(ApiError::Validation(
            "Name must be 255 characters or less".to_string(),
        ));
    }

    validate_email(&req.email)?;

    Ok(())
}

/// Validate an update user request
///
/// # Errors
///
/// Returns `ApiError::Validation` if validation fails
pub fn validate_update_request(req: &UpdateUserRequest) -> Result<(), ApiError> {
    if let Some(name) = &req.name {
        if name.trim().is_empty() {
            return Err(ApiError::Validation("Name cannot be empty".to_string()));
        }

        if name.len() > 255 {
            return Err(ApiError::Validation(
                "Name must be 255 characters or less".to_string(),
            ));
        }
    }

    if let Some(email) = &req.email {
        validate_email(email)?;
    }

    // At least one field must be provided
    if req.name.is_none() && req.email.is_none() {
        return Err(ApiError::Validation(
            "At least one field must be provided for update".to_string(),
        ));
    }

    Ok(())
}

/// Validate email format
///
/// # Errors
///
/// Returns `ApiError::Validation` if email is invalid
fn validate_email(email: &str) -> Result<(), ApiError> {
    if email.trim().is_empty() {
        return Err(ApiError::Validation("Email cannot be empty".to_string()));
    }

    if email.len() > 255 {
        return Err(ApiError::Validation(
            "Email must be 255 characters or less".to_string(),
        ));
    }

    // Basic email format validation
    if !email.contains('@') || !email.contains('.') {
        return Err(ApiError::Validation(
            "Email must be a valid email address".to_string(),
        ));
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(ApiError::Validation(
            "Email must be a valid email address".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_create_request_success() {
        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        assert!(validate_create_request(&request).is_ok());
    }

    #[test]
    fn test_validate_create_request_empty_name() {
        let request = CreateUserRequest {
            name: String::new(),
            email: "john@example.com".to_string(),
        };

        let result = validate_create_request(&request);
        assert!(result.is_err());
        assert!(matches!(result, Err(ApiError::Validation(_))));
    }

    #[test]
    fn test_validate_create_request_whitespace_name() {
        let request = CreateUserRequest {
            name: "   ".to_string(),
            email: "john@example.com".to_string(),
        };

        let result = validate_create_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_create_request_name_too_long() {
        let request = CreateUserRequest {
            name: "a".repeat(256),
            email: "john@example.com".to_string(),
        };

        let result = validate_create_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_email_success() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.user@example.co.uk").is_ok());
        assert!(validate_email("user+tag@example.com").is_ok());
    }

    #[test]
    fn test_validate_email_empty() {
        let result = validate_email("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_email_no_at_sign() {
        let result = validate_email("userexample.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_email_no_domain() {
        let result = validate_email("user@");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_email_no_local_part() {
        let result = validate_email("@example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_email_too_long() {
        let long_email = format!("{}@example.com", "a".repeat(250));
        let result = validate_email(&long_email);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_update_request_name_only() {
        let request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: None,
        };

        assert!(validate_update_request(&request).is_ok());
    }

    #[test]
    fn test_validate_update_request_email_only() {
        let request = UpdateUserRequest {
            name: None,
            email: Some("new@example.com".to_string()),
        };

        assert!(validate_update_request(&request).is_ok());
    }

    #[test]
    fn test_validate_update_request_both_fields() {
        let request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: Some("new@example.com".to_string()),
        };

        assert!(validate_update_request(&request).is_ok());
    }

    #[test]
    fn test_validate_update_request_no_fields() {
        let request = UpdateUserRequest {
            name: None,
            email: None,
        };

        let result = validate_update_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_update_request_empty_name() {
        let request = UpdateUserRequest {
            name: Some(String::new()),
            email: None,
        };

        let result = validate_update_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_update_request_invalid_email() {
        let request = UpdateUserRequest {
            name: None,
            email: Some("invalid-email".to_string()),
        };

        let result = validate_update_request(&request);
        assert!(result.is_err());
    }
}
