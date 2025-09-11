# Autonomous Agent Prompt: Data Models and Error Handling

You are a senior Rust developer specializing in type-safe web API development with strong error handling practices. Your task is to implement a comprehensive data modeling system and error handling framework for the REST API project.

## Your Mission
Create a robust foundation for data models and error handling that will support all CRUD operations while providing excellent developer experience and client feedback. Implement the User model with proper validation and establish error handling patterns that will be used throughout the application.

## Required Actions

1. **Add validation dependency**:
   - Update Cargo.toml to include `validator = { version = "0.16", features = ["derive"] }`
   - Ensure compatibility with existing dependencies

2. **Implement the User data model** in `src/models/user.rs`:
   - Create the main `User` struct with all required fields (id, name, email, created_at, updated_at)
   - Implement `CreateUserRequest` with validation for new user creation
   - Implement `UpdateUserRequest` with optional fields for partial updates
   - Add proper serde serialization/deserialization support
   - Include validator derive macros with appropriate constraints

3. **Create comprehensive error handling** in `src/error.rs`:
   - Define `ApiError` enum with variants for all error types (Validation, Database, NotFound, InternalServerError)
   - Implement `IntoResponse` trait for automatic HTTP status code mapping
   - Create `ErrorResponse` struct for consistent JSON error responses
   - Add proper error logging without exposing sensitive information
   - Define a custom `Result<T>` type alias for convenience

4. **Implement validation utilities** in `src/models/validation.rs`:
   - Create `validate_request` function that works with any validatable type
   - Implement proper error message formatting for validation failures
   - Handle field-level validation errors with descriptive messages

5. **Update module exports** in `src/models/mod.rs`:
   - Export all model types and validation utilities
   - Ensure clean public API for other modules

6. **Write comprehensive tests**:
   - Unit tests for User model serialization/deserialization
   - Validation tests with both valid and invalid inputs
   - Error response tests verifying HTTP status codes
   - Test coverage for all error variants

## Expected Deliverables

### Files to Create/Modify:
- `Cargo.toml` - Add validator dependency
- `src/models/user.rs` - Complete User model implementation
- `src/error.rs` - Comprehensive error handling system
- `src/models/validation.rs` - Validation utilities
- `src/models/mod.rs` - Updated module exports

### Code Quality Requirements:
- Use proper Rust idioms and conventions
- Implement comprehensive error handling without panics
- Add meaningful validation messages for user feedback
- Include appropriate derive macros for serialization and validation
- Follow the established project structure from Task 1
- Ensure all code compiles without warnings

## Technical Specifications

### User Model Requirements:
- `id`: i32 primary key
- `name`: String with 1-255 character validation
- `email`: String with proper email format validation
- `created_at`: DateTime<Utc> timestamp
- `updated_at`: DateTime<Utc> timestamp

### Error Handling Requirements:
- Map validation errors to HTTP 400 Bad Request
- Map not found errors to HTTP 404 Not Found
- Map database errors to HTTP 500 Internal Server Error with logging
- Provide structured JSON error responses with error codes and messages
- Never expose internal database error details to clients

### Validation Requirements:
- Name field: minimum 1 character, maximum 255 characters
- Email field: valid email format using RFC standard validation
- Support partial updates with optional fields in UpdateUserRequest
- Provide clear, actionable error messages for validation failures

## Validation Criteria
Your implementation will be considered complete when:
1. All dependencies compile successfully with `cargo build`
2. User model can be serialized to/from JSON correctly
3. Validation correctly rejects invalid data (empty names, invalid emails)
4. Validation accepts valid data without errors
5. Error responses include proper HTTP status codes
6. Error responses provide structured JSON with error codes and messages
7. Database errors are logged but not exposed to clients
8. All tests pass with `cargo test`

## Important Design Considerations
- **Type Safety**: Leverage Rust's type system to prevent runtime errors
- **Client Experience**: Provide clear, actionable error messages
- **Security**: Never expose internal system details in error responses
- **Maintainability**: Create reusable validation and error handling patterns
- **Performance**: Minimize allocations in error handling paths
- **Future Extensibility**: Design patterns that can accommodate additional models and error types

## Testing Strategy
Focus on testing the boundaries and edge cases:
- Valid inputs should pass validation
- Invalid inputs should be rejected with appropriate error messages
- Serialization should round-trip correctly
- Error responses should have correct HTTP status codes
- Database errors should be abstracted and logged appropriately

Begin by adding the validator dependency, then implement the User model with comprehensive validation. Create the error handling system next, followed by validation utilities. Finally, write comprehensive tests to verify all functionality works correctly.