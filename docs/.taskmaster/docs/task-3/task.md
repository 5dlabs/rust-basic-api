# Task 3: Data Models and Error Handling

## Overview
Implement a comprehensive data modeling system and error handling framework for the REST API. This task establishes the User model with proper validation, serialization, and a robust error handling system using Rust's type system and Axum's response traits.

## Technical Requirements

### Dependencies
This task extends the foundation from Task 1 and requires additional crates:
- **validator** (0.16+): Request validation with derive macros
- **chrono**: DateTime handling (already included via sqlx)
- **utoipa**: OpenAPI schema generation (preparation for future tasks)

### Data Models
The User model serves as the primary entity with full CRUD operation support:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Request/Response Schemas
Separate data transfer objects for API operations:
- **CreateUserRequest**: Validates new user data
- **UpdateUserRequest**: Handles partial updates with optional fields

### Error Handling Framework
Comprehensive error system using `thiserror` and Axum's `IntoResponse`:
- Type-safe error variants with automatic HTTP status mapping
- Structured JSON error responses
- Database error abstraction
- Request validation error handling

## Implementation Steps

### Step 1: Update Dependencies
Add the validator crate to `Cargo.toml`:

```toml
[dependencies]
validator = { version = "0.16", features = ["derive"] }
```

### Step 2: Implement User Model
Create `src/models/user.rs` with the complete User model:

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}
```

### Step 3: Implement Error Handling System
Create `src/error.rs` with comprehensive error handling:

```rust
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Resource not found")]
    NotFound,
    
    #[error("Internal server error")]
    InternalServerError,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            ApiError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "VALIDATION_ERROR".to_string(),
                    message: msg,
                },
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: "NOT_FOUND".to_string(),
                    message: "Resource not found".to_string(),
                },
            ),
            ApiError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        error: "DATABASE_ERROR".to_string(),
                        message: "A database error occurred".to_string(),
                    },
                )
            },
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "INTERNAL_ERROR".to_string(),
                    message: "An internal error occurred".to_string(),
                },
            ),
        };

        (status, Json(error_response)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;
```

### Step 4: Create Validation Utilities
Create `src/models/validation.rs` for request validation:

```rust
use validator::{Validate, ValidationErrors};
use crate::error::ApiError;

pub fn validate_request<T: Validate>(request: &T) -> crate::error::Result<()> {
    request.validate().map_err(|e: ValidationErrors| {
        let message = format_validation_errors(&e);
        ApiError::Validation(message)
    })
}

fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut messages = Vec::new();
    
    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            let message = error.message.as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| format!("Invalid value for field: {}", field));
            messages.push(message);
        }
    }
    
    messages.join(", ")
}
```

### Step 5: Update Module Exports
Update `src/models/mod.rs` to export all models:

```rust
pub mod user;
pub mod validation;

pub use user::{User, CreateUserRequest, UpdateUserRequest};
pub use validation::validate_request;
```

## Testing Strategy

### Unit Tests
Create comprehensive unit tests in `src/models/user.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::validation::validate_request;

    #[test]
    fn test_user_serialization() {
        let user = User {
            id: 1,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();
        
        assert_eq!(user.id, deserialized.id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.email, deserialized.email);
    }

    #[test]
    fn test_create_user_validation() {
        // Valid request
        let valid_req = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
        assert!(validate_request(&valid_req).is_ok());
        
        // Invalid email
        let invalid_email = CreateUserRequest {
            name: "Test User".to_string(),
            email: "invalid-email".to_string(),
        };
        assert!(validate_request(&invalid_email).is_err());
        
        // Empty name
        let empty_name = CreateUserRequest {
            name: "".to_string(),
            email: "test@example.com".to_string(),
        };
        assert!(validate_request(&empty_name).is_err());
    }

    #[test]
    fn test_update_user_validation() {
        // Valid partial update
        let valid_update = UpdateUserRequest {
            name: Some("Updated Name".to_string()),
            email: None,
        };
        assert!(validate_request(&valid_update).is_ok());
        
        // Invalid email in update
        let invalid_update = UpdateUserRequest {
            name: None,
            email: Some("invalid-email".to_string()),
        };
        assert!(validate_request(&invalid_update).is_err());
    }
}
```

### Error Response Tests
Test error handling in `src/error.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_validation_error_response() {
        let error = ApiError::Validation("Invalid email format".to_string());
        let response = error.into_response();
        
        // Response status should be 400 Bad Request
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_not_found_error_response() {
        let error = ApiError::NotFound;
        let response = error.into_response();
        
        // Response status should be 404 Not Found
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_internal_server_error_response() {
        let error = ApiError::InternalServerError;
        let response = error.into_response();
        
        // Response status should be 500 Internal Server Error
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
```

## Architecture Patterns

### Error Handling Design
- **Type Safety**: All errors are strongly typed with explicit variants
- **HTTP Mapping**: Automatic conversion from error types to HTTP status codes
- **Logging**: Database errors are logged but not exposed to clients
- **Client Feedback**: Validation errors include helpful messages

### Validation Strategy
- **Input Validation**: All incoming requests are validated before processing
- **Message Customization**: Custom validation messages for better UX
- **Field-Level Errors**: Detailed feedback on which fields have issues
- **Optional Fields**: Update requests support partial data

## Dependencies for Next Tasks
This task establishes patterns that subsequent tasks will build upon:
- **Task 4**: Will use the User model and error types in repository implementation
- **Task 5**: Will use validation utilities and error handling in route handlers
- **Task 6**: Will extend testing patterns established here
- **Task 8**: Will add OpenAPI annotations to these models

## Performance Considerations
- **Minimal Allocations**: Error handling avoids unnecessary string allocations
- **Efficient Serialization**: Models use serde's efficient JSON serialization
- **Validation Caching**: Validation rules are compile-time generated
- **Database Error Abstraction**: Prevents database-specific error details from leaking

## Security Considerations
- **Input Sanitization**: All user inputs are validated before processing
- **Error Information Disclosure**: Database errors are logged but not exposed
- **Email Validation**: Proper email format validation prevents injection
- **Length Limits**: Name field has reasonable length constraints