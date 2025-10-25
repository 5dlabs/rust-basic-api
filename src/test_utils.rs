//! Test utilities and factory functions
//!
//! This module provides reusable test helpers and factory functions for creating
//! test data across the application. These utilities are only compiled when testing.

#[cfg(test)]
pub mod factories {
    use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};
    use chrono::Utc;

    /// Create a test user with default values
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    ///
    /// # Returns
    ///
    /// Returns a `User` with test data
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_basic_api::test_utils::factories::create_test_user;
    ///
    /// let user = create_test_user(1);
    /// assert_eq!(user.id, 1);
    /// assert_eq!(user.name, "Test User 1");
    /// ```
    #[must_use]
    pub fn create_test_user(id: i32) -> User {
        User {
            id,
            name: format!("Test User {id}"),
            email: format!("test{id}@example.com"),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Create a test user with custom name and email
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    /// * `name` - User name
    /// * `email` - User email
    ///
    /// # Returns
    ///
    /// Returns a `User` with specified data
    #[must_use]
    pub fn create_test_user_with_data(id: i32, name: &str, email: &str) -> User {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Create a test `CreateUserRequest` with default values
    ///
    /// # Arguments
    ///
    /// * `suffix` - Suffix to add to default values for uniqueness
    ///
    /// # Returns
    ///
    /// Returns a `CreateUserRequest` with test data
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_basic_api::test_utils::factories::create_user_request;
    ///
    /// let request = create_user_request(1);
    /// assert_eq!(request.name, "Test User 1");
    /// ```
    #[must_use]
    pub fn create_user_request(suffix: i32) -> CreateUserRequest {
        CreateUserRequest {
            name: format!("Test User {suffix}"),
            email: format!("test{suffix}@example.com"),
        }
    }

    /// Create a test `CreateUserRequest` with custom values
    ///
    /// # Arguments
    ///
    /// * `name` - User name
    /// * `email` - User email
    ///
    /// # Returns
    ///
    /// Returns a `CreateUserRequest` with specified data
    #[must_use]
    pub fn create_user_request_with_data(name: &str, email: &str) -> CreateUserRequest {
        CreateUserRequest {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    /// Create a test `UpdateUserRequest` with default values
    ///
    /// # Arguments
    ///
    /// * `suffix` - Suffix to add to default values
    ///
    /// # Returns
    ///
    /// Returns an `UpdateUserRequest` with test data
    #[must_use]
    pub fn update_user_request(suffix: i32) -> UpdateUserRequest {
        UpdateUserRequest {
            name: Some(format!("Updated User {suffix}")),
            email: Some(format!("updated{suffix}@example.com")),
        }
    }

    /// Create a test `UpdateUserRequest` with custom values
    ///
    /// # Arguments
    ///
    /// * `name` - Optional new name
    /// * `email` - Optional new email
    ///
    /// # Returns
    ///
    /// Returns an `UpdateUserRequest` with specified data
    #[must_use]
    pub fn update_user_request_with_data(
        name: Option<&str>,
        email: Option<&str>,
    ) -> UpdateUserRequest {
        UpdateUserRequest {
            name: name.map(ToString::to_string),
            email: email.map(ToString::to_string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::factories::*;

    #[test]
    fn test_create_test_user() {
        let user = create_test_user(42);
        assert_eq!(user.id, 42);
        assert_eq!(user.name, "Test User 42");
        assert_eq!(user.email, "test42@example.com");
    }

    #[test]
    fn test_create_test_user_with_data() {
        let user = create_test_user_with_data(1, "Custom Name", "custom@example.com");
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Custom Name");
        assert_eq!(user.email, "custom@example.com");
    }

    #[test]
    fn test_create_user_request() {
        let request = create_user_request(99);
        assert_eq!(request.name, "Test User 99");
        assert_eq!(request.email, "test99@example.com");
    }

    #[test]
    fn test_create_user_request_with_data() {
        let request = create_user_request_with_data("John Doe", "john@example.com");
        assert_eq!(request.name, "John Doe");
        assert_eq!(request.email, "john@example.com");
    }

    #[test]
    fn test_update_user_request() {
        let request = update_user_request(5);
        assert_eq!(request.name, Some("Updated User 5".to_string()));
        assert_eq!(request.email, Some("updated5@example.com".to_string()));
    }

    #[test]
    fn test_update_user_request_with_data() {
        let request = update_user_request_with_data(Some("New Name"), None);
        assert_eq!(request.name, Some("New Name".to_string()));
        assert_eq!(request.email, None);
    }
}
