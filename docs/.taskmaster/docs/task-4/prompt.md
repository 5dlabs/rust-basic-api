# Implement User Repository with SQLx

You are tasked with implementing a comprehensive repository layer for user data management in a Rust API using SQLx and PostgreSQL. This implementation should follow the repository pattern to provide a clean abstraction between business logic and database operations.

## Context
You are working on a Rust-based REST API that requires a robust data access layer. The database setup and core models have already been implemented (Tasks 2 and 3). Your focus is on creating a type-safe, efficient, and testable repository implementation.

## Requirements

### 1. Repository Trait Definition
Create a trait-based abstraction in `src/repository/user_repository.rs` that defines the following operations:
- Create a new user
- Retrieve a user by ID
- List all users
- Update user information (partial updates supported)
- Delete a user

### 2. SQLx Implementation
Implement the repository trait using SQLx with PostgreSQL:
- Use compile-time SQL verification via SQLx macros
- Handle all database errors appropriately
- Support dynamic query building for partial updates
- Ensure proper connection pooling with PgPool

### 3. Error Handling
- Map SQLx errors to application-specific errors
- Provide meaningful error messages
- Handle edge cases (missing records, constraint violations)

### 4. Testing
Create comprehensive integration tests that:
- Test all CRUD operations
- Verify transaction boundaries
- Handle concurrent operations
- Test error scenarios

## Technical Specifications

### Dependencies to Add
```toml
async-trait = "0.1"
```

### File Structure
```
src/repository/
├── mod.rs              # Module exports
├── user_repository.rs  # Trait and implementation
└── test_utils.rs      # Test utilities (if needed)
```

### Key Implementation Points

1. **UserRepository Trait**:
   - Use async-trait for async methods in traits
   - Return Result types for error handling
   - Use Option for nullable returns

2. **SqlxUserRepository**:
   - Store PgPool for connection management
   - Use query_as! macro for type safety
   - Implement RETURNING clauses for immediate feedback

3. **Dynamic Updates**:
   - Build UPDATE queries dynamically
   - Only update provided fields
   - Maintain updated_at timestamp

4. **Error Mapping**:
   - Convert sqlx::Error to ApiError::Database
   - Preserve error context for debugging
   - Handle specific database constraints

## Implementation Steps

1. **Create the repository trait** with all required methods
2. **Implement SqlxUserRepository** with proper error handling
3. **Add module exports** to make the repository accessible
4. **Write integration tests** using SQLx test utilities
5. **Verify compile-time SQL checking** works correctly
6. **Test transaction behavior** to ensure proper isolation

## Validation Criteria

Your implementation should:
- ✅ Compile without warnings
- ✅ Pass all integration tests
- ✅ Handle database errors gracefully
- ✅ Support concurrent operations safely
- ✅ Use prepared statements for security
- ✅ Follow Rust idioms and best practices

## Code Quality Requirements

- Use descriptive variable names
- Add documentation comments for public APIs
- Follow consistent error handling patterns
- Implement proper logging for debugging
- Ensure code is formatted with rustfmt

## Example Usage

```rust
let pool = create_pool(&database_url).await?;
let repo = SqlxUserRepository::new(pool);

// Create a user
let user = repo.create_user(CreateUserRequest {
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
}).await?;

// Update user
let updated = repo.update_user(user.id, UpdateUserRequest {
    name: Some("Jane Doe".to_string()),
    email: None,
}).await?;

// Delete user
let deleted = repo.delete_user(user.id).await?;
```

## Notes
- Ensure all SQL queries are parameterized to prevent injection
- Use transactions where appropriate for data consistency
- Consider implementing query builders for complex filters in the future
- Monitor query performance and add indexes as needed

Begin by creating the repository trait, then implement the SQLx-based repository with all required methods. Ensure comprehensive error handling and write thorough tests to validate the implementation.