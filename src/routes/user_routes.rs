use crate::error::ApiError;
use crate::models::{validation, CreateUserRequest, UpdateUserRequest, User};
use crate::repository::{SqlxUserRepository, UserRepository};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

/// Get all users
///
/// # Errors
///
/// Returns `ApiError` if database query fails
pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, ApiError> {
    let repo = SqlxUserRepository::new(pool);
    let users = repo.get_all().await?;
    Ok(Json(users))
}

/// Get user by ID
///
/// # Errors
///
/// Returns `ApiError::NotFound` if user doesn't exist
/// Returns `ApiError::Database` if database query fails
pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let repo = SqlxUserRepository::new(pool);
    let user = repo.get_by_id(id).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(user))
}

/// Create a new user
///
/// # Errors
///
/// Returns `ApiError::Validation` if request validation fails
/// Returns `ApiError::Database` if database insert fails
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    // Validate request
    validation::validate_create_request(&request)?;

    // Create user
    let repo = SqlxUserRepository::new(pool);
    let user = repo.create(&request).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// Update an existing user
///
/// # Errors
///
/// Returns `ApiError::NotFound` if user doesn't exist
/// Returns `ApiError::Validation` if request validation fails
/// Returns `ApiError::Database` if database update fails
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<User>, ApiError> {
    // Validate request
    validation::validate_update_request(&request)?;

    // Update user
    let repo = SqlxUserRepository::new(pool);
    let user = repo.update(id, &request).await?.ok_or(ApiError::NotFound)?;

    Ok(Json(user))
}

/// Delete a user
///
/// # Errors
///
/// Returns `ApiError::NotFound` if user doesn't exist
/// Returns `ApiError::Database` if database delete fails
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    let repo = SqlxUserRepository::new(pool);
    let deleted = repo.delete(id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These are unit tests for handler logic
    // Integration tests with database would go in tests/ directory

    #[test]
    fn test_create_user_request_deserialization() {
        let json = r#"{"name":"Test User","email":"test@example.com"}"#;
        let request: CreateUserRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, "Test User");
        assert_eq!(request.email, "test@example.com");
    }

    #[test]
    fn test_update_user_request_deserialization() {
        let json = r#"{"name":"Updated Name"}"#;
        let request: UpdateUserRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.name, Some("Updated Name".to_string()));
        assert_eq!(request.email, None);
    }

    #[test]
    fn test_validation_is_called() {
        // Test that validation catches empty names
        let request = CreateUserRequest {
            name: String::new(),
            email: "test@example.com".to_string(),
        };

        let result = validation::validate_create_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_catches_invalid_email() {
        let request = CreateUserRequest {
            name: "Test".to_string(),
            email: "invalid-email".to_string(),
        };

        let result = validation::validate_create_request(&request);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_validation_requires_at_least_one_field() {
        let request = UpdateUserRequest {
            name: None,
            email: None,
        };

        let result = validation::validate_update_request(&request);
        assert!(result.is_err());
    }
}
