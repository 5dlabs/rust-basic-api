use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// User entity representing a database record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request DTO for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

/// Request DTO for updating an existing user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }

    #[test]
    fn test_create_user_request_serialization() {
        let request = CreateUserRequest {
            name: "Jane Doe".to_string(),
            email: "jane@example.com".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Jane Doe"));
        assert!(json.contains("jane@example.com"));
    }

    #[test]
    fn test_create_user_request_deserialization() {
        let json = r#"{"name":"Test User","email":"test@example.com"}"#;
        let request: CreateUserRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, "Test User");
        assert_eq!(request.email, "test@example.com");
    }

    #[test]
    fn test_update_user_request_partial_update() {
        let request = UpdateUserRequest {
            name: Some("Updated Name".to_string()),
            email: None,
        };

        assert!(request.name.is_some());
        assert!(request.email.is_none());
    }

    #[test]
    fn test_update_user_request_full_update() {
        let request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: Some("new@example.com".to_string()),
        };

        assert_eq!(request.name, Some("New Name".to_string()));
        assert_eq!(request.email, Some("new@example.com".to_string()));
    }

    #[test]
    fn test_update_user_request_deserialization() {
        let json = r#"{"name":"Updated"}"#;
        let request: UpdateUserRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, Some("Updated".to_string()));
        assert_eq!(request.email, None);
    }
}
