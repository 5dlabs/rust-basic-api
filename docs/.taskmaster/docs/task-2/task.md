# Task 2: Database Schema and Migrations

## Overview
Set up PostgreSQL database schema with SQLx migrations, connection pooling, and testing utilities for a user management system.

## Technical Requirements

### Database Schema
The system requires a `users` table with:
- Auto-incrementing primary key
- User name (required)
- Unique email address (required)
- Timestamp tracking for creation and updates
- Performance indexes on email and creation date

### SQLx Integration
- Migration-based schema management
- Connection pooling with configurable parameters
- Compile-time SQL verification
- Transaction support for testing

## Implementation Steps

### Step 1: Create Migrations Directory
```bash
mkdir migrations
```

### Step 2: Create Initial Migration
Create `migrations/001_initial_schema.sql`:

```sql
-- Main users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Performance indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at DESC);
```

### Step 3: Install SQLx CLI
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Step 4: Configure Database Connection Pool
Update `src/repository/mod.rs`:

```rust
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}
```

### Step 5: Update Main Application
Modify `src/main.rs` to include database initialization:

```rust
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... existing initialization code ...
    
    // Create database connection pool
    let pool = repository::create_pool(&config.database_url).await?;
    
    // Run migrations
    sqlx::migrate!().run(&pool).await?;
    
    tracing::info!("Database migrations completed");
    
    // Create app state
    let state = AppState { pool };
    
    // Build application router with state
    let app = Router::new()
        .route("/health", axum::routing::get(health_check))
        .with_state(state);
    
    // ... rest of server setup ...
}
```

### Step 6: Create Test Utilities
Create `src/repository/test_utils.rs`:

```rust
#[cfg(test)]
pub mod test_utils {
    use sqlx::{PgPool, Postgres, Transaction};
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    pub async fn setup_test_database() -> PgPool {
        INIT.call_once(|| {
            dotenv::from_filename(".env.test").ok();
        });
        
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env.test");
            
        let pool = super::create_pool(&database_url).await.unwrap();
        
        // Run migrations
        sqlx::migrate!().run(&pool).await.unwrap();
        
        pool
    }
    
    pub async fn transaction<'a>(pool: &'a PgPool) -> Transaction<'a, Postgres> {
        pool.begin().await.unwrap()
    }
}
```

### Step 7: Configure Test Environment
Create `.env.test`:
```env
DATABASE_URL=postgresql://postgres:password@localhost/rust_api_test
RUST_LOG=debug
```

### Step 8: Add Database Health Check
Enhance the health check to verify database connectivity:

```rust
async fn health_check(State(state): State<AppState>) -> Result<&'static str, StatusCode> {
    // Try to ping the database
    match sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => Ok("OK"),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
```

## Migration Management

### Running Migrations
```bash
# Run all pending migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert

# Check migration status
sqlx migrate info
```

### Creating New Migrations
```bash
sqlx migrate add <migration_name>
```

## Connection Pool Configuration

The connection pool is configured with:
- **Max Connections**: 10 (adjustable based on load)
- **Acquire Timeout**: 3 seconds (prevents hanging on connection issues)
- **Connection Recycling**: Automatic via SQLx

## Testing Strategy

### Integration Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::test_utils;
    
    #[sqlx::test]
    async fn test_database_schema() {
        let pool = test_utils::setup_test_database().await;
        
        // Check if users table exists
        let result = sqlx::query!(
            "SELECT EXISTS (
                SELECT FROM information_schema.tables 
                WHERE table_name = 'users'
            ) as exists"
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        
        assert!(result.exists.unwrap());
    }
    
    #[sqlx::test]
    async fn test_indexes_created() {
        let pool = test_utils::setup_test_database().await;
        
        // Check if indexes exist
        let indexes = sqlx::query!(
            "SELECT indexname FROM pg_indexes 
             WHERE tablename = 'users' 
             AND indexname IN ('idx_users_email', 'idx_users_created_at')"
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        
        assert_eq!(indexes.len(), 2);
    }
}
```

## Error Handling
Database errors should be properly handled and logged:
- Connection failures should trigger service unavailability
- Migration failures should prevent application startup
- Query errors should be properly propagated with context

## Performance Considerations
- Connection pooling reduces overhead of connection establishment
- Indexes on email and created_at optimize common query patterns
- Prepared statements via SQLx macros improve query performance

## Security Notes
- Use parameterized queries (SQLx prevents SQL injection)
- Store database credentials securely (environment variables)
- Use SSL/TLS for database connections in production
- Implement proper connection string validation

## Dependencies for Next Tasks
- Task 3: Will use the connection pool for user operations
- Task 4: Will implement user model matching this schema
- Task 5-8: Will build upon this database foundation