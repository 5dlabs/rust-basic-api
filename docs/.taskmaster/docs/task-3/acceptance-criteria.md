# Acceptance Criteria: Data Models and Error Handling

## Required Deliverables

### 1. Dependency Configuration ✓
- [ ] `Cargo.toml` updated with validator dependency:
  - [ ] validator = { version = "0.16", features = ["derive"] }
  - [ ] All existing dependencies remain functional
  - [ ] Project builds successfully with new dependency

### 2. User Data Model Implementation ✓
- [ ] `src/models/user.rs` created with complete User struct:
  - [ ] `id: i32` field for primary key
  - [ ] `name: String` field for user name
  - [ ] `email: String` field for user email
  - [ ] `created_at: DateTime<Utc>` for creation timestamp
  - [ ] `updated_at: DateTime<Utc>` for last update timestamp
  - [ ] Proper serde Serialize and Deserialize derives
  - [ ] Clone and PartialEq derives for testing

- [ ] `CreateUserRequest` struct implemented:
  - [ ] `name: String` field with length validation (1-255 characters)
  - [ ] `email: String` field with email format validation
  - [ ] Validator derive macro with appropriate attributes
  - [ ] Custom validation error messages

- [ ] `UpdateUserRequest` struct implemented:
  - [ ] `name: Option<String>` field with conditional validation
  - [ ] `email: Option<String>` field with conditional validation
  - [ ] Proper handling of optional fields during validation

### 3. Error Handling System ✓
- [ ] `src/error.rs` created with comprehensive error handling:
  - [ ] `ApiError` enum with all required variants:
    - [ ] `Validation(String)` for validation errors
    - [ ] `Database(#[from] sqlx::Error)` for database errors
    - [ ] `NotFound` for resource not found errors
    - [ ] `InternalServerError` for internal errors
  - [ ] `ErrorResponse` struct with error and message fields
  - [ ] `IntoResponse` implementation for `ApiError`
  - [ ] Proper HTTP status code mapping
  - [ ] Database error logging without exposure
  - [ ] `Result<T>` type alias defined

### 4. Validation Utilities ✓
- [ ] `src/models/validation.rs` created:
  - [ ] `validate_request<T: Validate>` function implemented
  - [ ] Proper conversion from ValidationErrors to ApiError
  - [ ] Error message formatting for multiple validation failures
  - [ ] Field-level error message extraction

### 5. Module Organization ✓
- [ ] `src/models/mod.rs` updated:
  - [ ] Export user module and types
  - [ ] Export validation utilities
  - [ ] Clean public API for other modules

## Test Cases

### Test Case 1: User Model Serialization
```rust
#[test]
fn test_user_serialization() {
    let user = User {
        id: 1,
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Expected: Successful serialization and deserialization
    let json = serde_json::to_string(&user).unwrap();
    let deserialized: User = serde_json::from_str(&json).unwrap();
    
    assert_eq!(user.id, deserialized.id);
    assert_eq!(user.name, deserialized.name);
    assert_eq!(user.email, deserialized.email);
}
```

### Test Case 2: Valid Request Validation
```rust
#[test]
fn test_valid_create_user_request() {
    let request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    };
    
    // Expected: Validation passes
    assert!(validate_request(&request).is_ok());
}
```

### Test Case 3: Invalid Email Validation
```rust
#[test]
fn test_invalid_email_validation() {
    let request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "invalid-email".to_string(),
    };
    
    // Expected: Validation fails with email format error
    let result = validate_request(&request);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        ApiError::Validation(msg) => {
            assert!(msg.contains("Invalid email format"));
        },
        _ => panic!("Expected validation error"),
    }
}
```

### Test Case 4: Empty Name Validation
```rust
#[test]
fn test_empty_name_validation() {
    let request = CreateUserRequest {
        name: "".to_string(),
        email: "test@example.com".to_string(),
    };
    
    // Expected: Validation fails with length error
    let result = validate_request(&request);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        ApiError::Validation(msg) => {
            assert!(msg.contains("Name must be between 1 and 255 characters"));
        },
        _ => panic!("Expected validation error"),
    }
}
```

### Test Case 5: Long Name Validation
```rust
#[test]
fn test_long_name_validation() {
    let long_name = "a".repeat(256);
    let request = CreateUserRequest {
        name: long_name,
        email: "test@example.com".to_string(),
    };
    
    // Expected: Validation fails with length error
    assert!(validate_request(&request).is_err());
}
```

### Test Case 6: Update Request Validation
```rust
#[test]
fn test_update_request_validation() {
    // Valid partial update
    let valid_update = UpdateUserRequest {
        name: Some("Updated Name".to_string()),
        email: None,
    };
    assert!(validate_request(&valid_update).is_ok());
    
    // Empty optional fields should be valid
    let empty_update = UpdateUserRequest {
        name: None,
        email: None,
    };
    assert!(validate_request(&empty_update).is_ok());
    
    // Invalid email in update should fail
    let invalid_update = UpdateUserRequest {
        name: None,
        email: Some("invalid-email".to_string()),
    };
    assert!(validate_request(&invalid_update).is_err());
}
```

### Test Case 7: Error Response HTTP Mapping
```rust
#[test]
fn test_error_response_mapping() {
    // Validation error -> 400 Bad Request
    let validation_error = ApiError::Validation("Test error".to_string());
    let response = validation_error.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    
    // Not found error -> 404 Not Found
    let not_found_error = ApiError::NotFound;
    let response = not_found_error.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    
    // Internal server error -> 500 Internal Server Error
    let internal_error = ApiError::InternalServerError;
    let response = internal_error.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
```

### Test Case 8: Error Response JSON Structure
```rust
#[test]
fn test_error_response_json_structure() {
    let error = ApiError::Validation("Test validation error".to_string());
    let response = error.into_response();
    
    // Extract body and parse JSON
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error_response: ErrorResponse = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(error_response.error, "VALIDATION_ERROR");
    assert_eq!(error_response.message, "Test validation error");
}
```

## Performance Criteria
- [ ] Validation errors create minimal heap allocations
- [ ] Error response generation completes in < 1ms
- [ ] JSON serialization/deserialization performs within 10% of baseline
- [ ] Memory usage remains constant during validation operations

## Security Checklist
- [ ] Database error details are never exposed in HTTP responses
- [ ] Validation error messages don't reveal internal system information
- [ ] Email validation prevents obvious injection attempts
- [ ] Length limits prevent potential DoS attacks via large inputs
- [ ] Error logging includes appropriate context without sensitive data

## Code Quality Checklist
- [ ] All code compiles without warnings (`cargo build`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code is properly formatted (`cargo fmt`)
- [ ] All public APIs have appropriate documentation
- [ ] Error messages are helpful and actionable
- [ ] Validation logic is comprehensive but not overly restrictive

## Integration Requirements
- [ ] Models work correctly with serde JSON serialization
- [ ] Error types integrate seamlessly with Axum response system
- [ ] Validation utilities can be used throughout the application
- [ ] Module structure supports future expansion
- [ ] Error handling patterns are consistent and reusable

## Definition of Done
- [ ] All test cases pass
- [ ] Code coverage > 80% for new modules
- [ ] Documentation covers all public APIs
- [ ] Error handling covers all identified error scenarios
- [ ] Validation rules match business requirements
- [ ] Performance criteria are met
- [ ] Security checklist is complete
- [ ] Integration with existing codebase is seamless