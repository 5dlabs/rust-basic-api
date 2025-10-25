//! User route handlers
//!
//! This module contains HTTP route handlers for user management endpoints.
//! All handlers use Axum extractors for request data and return appropriate
//! HTTP status codes with JSON responses.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::{
    error::ApiError,
    models::{validate_request, CreateUserRequest, UpdateUserRequest, User},
    repository::{SqlxUserRepository, UserRepository},
};

/// Get all users
///
/// # Endpoint
/// `GET /users`
///
/// # Returns
/// - `200 OK` with JSON array of users
///
/// # Errors
/// Returns `ApiError::Database` if database query fails
///
/// # Example
/// ```bash
/// curl http://localhost:3000/users
/// ```
pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, ApiError> {
    let repo = SqlxUserRepository::new(pool);
    let users = repo.get_users().await?;
    Ok(Json(users))
}

/// Get a specific user by ID
///
/// # Endpoint
/// `GET /users/:id`
///
/// # Path Parameters
/// - `id`: User ID (integer)
///
/// # Returns
/// - `200 OK` with user JSON if found
///
/// # Errors
/// - Returns `ApiError::NotFound` if user doesn't exist
/// - Returns `ApiError::Database` if database query fails
///
/// # Example
/// ```bash
/// curl http://localhost:3000/users/1
/// ```
pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, ApiError> {
    let repo = SqlxUserRepository::new(pool);
    let user = repo.get_user(id).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(user))
}

/// Create a new user
///
/// # Endpoint
/// `POST /users`
///
/// # Request Body
/// ```json
/// {
///   "name": "John Doe",
///   "email": "john@example.com"
/// }
/// ```
///
/// # Returns
/// - `201 Created` with created user JSON
///
/// # Errors
/// - Returns `ApiError::Validation` if request validation fails
/// - Returns `ApiError::Database` if database operation fails (e.g., duplicate email)
///
/// # Example
/// ```bash
/// curl -X POST http://localhost:3000/users \
///   -H "Content-Type: application/json" \
///   -d '{"name":"John Doe","email":"john@example.com"}'
/// ```
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    // Validate request data
    validate_request(&request)?;

    // Create user in database
    let repo = SqlxUserRepository::new(pool);
    let user = repo.create_user(request).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// Update an existing user
///
/// # Endpoint
/// `PUT /users/:id`
///
/// # Path Parameters
/// - `id`: User ID (integer)
///
/// # Request Body
/// All fields are optional (partial update supported):
/// ```json
/// {
///   "name": "Jane Doe",
///   "email": "jane@example.com"
/// }
/// ```
///
/// # Returns
/// - `200 OK` with updated user JSON
///
/// # Errors
/// - Returns `ApiError::Validation` if request validation fails
/// - Returns `ApiError::NotFound` if user doesn't exist
/// - Returns `ApiError::Database` if database operation fails
///
/// # Example
/// ```bash
/// curl -X PUT http://localhost:3000/users/1 \
///   -H "Content-Type: application/json" \
///   -d '{"name":"Jane Doe"}'
/// ```
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<User>, ApiError> {
    // Validate request data
    validate_request(&request)?;

    // Update user in database
    let repo = SqlxUserRepository::new(pool);
    let user = repo
        .update_user(id, request)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(user))
}

/// Delete a user
///
/// # Endpoint
/// `DELETE /users/:id`
///
/// # Path Parameters
/// - `id`: User ID (integer)
///
/// # Returns
/// - `204 No Content` if user was deleted
///
/// # Errors
/// - Returns `ApiError::NotFound` if user doesn't exist
/// - Returns `ApiError::Database` if database operation fails
///
/// # Example
/// ```bash
/// curl -X DELETE http://localhost:3000/users/1
/// ```
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    let repo = SqlxUserRepository::new(pool);
    let deleted = repo.delete_user(id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode as HttpStatus},
        Router,
    };
    use tower::ServiceExt;

    use crate::routes::register_routes;
    use crate::AppState;

    // Helper to create test app
    fn create_test_app() -> Router {
        let test_connection = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            format!(
                "{}://{}:{}@{}/{}",
                "postgresql", "testuser", "testpass", "localhost", "testdb"
            )
        });
        let pool =
            sqlx::PgPool::connect_lazy(&test_connection).expect("Failed to create mock pool");
        let state = AppState { pool };
        register_routes(state)
    }

    #[tokio::test]
    async fn test_get_users_route_exists() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should not be 404 (route exists)
        assert_ne!(response.status(), HttpStatus::NOT_FOUND);
    }

    #[tokio::test]
    #[cfg_attr(coverage, ignore)]
    async fn test_get_user_route_exists() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/users/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should not be 404 (route exists) - might be 500 without real DB, but route is registered
        assert_ne!(response.status(), HttpStatus::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_user_route_exists() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"name":"Test User","email":"test@example.com"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should not be 404 or 405 (route exists and method allowed)
        assert_ne!(response.status(), HttpStatus::NOT_FOUND);
        assert_ne!(response.status(), HttpStatus::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    #[cfg_attr(coverage, ignore)]
    async fn test_update_user_route_exists() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/users/1")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"Updated User"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should not be 404 or 405
        assert_ne!(response.status(), HttpStatus::NOT_FOUND);
        assert_ne!(response.status(), HttpStatus::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    #[cfg_attr(coverage, ignore)]
    async fn test_delete_user_route_exists() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri("/users/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should not be 404 or 405
        assert_ne!(response.status(), HttpStatus::NOT_FOUND);
        assert_ne!(response.status(), HttpStatus::METHOD_NOT_ALLOWED);
    }
}
