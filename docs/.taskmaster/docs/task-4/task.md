# Task 4: User Repository Implementation

## Overview
Implement a robust repository pattern for user data operations using SQLx and PostgreSQL. This task establishes the data access layer for the user management system, providing a clean abstraction between the business logic and database operations.

## Technical Context

### Repository Pattern Benefits
- **Separation of Concerns**: Isolates database logic from business logic
- **Testability**: Enables easy mocking and testing with trait-based design
- **Flexibility**: Allows switching database implementations without changing business logic
- **Type Safety**: Leverages Rust's type system with SQLx compile-time query verification

### Technology Stack
- **SQLx**: Type-safe SQL toolkit for Rust
- **PostgreSQL**: Primary database system
- **async-trait**: Enables async functions in traits
- **Rust async/await**: For non-blocking database operations

## Implementation Guide

### Step 1: Define the Repository Trait
Create `src/repository/user_repository.rs` with the UserRepository trait definition:

```rust
use async_trait::async_trait;
use crate::models::{User, CreateUserRequest, UpdateUserRequest};
use crate::error::{ApiError, Result};

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, req: CreateUserRequest) -> Result<User>;
    async fn get_user(&self, id: i32) -> Result<Option<User>>;
    async fn get_users(&self) -> Result<Vec<User>>;
    async fn update_user(&self, id: i32, req: UpdateUserRequest) -> Result<Option<User>>;
    async fn delete_user(&self, id: i32) -> Result<bool>;
}
```

**Key Design Decisions:**
- Use `Option<T>` for operations that may not find a resource
- Return `Result<T>` for all operations to handle database errors
- Accept request DTOs rather than raw parameters for flexibility

### Step 2: Implement SQLx Repository
Create the concrete implementation using SQLx and PostgreSQL:

```rust
pub struct SqlxUserRepository {
    pool: PgPool,
}
```

**Implementation Details:**

1. **Create User**: Use SQLx's query_as! macro for compile-time SQL verification
2. **Get User**: Implement with fetch_optional for null-safe retrieval
3. **Update User**: Build dynamic queries for partial updates
4. **Delete User**: Return boolean indicating successful deletion

### Step 3: Handle Dynamic Updates
The update operation requires special attention for partial updates:

```rust
async fn update_user(&self, id: i32, req: UpdateUserRequest) -> Result<Option<User>> {
    // Build query dynamically based on provided fields
    let mut query = String::from("UPDATE users SET updated_at = NOW()");
    let mut params = vec![];
    
    if let Some(name) = &req.name {
        params.push(name.clone());
        query.push_str(&format!(", name = ${}", params.len()));
    }
    // ... continue for other fields
}
```

**Benefits of Dynamic Queries:**
- Only update fields that are provided
- Maintain data integrity for unchanged fields
- Reduce unnecessary database writes

### Step 4: Module Organization
Update `src/repository/mod.rs`:

```rust
mod user_repository;
pub use user_repository::{UserRepository, SqlxUserRepository};
```

### Step 5: Add Dependencies
Update `Cargo.toml`:

```toml
[dependencies]
async-trait = "0.1"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono"] }
```

## Error Handling Strategy

### Database Error Mapping
Convert SQLx errors to application-specific errors:

```rust
.map_err(ApiError::Database)?
```

**Common Error Scenarios:**
- **Unique Constraint Violations**: Email already exists
- **Foreign Key Violations**: Referenced entity doesn't exist
- **Connection Errors**: Database unavailable
- **Query Timeout**: Long-running operations

### Error Propagation
Use the `?` operator for clean error propagation while maintaining error context.

## Testing Approach

### Integration Tests
Test against a real PostgreSQL instance for accurate behavior:

```rust
#[sqlx::test]
async fn test_create_user() {
    let pool = setup_test_database().await;
    let repo = SqlxUserRepository::new(pool);
    // Test implementation
}
```

### Transaction Testing
Ensure database operations respect transaction boundaries:

```rust
#[sqlx::test]
async fn test_with_transaction() {
    let mut tx = pool.begin().await.unwrap();
    // Perform operations
    tx.rollback().await.unwrap();
    // Verify rollback
}
```

### Test Coverage Requirements
- **CRUD Operations**: Test all basic operations
- **Edge Cases**: Null values, empty results, invalid IDs
- **Concurrent Access**: Test parallel operations
- **Transaction Isolation**: Verify transaction boundaries

## Performance Considerations

### Connection Pooling
- Use PgPool for efficient connection management
- Configure pool size based on expected load
- Monitor connection usage metrics

### Query Optimization
- Use prepared statements via SQLx macros
- Create appropriate database indexes
- Implement pagination for list operations

### Caching Strategy
Consider implementing caching for frequently accessed data:
- User lookups by ID
- Recently created users
- Common query patterns

## Security Considerations

### SQL Injection Prevention
- Use parameterized queries exclusively
- Leverage SQLx's compile-time verification
- Never concatenate user input into SQL strings

### Data Validation
- Validate email formats before database insertion
- Enforce string length limits
- Sanitize user input appropriately

## Migration Path

### From In-Memory to Database
1. Replace in-memory implementations with repository calls
2. Update service layer to use repository trait
3. Maintain backward compatibility during transition

### Future Enhancements
- Add pagination support to get_users
- Implement soft deletes with deleted_at timestamp
- Add query filtering and sorting capabilities
- Support bulk operations for efficiency

## Dependencies and Integration

### Required Components
- **Task 2**: Database Setup and Migrations (completed)
- **Task 3**: Core Models Implementation (completed)

### Integration Points
- Service layer will inject repository implementations
- Handlers will use services that depend on repositories
- Test utilities will provide mock implementations

## Monitoring and Observability

### Metrics to Track
- Query execution time
- Connection pool utilization
- Error rates by operation type
- Cache hit/miss ratios

### Logging Requirements
- Log slow queries (> 100ms)
- Track failed operations with context
- Monitor connection pool events

## Code Quality Standards

### Documentation
- Document all public methods
- Include usage examples in doc comments
- Explain complex query logic

### Error Messages
- Provide context-rich error messages
- Include relevant IDs and operation types
- Maintain error message consistency

## Next Steps
After completing this task:
1. Implement service layer using the repository
2. Create HTTP handlers that utilize services
3. Add comprehensive integration tests
4. Set up monitoring and metrics collection