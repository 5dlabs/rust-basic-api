# Task 5: API Route Handlers Implementation

## Overview

Implement comprehensive API route handlers for all user endpoints using Axum framework, providing a clean RESTful interface for user management operations.

## Dependencies

- **Task 4**: User Repository Implementation (must be completed first)
- **Task 3**: User Models and Validation
- **Task 2**: Database Setup

## Technical Requirements

### Core Components

1. **Route Handlers** (`src/routes/user_routes.rs`)
   - GET /users - List all users
   - GET /users/:id - Get specific user
   - POST /users - Create new user
   - PUT /users/:id - Update existing user
   - DELETE /users/:id - Delete user

2. **Health Check Endpoint**
   - GET /health - Simple health check returning "OK"

3. **Middleware Integration**
   - Request tracing with tower-http
   - Proper state management with PgPool

## Implementation Guide

### Step 1: Create Route Handler Module

```rust
// src/routes/user_routes.rs
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

### Step 2: Implement CRUD Handlers

#### List Users Handler
```rust
pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>> {
    let repo = SqlxUserRepository::new(pool);
    let users = repo.get_users().await?;
    Ok(Json(users))
}
```

#### Get Single User Handler
```rust
pub async fn get_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<User>> {
    let repo = SqlxUserRepository::new(pool);
    let user = repo.get_user(id).await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(user))
}
```

#### Create User Handler
```rust
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>)> {
    validate_request(&req)?;
    
    let repo = SqlxUserRepository::new(pool);
    let user = repo.create_user(req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
```

#### Update User Handler
```rust
pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<User>> {
    validate_request(&req)?;
    
    let repo = SqlxUserRepository::new(pool);
    let user = repo.update_user(id, req).await?
        .ok_or(ApiError::NotFound)?;
    Ok(Json(user))
}
```

#### Delete User Handler
```rust
pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    let repo = SqlxUserRepository::new(pool);
    let deleted = repo.delete_user(id).await?;
    
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}
```

### Step 3: Create Module Exports

```rust
// src/routes/mod.rs
mod user_routes;

pub use user_routes::*;

// Health check route
pub async fn health_check() -> &'static str {
    "OK"
}
```

### Step 4: Configure Router in Main

```rust
// In main.rs
use tower_http::trace::TraceLayer;

let app = Router::new()
    .route("/health", axum::routing::get(routes::health_check))
    .route("/users", axum::routing::get(routes::get_users))
    .route("/users", axum::routing::post(routes::create_user))
    .route("/users/:id", axum::routing::get(routes::get_user))
    .route("/users/:id", axum::routing::put(routes::update_user))
    .route("/users/:id", axum::routing::delete(routes::delete_user))
    .layer(TraceLayer::new_for_http())
    .with_state(pool);
```

### Step 5: Add Dependencies

```toml
# Cargo.toml
tower-http = { version = "0.4", features = ["trace"] }
```

## Error Handling

### Status Code Mapping
- **200 OK**: Successful GET requests
- **201 Created**: Successful POST requests
- **204 No Content**: Successful DELETE requests
- **400 Bad Request**: Validation errors
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Database or server errors

### Error Response Format
```json
{
    "error": "ERROR_CODE",
    "message": "Human-readable error message"
}
```

## Request/Response Examples

### Create User
**Request:**
```http
POST /users
Content-Type: application/json

{
    "name": "John Doe",
    "email": "john@example.com"
}
```

**Response:**
```http
HTTP/1.1 201 Created
Content-Type: application/json

{
    "id": 1,
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
}
```

### Update User
**Request:**
```http
PUT /users/1
Content-Type: application/json

{
    "name": "Jane Doe"
}
```

**Response:**
```http
HTTP/1.1 200 OK
Content-Type: application/json

{
    "id": 1,
    "name": "Jane Doe",
    "email": "john@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:01:00Z"
}
```

## Testing Approach

### Integration Tests

1. **Setup Test Router**
   - Create test database connection
   - Configure router with test state
   - Use tower::ServiceExt for testing

2. **Test Coverage**
   - All CRUD operations
   - Validation error handling
   - Not found scenarios
   - Concurrent request handling

3. **Test Utilities**
   ```rust
   async fn setup_router() -> Router {
       let pool = setup_test_database().await;
       Router::new()
           // ... route configuration
           .with_state(pool)
   }
   ```

## Performance Considerations

1. **Connection Pooling**
   - Repository instances created per request
   - Shared PgPool via Axum state

2. **Request Tracing**
   - Tower-http middleware for observability
   - Minimal performance overhead

3. **Response Serialization**
   - Efficient JSON serialization with serde
   - Proper content-type headers

## Security Notes

1. **Input Validation**
   - All requests validated before processing
   - Email format validation
   - Required field checking

2. **SQL Injection Prevention**
   - Parameterized queries via repository layer
   - No direct SQL construction in handlers

3. **Error Information**
   - Generic error messages for clients
   - Detailed errors in server logs only

## Future Enhancements

- Pagination for GET /users
- Query filters and sorting
- Request rate limiting
- Authentication middleware
- API versioning
- OpenAPI documentation generation