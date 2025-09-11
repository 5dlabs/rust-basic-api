# Autonomous Agent Prompt: Database Schema and Migrations

You are a senior Rust developer with expertise in PostgreSQL and SQLx. Your task is to set up a robust database layer with migrations, connection pooling, and testing infrastructure.

## Your Mission
Implement a complete database setup for a user management API, including schema creation via migrations, connection pool configuration, and comprehensive testing utilities. The implementation should follow database best practices and integrate seamlessly with the existing Axum application.

## Required Actions

1. **Create the migrations structure**:
   - Create a `migrations` directory in the project root
   - Add the initial migration file for the users table

2. **Design the database schema**:
   - Users table with id, name, email, created_at, updated_at
   - Email must be unique
   - Add performance indexes on email and created_at
   - Use appropriate PostgreSQL data types

3. **Implement connection pooling**:
   - Create a `create_pool` function in `src/repository/mod.rs`
   - Configure with 10 max connections and 3-second timeout
   - Use PgPoolOptions for configuration

4. **Integrate database with the application**:
   - Update main.rs to create the connection pool
   - Run migrations automatically on startup
   - Add the pool to application state
   - Share state with all route handlers

5. **Create testing utilities**:
   - Implement `setup_test_database` function
   - Create transaction helper for test isolation
   - Use `.env.test` for test configuration
   - Ensure test database is separate from development

6. **Enhance health check**:
   - Update the /health endpoint to verify database connectivity
   - Return SERVICE_UNAVAILABLE if database is unreachable

7. **Add comprehensive tests**:
   - Test that migrations run successfully
   - Verify table and indexes are created
   - Test connection pool functionality

## Expected Deliverables

### Files to Create/Modify:
- `migrations/001_initial_schema.sql` - Initial database schema
- `src/repository/mod.rs` - Connection pool implementation
- `src/repository/test_utils.rs` - Testing utilities
- `.env.test` - Test environment configuration
- Update `src/main.rs` - Database integration
- Update health check endpoint

### Migration Schema:
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at DESC);
```

### Code Quality Requirements:
- Use async/await consistently
- Proper error handling with meaningful messages
- Include tracing statements for debugging
- Follow SQLx best practices
- Ensure compile-time SQL verification works

## Validation Criteria
Your implementation will be considered complete when:
1. Migrations run successfully with `sqlx migrate run`
2. Application connects to database on startup
3. Health check returns OK when database is available
4. Health check returns 503 when database is unavailable
5. Test database setup works independently
6. All tests pass with `cargo test`
7. Connection pool handles concurrent requests

## Technical Constraints
- Must use SQLx with compile-time verification
- PostgreSQL 15 or compatible
- Migrations must be reversible
- Connection pool must handle reconnection
- Tests must not affect production/development data

## Important Notes
- The database URL format should be: `postgresql://user:password@host:port/database`
- Ensure migrations are idempotent (can be run multiple times safely)
- Use transactions in tests for proper cleanup
- Consider future scalability in schema design
- The updated_at field will be managed by application logic (no triggers yet)

## Testing Requirements
Create tests that verify:
1. Database schema is created correctly
2. Indexes exist and are properly configured
3. Connection pool handles maximum connections
4. Migrations can be rolled back
5. Test utilities provide proper isolation

Begin by creating the migrations directory and proceed systematically through database setup. Ensure all database operations are properly logged and errors are handled gracefully.