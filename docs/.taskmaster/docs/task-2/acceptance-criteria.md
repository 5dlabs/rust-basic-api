# Acceptance Criteria: Database Schema and Migrations

## Required Deliverables

### 1. Migration Files ✓
- [ ] `migrations` directory created in project root
- [ ] `migrations/001_initial_schema.sql` file created with:
  - [ ] Users table definition
  - [ ] All required columns (id, name, email, created_at, updated_at)
  - [ ] Primary key constraint on id
  - [ ] Unique constraint on email
  - [ ] NOT NULL constraints on name and email
  - [ ] Default values for timestamps
  - [ ] Index on email column
  - [ ] Index on created_at column (descending)

### 2. Database Connection Pool ✓
- [ ] `src/repository/mod.rs` updated with:
  - [ ] `create_pool` function implemented
  - [ ] PgPoolOptions configuration
  - [ ] Max connections set to 10
  - [ ] Acquire timeout set to 3 seconds
  - [ ] Proper error handling

### 3. Application Integration ✓
- [ ] `src/main.rs` updated with:
  - [ ] Database pool creation on startup
  - [ ] Automatic migration execution
  - [ ] AppState struct with pool field
  - [ ] State sharing with router
  - [ ] Logging for migration status

### 4. Test Utilities ✓
- [ ] `src/repository/test_utils.rs` created with:
  - [ ] `setup_test_database` function
  - [ ] `transaction` helper function
  - [ ] Once initialization for test environment
  - [ ] Separate test database configuration
- [ ] `.env.test` file created with test database URL

### 5. Enhanced Health Check ✓
- [ ] Health endpoint updated to:
  - [ ] Accept application state
  - [ ] Query database for connectivity
  - [ ] Return OK on success
  - [ ] Return SERVICE_UNAVAILABLE on database error

## Test Cases

### Test Case 1: Migration Execution
```bash
# Setup database
docker-compose up -d db
export DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_api

# Run migrations
sqlx migrate run

# Expected: Migrations completed successfully
# Verify with:
sqlx migrate info
```

### Test Case 2: Schema Verification
```sql
-- Connect to database
psql $DATABASE_URL

-- Check table exists
\dt users

-- Check table structure
\d users

-- Expected output should show:
-- id: integer, primary key
-- name: character varying(255), not null
-- email: character varying(255), not null, unique
-- created_at: timestamp with time zone
-- updated_at: timestamp with time zone

-- Check indexes
\di *users*

-- Expected: idx_users_email and idx_users_created_at present
```

### Test Case 3: Application Startup
```bash
# Start application
cargo run

# Expected logs:
# - "Database migrations completed"
# - "Listening on 0.0.0.0:3000"

# No errors about database connection
```

### Test Case 4: Health Check with Database
```bash
# With database running
curl -i http://localhost:3000/health

# Expected: HTTP 200 OK with body "OK"

# Stop database
docker-compose stop db

# Restart application and test
curl -i http://localhost:3000/health

# Expected: HTTP 503 Service Unavailable
```

### Test Case 5: Connection Pool Testing
```rust
#[tokio::test]
async fn test_connection_pool_limits() {
    let pool = create_pool(&database_url).await.unwrap();
    
    // Acquire max connections
    let mut connections = vec![];
    for _ in 0..10 {
        connections.push(pool.acquire().await.unwrap());
    }
    
    // Try to acquire one more (should timeout)
    let result = tokio::time::timeout(
        Duration::from_secs(4),
        pool.acquire()
    ).await;
    
    assert!(result.is_err()); // Should timeout
}
```

### Test Case 6: Test Database Isolation
```bash
# Run tests
DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_api_test \
cargo test

# Expected: Tests pass without affecting main database
# Verify main database unchanged
```

## Performance Criteria
- [ ] Database connection established in < 1 second
- [ ] Migrations complete in < 2 seconds
- [ ] Health check with database query < 100ms
- [ ] Connection pool handles 10 concurrent connections
- [ ] Acquire timeout triggers after exactly 3 seconds

## Security Checklist
- [ ] Database credentials not hardcoded
- [ ] Connection string uses environment variables
- [ ] SQL injection prevented via parameterized queries
- [ ] Test database separate from production
- [ ] No sensitive data in migration files

## Error Handling Requirements
- [ ] Migration failures prevent application startup
- [ ] Connection failures logged with details
- [ ] Pool exhaustion handled gracefully
- [ ] Database errors don't crash application
- [ ] Meaningful error messages in logs

## Migration Requirements
- [ ] Migrations are reversible
- [ ] Migration files use sequential numbering
- [ ] Each migration is atomic (all or nothing)
- [ ] Migrations can be run multiple times safely
- [ ] Migration status trackable via SQLx CLI

## Documentation Requirements
- [ ] README updated with:
  - [ ] Database setup instructions
  - [ ] Migration commands
  - [ ] Test database configuration
  - [ ] Connection string format
  - [ ] SQLx CLI installation steps

## Definition of Done
- [ ] All migrations run successfully
- [ ] Database pool connects reliably
- [ ] Health check reflects database status
- [ ] Tests run in isolated environment
- [ ] No database-related warnings in logs
- [ ] All test cases pass
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] SQLx offline mode data generated (for CI/CD)