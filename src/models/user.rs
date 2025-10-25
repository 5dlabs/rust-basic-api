//! User model and related DTOs
//!
//! This module contains the User domain model and Data Transfer Objects (DTOs)
//! for creating and updating users.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// User domain model representing a user in the system
///
/// This struct represents the complete user entity as stored in the database.
/// It includes all fields including timestamps and the database-generated ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for the user
    pub id: i32,

    /// User's display name
    pub name: String,

    /// User's email address
    pub email: String,

    /// Timestamp when the user was created
    pub created_at: DateTime<Utc>,

    /// Timestamp when the user was last updated
    pub updated_at: DateTime<Utc>,
}

/// Request DTO for creating a new user
///
/// This struct is used to validate incoming requests for creating new users.
/// All fields are required and validated according to the specified constraints.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    /// User's display name (1-255 characters)
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    /// User's email address (must be valid email format)
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Request DTO for updating an existing user
///
/// This struct is used to validate incoming requests for updating users.
/// All fields are optional, allowing partial updates.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    /// Optional new name for the user (1-255 characters if provided)
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    /// Optional new email for the user (must be valid email format if provided)
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_user_serialization() {
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&user).expect("Failed to serialize");
        let deserialized: User = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(user.id, deserialized.id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.email, deserialized.email);
    }

    #[test]
    fn test_create_user_request_valid() {
        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_user_request_invalid_email() {
        let request = CreateUserRequest {
            name: "John Doe".to_string(),
            email: "invalid-email".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());

        if let Err(errors) = result {
            assert!(errors.field_errors().contains_key("email"));
        }
    }

    #[test]
    fn test_create_user_request_empty_name() {
        let request = CreateUserRequest {
            name: String::new(),
            email: "john@example.com".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());

        if let Err(errors) = result {
            assert!(errors.field_errors().contains_key("name"));
        }
    }

    #[test]
    fn test_create_user_request_name_too_long() {
        let request = CreateUserRequest {
            name: "a".repeat(256),
            email: "john@example.com".to_string(),
        };

        let result = request.validate();
        assert!(result.is_err());

        if let Err(errors) = result {
            assert!(errors.field_errors().contains_key("name"));
        }
    }

    #[test]
    fn test_update_user_request_all_fields() {
        let request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: Some("new@example.com".to_string()),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_user_request_partial() {
        let request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_user_request_none() {
        let request = UpdateUserRequest {
            name: None,
            email: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_user_request_invalid_email() {
        let request = UpdateUserRequest {
            name: None,
            email: Some("invalid-email".to_string()),
        };

        let result = request.validate();
        assert!(result.is_err());

        if let Err(errors) = result {
            assert!(errors.field_errors().contains_key("email"));
        }
    }

    #[test]
    fn test_update_user_request_empty_name() {
        let request = UpdateUserRequest {
            name: Some(String::new()),
            email: None,
        };

        let result = request.validate();
        assert!(result.is_err());

        if let Err(errors) = result {
            assert!(errors.field_errors().contains_key("name"));
        }
    }

    #[test]
    fn test_update_user_request_serialization_with_some() {
        let request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: None,
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(json.contains("name"));
        assert!(json.contains("New Name"));
    }

    #[test]
    fn test_update_user_request_serialization_with_none() {
        let request = UpdateUserRequest {
            name: None,
            email: Some("test@example.com".to_string()),
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(json.contains("email"));
        assert!(json.contains("test@example.com"));
    }
}
