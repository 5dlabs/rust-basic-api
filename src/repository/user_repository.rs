//! User repository - database operations for User entities
//!
//! This module provides the repository pattern implementation for user data access.
//! It defines a trait for database operations and implements it using `SQLx` for `PostgreSQL`.

use async_trait::async_trait;
use sqlx::PgPool;

use crate::error::ApiError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};

/// Repository trait for user database operations
///
/// This trait defines the interface for all user-related database operations.
/// It allows for different implementations (e.g., `SQLx`, mock for testing)
/// while maintaining a consistent API.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user in the database
    ///
    /// # Arguments
    ///
    /// * `request` - User creation request containing name and email
    ///
    /// # Returns
    ///
    /// Returns the created user with generated ID and timestamps
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Database connection fails
    /// - Email already exists (unique constraint violation)
    /// - Database insert operation fails
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, ApiError>;

    /// Get a user by their ID
    ///
    /// # Arguments
    ///
    /// * `id` - The user's unique identifier
    ///
    /// # Returns
    ///
    /// Returns `Some(User)` if found, `None` if not found
    ///
    /// # Errors
    ///
    /// Returns an error if database query fails
    async fn get_user(&self, id: i32) -> Result<Option<User>, ApiError>;

    /// Get all users from the database
    ///
    /// # Returns
    ///
    /// Returns a vector of all users, ordered by ID
    ///
    /// # Errors
    ///
    /// Returns an error if database query fails
    async fn get_users(&self) -> Result<Vec<User>, ApiError>;

    /// Update a user's information
    ///
    /// # Arguments
    ///
    /// * `id` - The user's unique identifier
    /// * `request` - Update request with optional name and/or email
    ///
    /// # Returns
    ///
    /// Returns `Some(User)` with updated data if user exists, `None` if not found
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Database connection fails
    /// - Email already exists (unique constraint violation)
    /// - Database update operation fails
    async fn update_user(
        &self,
        id: i32,
        request: UpdateUserRequest,
    ) -> Result<Option<User>, ApiError>;

    /// Delete a user from the database
    ///
    /// # Arguments
    ///
    /// * `id` - The user's unique identifier
    ///
    /// # Returns
    ///
    /// Returns `true` if user was deleted, `false` if user was not found
    ///
    /// # Errors
    ///
    /// Returns an error if database operation fails
    async fn delete_user(&self, id: i32) -> Result<bool, ApiError>;
}

