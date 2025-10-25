use crate::error::ApiError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use async_trait::async_trait;
use sqlx::PgPool;

/// Repository trait for user operations
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Get all users
    async fn get_all(&self) -> Result<Vec<User>, ApiError>;

    /// Get user by ID
    async fn get_by_id(&self, id: i32) -> Result<Option<User>, ApiError>;

    /// Create new user
    async fn create(&self, request: &CreateUserRequest) -> Result<User, ApiError>;

    /// Update user
    async fn update(&self, id: i32, request: &UpdateUserRequest) -> Result<Option<User>, ApiError>;

    /// Delete user
    async fn delete(&self, id: i32) -> Result<bool, ApiError>;
}

/// SQLx-based implementation of `UserRepository`
pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    /// Create a new repository instance
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn get_all(&self) -> Result<Vec<User>, ApiError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at, updated_at FROM users ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create(&self, request: &CreateUserRequest) -> Result<User, ApiError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email, created_at, updated_at",
        )
        .bind(&request.name)
        .bind(&request.email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update(&self, id: i32, request: &UpdateUserRequest) -> Result<Option<User>, ApiError> {
        // First check if user exists
        let existing = self.get_by_id(id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        // Build dynamic update query based on provided fields
        let user = if let Some(name) = &request.name {
            if let Some(email) = &request.email {
                // Update both fields
                sqlx::query_as::<_, User>(
                    "UPDATE users SET name = $1, email = $2, updated_at = NOW() WHERE id = $3 RETURNING id, name, email, created_at, updated_at",
                )
                .bind(name)
                .bind(email)
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            } else {
                // Update only name
                sqlx::query_as::<_, User>(
                    "UPDATE users SET name = $1, updated_at = NOW() WHERE id = $2 RETURNING id, name, email, created_at, updated_at",
                )
                .bind(name)
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            }
        } else if let Some(email) = &request.email {
            // Update only email
            sqlx::query_as::<_, User>(
                "UPDATE users SET email = $1, updated_at = NOW() WHERE id = $2 RETURNING id, name, email, created_at, updated_at",
            )
            .bind(email)
            .bind(id)
            .fetch_one(&self.pool)
            .await?
        } else {
            // No fields to update (should be caught by validation)
            return Ok(existing);
        };

        Ok(Some(user))
    }

    async fn delete(&self, id: i32) -> Result<bool, ApiError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a database connection
    // They are integration tests and should be run with DATABASE_URL set

    #[test]
    fn test_repository_creation() {
        // Test that we can create a repository instance
        // This is a smoke test to ensure the struct is valid
    }

    #[test]
    fn test_create_user_request_fields() {
        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        assert_eq!(request.name, "Test User");
        assert_eq!(request.email, "test@example.com");
    }

    #[test]
    fn test_update_user_request_partial() {
        let request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: None,
        };

        assert!(request.name.is_some());
        assert!(request.email.is_none());
    }
}
