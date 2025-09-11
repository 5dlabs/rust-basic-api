# Task 5: API Route Handlers Implementation - Agent Prompt

You are a senior Rust backend developer tasked with implementing API route handlers for a user management system using the Axum web framework.

## Your Mission

Implement comprehensive RESTful API route handlers that bridge the gap between HTTP requests and the repository layer, providing a clean interface for user CRUD operations.

## Context

The project already has:
- Database setup with SQLx and PostgreSQL (Task 2)
- User models with validation (Task 3)
- Repository pattern implementation (Task 4)

You need to create the HTTP layer that exposes these capabilities through RESTful endpoints.

## Implementation Requirements

### 1. Route Handlers Module (`src/routes/user_routes.rs`)

Create five handler functions:
- `get_users`: List all users
- `get_user`: Get a specific user by ID
- `create_user`: Create a new user
- `update_user`: Update an existing user
- `delete_user`: Remove a user

Each handler should:
- Extract necessary data from the request (path parameters, JSON body, state)
- Validate input data using the existing validation module
- Call the appropriate repository method
- Return appropriate HTTP status codes and JSON responses
- Handle errors gracefully with proper error mapping

### 2. Module Organization (`src/routes/mod.rs`)

- Export all user route handlers
- Implement a simple health check endpoint returning "OK"

### 3. Router Configuration

In the main application setup:
- Configure all routes with appropriate HTTP methods
- Add request tracing middleware using tower-http
- Inject the database pool as shared state

### 4. Error Handling

Ensure proper HTTP status codes:
- 200 OK for successful GET requests
- 201 Created for successful POST requests
- 204 No Content for successful DELETE requests
- 400 Bad Request for validation errors
- 404 Not Found when resources don't exist
- 500 Internal Server Error for unexpected failures

## Technical Specifications

### Dependencies Required
```toml
tower-http = { version = "0.4", features = ["trace"] }
```

### Key Imports
```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::models::{User, CreateUserRequest, UpdateUserRequest, validation::validate_request};
use crate::repository::{UserRepository, SqlxUserRepository};
use crate::error::{ApiError, Result};
```

### Handler Signatures

1. **List Users**
   ```rust
   pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>>
   ```

2. **Get User**
   ```rust
   pub async fn get_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<User>>
   ```

3. **Create User**
   ```rust
   pub async fn create_user(State(pool): State<PgPool>, Json(req): Json<CreateUserRequest>) 
       -> Result<(StatusCode, Json<User>)>
   ```

4. **Update User**
   ```rust
   pub async fn update_user(State(pool): State<PgPool>, Path(id): Path<i32>, 
       Json(req): Json<UpdateUserRequest>) -> Result<Json<User>>
   ```

5. **Delete User**
   ```rust
   pub async fn delete_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<StatusCode>
   ```

## Implementation Guidelines

1. **Repository Usage**
   - Create a new `SqlxUserRepository` instance for each request
   - Pass the shared `PgPool` from the state
   - Handle Option returns appropriately (convert None to 404)

2. **Validation**
   - Call `validate_request` before processing create/update operations
   - Let validation errors propagate through the error handling chain

3. **Response Format**
   - Always return JSON for data responses
   - Include appropriate status codes in responses
   - Return empty body with 204 for successful deletions

4. **Error Mapping**
   - Convert missing resources to `ApiError::NotFound`
   - Let other errors bubble up through the Result type
   - Ensure error responses follow the project's error format

## Testing Requirements

Implement comprehensive integration tests that:
1. Test all CRUD operations end-to-end
2. Verify correct status codes for all scenarios
3. Test validation error handling
4. Test not-found scenarios
5. Use a test database for isolation
6. Utilize tower::ServiceExt for testing the router

## Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;
    
    async fn setup_router() -> Router {
        let pool = setup_test_database().await;
        Router::new()
            .route("/users", get(get_users).post(create_user))
            .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
            .with_state(pool)
    }
    
    #[tokio::test]
    async fn test_create_user_endpoint() {
        let app = setup_router().await;
        // Test implementation
    }
}
```

## Success Criteria

Your implementation is complete when:
1. All five CRUD endpoints are functional
2. Health check endpoint returns "OK"
3. Request validation is enforced
4. Proper HTTP status codes are returned
5. All integration tests pass
6. Request tracing is enabled
7. Code compiles without warnings

Remember to follow Rust best practices, use idiomatic Axum patterns, and ensure the code is production-ready with proper error handling and logging.