/// `SQLx`-based implementation of the `UserRepository` trait
///
/// This implementation uses `PostgreSQL` via `SQLx` for all database operations.
/// It uses a connection pool for efficient resource management.
#[derive(Clone)]
pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    /// Create a new `SQLx` user repository
    ///
    /// # Arguments
    ///
    /// * `pool` - `PostgreSQL` connection pool
    ///
    /// # Returns
    ///
    /// Returns a new `SqlxUserRepository` instance
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, ApiError> {
        let user = sqlx::query_as::<_, User>(
            r"
            INSERT INTO users (name, email)
            VALUES ($1, $2)
            RETURNING id, name, email, created_at, updated_at
            ",
        )
        .bind(&request.name)
        .bind(&request.email)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create user: {}", e);
            ApiError::Database(e)
        })?;

        tracing::info!("Created user with id: {}", user.id);
        Ok(user)
    }

    async fn get_user(&self, id: i32) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as::<_, User>(
            r"
            SELECT id, name, email, created_at, updated_at
            FROM users
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user {}: {}", id, e);
            ApiError::Database(e)
        })?;

        Ok(user)
    }

    async fn get_users(&self) -> Result<Vec<User>, ApiError> {
        let users = sqlx::query_as::<_, User>(
            r"
            SELECT id, name, email, created_at, updated_at
            FROM users
            ORDER BY id
            ",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get users: {}", e);
            ApiError::Database(e)
        })?;

        Ok(users)
    }

    async fn update_user(
        &self,
        id: i32,
        request: UpdateUserRequest,
    ) -> Result<Option<User>, ApiError> {
        // First check if user exists
        let existing_user = self.get_user(id).await?;
        if existing_user.is_none() {
            return Ok(None);
        }

        // Build dynamic SQL query based on provided fields
        // We need to handle the cases where name and/or email are provided
        let updated_user = match (request.name, request.email) {
            (Some(name), Some(email)) => {
                // Both fields provided
                sqlx::query_as::<_, User>(
                    r"
                    UPDATE users
                    SET name = $2, email = $3, updated_at = NOW()
                    WHERE id = $1
                    RETURNING id, name, email, created_at, updated_at
                    ",
                )
                .bind(id)
                .bind(&name)
                .bind(&email)
                .fetch_one(&self.pool)
                .await
            }
            (Some(name), None) => {
                // Only name provided
                sqlx::query_as::<_, User>(
                    r"
                    UPDATE users
                    SET name = $2, updated_at = NOW()
                    WHERE id = $1
                    RETURNING id, name, email, created_at, updated_at
                    ",
                )
                .bind(id)
                .bind(&name)
                .fetch_one(&self.pool)
                .await
            }
            (None, Some(email)) => {
                // Only email provided
                sqlx::query_as::<_, User>(
                    r"
                    UPDATE users
                    SET email = $2, updated_at = NOW()
                    WHERE id = $1
                    RETURNING id, name, email, created_at, updated_at
                    ",
                )
                .bind(id)
                .bind(&email)
                .fetch_one(&self.pool)
                .await
            }
            (None, None) => {
                // No fields to update, just refresh updated_at
                sqlx::query_as::<_, User>(
                    r"
                    UPDATE users
                    SET updated_at = NOW()
                    WHERE id = $1
                    RETURNING id, name, email, created_at, updated_at
                    ",
                )
                .bind(id)
                .fetch_one(&self.pool)
                .await
            }
        };

        match updated_user {
            Ok(user) => {
                tracing::info!("Updated user with id: {}", user.id);
                Ok(Some(user))
            }
            Err(e) => {
                tracing::error!("Failed to update user {}: {}", id, e);
                Err(ApiError::Database(e))
            }
        }
    }

    async fn delete_user(&self, id: i32) -> Result<bool, ApiError> {
        let result = sqlx::query(
            r"
            DELETE FROM users
            WHERE id = $1
            ",
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete user {}: {}", id, e);
            ApiError::Database(e)
        })?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            tracing::info!("Deleted user with id: {}", id);
        }

        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::test_utils::setup_test_database;

    async fn setup_test_repo() -> SqlxUserRepository {
        // Check if DATABASE_URL is set - if not, panic with a clear message
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for repository tests (see .env.test.example)");
        let pool = setup_test_database().await;
        SqlxUserRepository::new(pool)
    }

    async fn cleanup_test_user(repo: &SqlxUserRepository, id: i32) {
        let _ = repo.delete_user(id).await;
    }

    #[tokio::test]
    async fn test_create_user() {
        let repo = setup_test_repo().await;

        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: format!("test-{}@example.com", uuid::Uuid::new_v4()),
        };

        let user = repo
            .create_user(request)
            .await
            .expect("Failed to create user");

        assert!(user.id > 0);
        assert_eq!(user.name, "Test User");
        assert!(user.created_at <= user.updated_at);

        cleanup_test_user(&repo, user.id).await;
    }

    #[tokio::test]
    async fn test_get_user_existing() {
        let repo = setup_test_repo().await;

        // Create a user first
        let request = CreateUserRequest {
            name: "Get Test User".to_string(),
            email: format!("get-test-{}@example.com", uuid::Uuid::new_v4()),
        };
        let created = repo
            .create_user(request)
            .await
            .expect("Failed to create user");

        // Get the user
        let user = repo.get_user(created.id).await.expect("Failed to get user");

        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.id, created.id);
        assert_eq!(user.name, "Get Test User");

        cleanup_test_user(&repo, user.id).await;
    }

    #[tokio::test]
    async fn test_get_user_nonexistent() {
        let repo = setup_test_repo().await;

        let user = repo.get_user(999_999).await.expect("Failed to get user");
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_get_users() {
        let repo = setup_test_repo().await;

        // Create two users
        let request1 = CreateUserRequest {
            name: "User 1".to_string(),
            email: format!("user1-{}@example.com", uuid::Uuid::new_v4()),
        };
        let created_user1 = repo
            .create_user(request1)
            .await
            .expect("Failed to create user 1");

        let request2 = CreateUserRequest {
            name: "User 2".to_string(),
            email: format!("user2-{}@example.com", uuid::Uuid::new_v4()),
        };
        let created_user2 = repo
            .create_user(request2)
            .await
            .expect("Failed to create user 2");

        // Get all users
        let all_users = repo.get_users().await.expect("Failed to get users");

        assert!(all_users.len() >= 2);
        assert!(all_users.iter().any(|u| u.id == created_user1.id));
        assert!(all_users.iter().any(|u| u.id == created_user2.id));

        // Verify ordering
        let window: Vec<_> = all_users.windows(2).collect();
        for pair in window {
            assert!(pair[0].id <= pair[1].id, "Users should be ordered by ID");
        }

        cleanup_test_user(&repo, created_user1.id).await;
        cleanup_test_user(&repo, created_user2.id).await;
    }

    #[tokio::test]
    async fn test_update_user_both_fields() {
        let repo = setup_test_repo().await;

        // Create a user
        let request = CreateUserRequest {
            name: "Original Name".to_string(),
            email: format!("original-{}@example.com", uuid::Uuid::new_v4()),
        };
        let created = repo
            .create_user(request)
            .await
            .expect("Failed to create user");

        // Update both fields
        let update_request = UpdateUserRequest {
            name: Some("Updated Name".to_string()),
            email: Some(format!("updated-{}@example.com", uuid::Uuid::new_v4())),
        };

        let updated = repo
            .update_user(created.id, update_request)
            .await
            .expect("Failed to update user");

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert!(updated.updated_at > created.updated_at);

        cleanup_test_user(&repo, updated.id).await;
    }

    #[tokio::test]
    async fn test_update_user_name_only() {
        let repo = setup_test_repo().await;

        // Create a user
        let request = CreateUserRequest {
            name: "Original Name".to_string(),
            email: format!("original-{}@example.com", uuid::Uuid::new_v4()),
        };
        let created = repo
            .create_user(request)
            .await
            .expect("Failed to create user");
        let original_email = created.email.clone();

        // Update only name
        let update_request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: None,
        };

        let updated = repo
            .update_user(created.id, update_request)
            .await
            .expect("Failed to update user");

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.name, "New Name");
        assert_eq!(updated.email, original_email);
        assert!(updated.updated_at > created.updated_at);

        cleanup_test_user(&repo, updated.id).await;
    }

    #[tokio::test]
    async fn test_update_user_email_only() {
        let repo = setup_test_repo().await;

        // Create a user
        let request = CreateUserRequest {
            name: "Test Name".to_string(),
            email: format!("original-{}@example.com", uuid::Uuid::new_v4()),
        };
        let created = repo
            .create_user(request)
            .await
            .expect("Failed to create user");

        // Update only email
        let update_request = UpdateUserRequest {
            name: None,
            email: Some(format!("new-{}@example.com", uuid::Uuid::new_v4())),
        };

        let updated = repo
            .update_user(created.id, update_request)
            .await
            .expect("Failed to update user");

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.name, "Test Name");
        assert!(updated.updated_at > created.updated_at);

        cleanup_test_user(&repo, updated.id).await;
    }

    #[tokio::test]
    async fn test_update_user_nonexistent() {
        let repo = setup_test_repo().await;

        let update_request = UpdateUserRequest {
            name: Some("New Name".to_string()),
            email: None,
        };

        let result = repo
            .update_user(999_999, update_request)
            .await
            .expect("Failed to update user");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_delete_user_existing() {
        let repo = setup_test_repo().await;

        // Create a user
        let request = CreateUserRequest {
            name: "Delete Test".to_string(),
            email: format!("delete-{}@example.com", uuid::Uuid::new_v4()),
        };
        let created = repo
            .create_user(request)
            .await
            .expect("Failed to create user");

        // Delete the user
        let deleted = repo
            .delete_user(created.id)
            .await
            .expect("Failed to delete user");
        assert!(deleted);

        // Verify user is deleted
        let user = repo.get_user(created.id).await.expect("Failed to get user");
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_delete_user_nonexistent() {
        let repo = setup_test_repo().await;

        let deleted = repo
            .delete_user(999_999)
            .await
            .expect("Failed to delete user");
        assert!(!deleted);
    }
}
