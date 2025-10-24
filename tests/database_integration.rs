//! Integration tests for database schema and operations
//!
//! These tests verify the database schema, indexes, constraints, and basic operations.
//! They require a running `PostgreSQL` database configured in .env.test

use rust_basic_api::repository::test_utils::{cleanup_database, setup_test_database};

// Helper structs for query results
#[derive(sqlx::FromRow)]
struct InsertResult {
    id: i32,
}

#[derive(sqlx::FromRow)]
struct UserTimestamps {
    id: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct TimestampTest {
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::test]
async fn test_database_connection() {
    // Skip if DATABASE_URL not set
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;
    // Pool created successfully - just verify it exists
    drop(pool);
}

#[tokio::test]
async fn test_users_table_exists() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;

    // Check if users table exists
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public'
            AND table_name = 'users'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to check table existence");

    assert!(exists, "Users table should exist");
}

#[tokio::test]
async fn test_users_table_columns() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;

    // Check all required columns exist
    let columns: Vec<String> = sqlx::query_scalar(
        "SELECT column_name 
         FROM information_schema.columns 
         WHERE table_name = 'users' 
         ORDER BY ordinal_position",
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch column names");

    assert!(
        columns.contains(&"id".to_string()),
        "Users table should have id column"
    );
    assert!(
        columns.contains(&"name".to_string()),
        "Users table should have name column"
    );
    assert!(
        columns.contains(&"email".to_string()),
        "Users table should have email column"
    );
    assert!(
        columns.contains(&"created_at".to_string()),
        "Users table should have created_at column"
    );
    assert!(
        columns.contains(&"updated_at".to_string()),
        "Users table should have updated_at column"
    );
}

#[tokio::test]
async fn test_users_indexes_exist() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;

    // Check if email index exists
    let email_index_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM pg_indexes 
            WHERE tablename = 'users' 
            AND indexname = 'idx_users_email'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to check email index");

    assert!(email_index_exists, "Email index should exist");

    // Check if created_at index exists
    let created_at_index_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM pg_indexes 
            WHERE tablename = 'users' 
            AND indexname = 'idx_users_created_at'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to check created_at index");

    assert!(created_at_index_exists, "Created_at index should exist");
}

#[tokio::test]
async fn test_user_insertion() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;
    cleanup_database(&pool).await;

    // Insert a test user
    let result = sqlx::query_as::<_, InsertResult>(
        "INSERT INTO users (name, email) 
         VALUES ($1, $2) 
         RETURNING id",
    )
    .bind("Test User")
    .bind("test@example.com")
    .fetch_one(&pool)
    .await
    .expect("Failed to insert user");

    assert!(result.id > 0, "Inserted user should have valid ID");

    // Cleanup
    cleanup_database(&pool).await;
}

#[tokio::test]
async fn test_email_unique_constraint() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;
    cleanup_database(&pool).await;

    // Insert first user
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("User 1")
        .bind("unique@example.com")
        .execute(&pool)
        .await
        .expect("First insert should succeed");

    // Try to insert duplicate email
    let result = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("User 2")
        .bind("unique@example.com")
        .execute(&pool)
        .await;

    assert!(
        result.is_err(),
        "Duplicate email insertion should fail due to unique constraint"
    );

    // Cleanup
    cleanup_database(&pool).await;
}

#[tokio::test]
async fn test_updated_at_trigger() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;
    cleanup_database(&pool).await;

    // Insert a user
    let user = sqlx::query_as::<_, UserTimestamps>(
        "INSERT INTO users (name, email) 
         VALUES ($1, $2) 
         RETURNING id, created_at, updated_at",
    )
    .bind("Trigger Test User")
    .bind("trigger@example.com")
    .fetch_one(&pool)
    .await
    .expect("Failed to insert user");

    let initial_updated_at = user.updated_at;

    // Wait a bit to ensure timestamp difference
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Update the user
    sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
        .bind("Updated Name")
        .bind(user.id)
        .execute(&pool)
        .await
        .expect("Failed to update user");

    // Fetch updated record
    let updated_user = sqlx::query_as::<_, UserTimestamps>(
        "SELECT id, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(user.id)
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch updated user");

    assert!(
        updated_user.updated_at > initial_updated_at,
        "updated_at should be automatically updated by trigger"
    );
    assert_eq!(
        updated_user.created_at, user.created_at,
        "created_at should remain unchanged"
    );

    // Cleanup
    cleanup_database(&pool).await;
}

#[tokio::test]
async fn test_not_null_constraints() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;
    cleanup_database(&pool).await;

    // Try to insert user without name
    let result = sqlx::query("INSERT INTO users (email) VALUES ($1)")
        .bind("noname@example.com")
        .execute(&pool)
        .await;

    assert!(
        result.is_err(),
        "Insert without name should fail due to NOT NULL constraint"
    );

    // Try to insert user without email
    let result = sqlx::query("INSERT INTO users (name) VALUES ($1)")
        .bind("No Email User")
        .execute(&pool)
        .await;

    assert!(
        result.is_err(),
        "Insert without email should fail due to NOT NULL constraint"
    );

    // Cleanup
    cleanup_database(&pool).await;
}

#[tokio::test]
async fn test_default_timestamps() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;
    cleanup_database(&pool).await;

    // Insert user without explicit timestamps
    let user = sqlx::query_as::<_, TimestampTest>(
        "INSERT INTO users (name, email) 
         VALUES ($1, $2) 
         RETURNING created_at, updated_at",
    )
    .bind("Timestamp Test")
    .bind("timestamp@example.com")
    .fetch_one(&pool)
    .await
    .expect("Failed to insert user");

    // Both timestamps should be very close (within a second)
    let created = user.created_at;
    let updated = user.updated_at;
    let diff = (updated - created).num_milliseconds().abs();

    assert!(
        diff < 1000,
        "Initial created_at and updated_at should be nearly identical"
    );

    // Cleanup
    cleanup_database(&pool).await;
}

#[tokio::test]
async fn test_migration_idempotency() {
    if std::env::var("DATABASE_URL").is_err() {
        // Test skipped, no database configured
        return;
    }

    let pool = setup_test_database().await;

    // Running migrations again should not fail
    let result = sqlx::migrate!().run(&pool).await;

    assert!(
        result.is_ok(),
        "Running migrations multiple times should be idempotent"
    );
}
