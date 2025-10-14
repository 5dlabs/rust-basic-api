//! Database integration tests
//!
//! Tests for database schema, migrations, and basic operations.

use sqlx::types::chrono;
use sqlx::{PgPool, Row};

/// Helper function to get test database pool
async fn get_test_pool() -> PgPool {
    dotenvy::from_filename(".env.test").ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

/// Helper function to cleanup test data
async fn cleanup_test_data(pool: &PgPool) {
    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(pool)
        .await
        .expect("Failed to cleanup test data");
}

#[tokio::test]
async fn test_database_connection() {
    let pool = get_test_pool().await;
    assert!(pool.size() > 0, "Pool should have connections");
}

#[tokio::test]
async fn test_users_table_exists() {
    let pool = get_test_pool().await;

    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public'
            AND table_name = 'users'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("Query failed");

    assert!(exists, "Users table should exist");
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_users_table_columns() {
    let pool = get_test_pool().await;

    // Check for expected columns
    let columns = sqlx::query_scalar::<_, String>(
        "SELECT column_name 
         FROM information_schema.columns 
         WHERE table_name = 'users'
         ORDER BY ordinal_position",
    )
    .fetch_all(&pool)
    .await
    .expect("Query failed");

    assert_eq!(columns.len(), 5, "Should have 5 columns");
    assert_eq!(columns[0], "id");
    assert_eq!(columns[1], "name");
    assert_eq!(columns[2], "email");
    assert_eq!(columns[3], "created_at");
    assert_eq!(columns[4], "updated_at");

    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_email_index_exists() {
    let pool = get_test_pool().await;

    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT FROM pg_indexes 
            WHERE tablename = 'users' 
            AND indexname = 'idx_users_email'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("Query failed");

    assert!(exists, "Email index should exist");
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_created_at_index_exists() {
    let pool = get_test_pool().await;

    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT FROM pg_indexes 
            WHERE tablename = 'users' 
            AND indexname = 'idx_users_created_at'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("Query failed");

    assert!(exists, "Created_at index should exist");
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_user_insert() {
    let pool = get_test_pool().await;
    cleanup_test_data(&pool).await;

    let row = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id")
        .bind("Test User")
        .bind("test@example.com")
        .fetch_one(&pool)
        .await
        .expect("Insert failed");

    let id: i32 = row.get("id");
    assert!(id > 0, "Should return valid ID");
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_email_unique_constraint() {
    let pool = get_test_pool().await;
    cleanup_test_data(&pool).await;

    // First insert should succeed
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("User1")
        .bind("unique@example.com")
        .execute(&pool)
        .await
        .expect("First insert should succeed");

    // Second insert with same email should fail
    let result = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("User2")
        .bind("unique@example.com")
        .execute(&pool)
        .await;

    assert!(result.is_err(), "Duplicate email should fail");
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_timestamps_auto_populate() {
    let pool = get_test_pool().await;
    cleanup_test_data(&pool).await;

    let row = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING created_at, updated_at")
        .bind("Timestamp Test")
        .bind("timestamp@example.com")
        .fetch_one(&pool)
        .await
        .expect("Insert failed");

    let created_at: Option<chrono::DateTime<chrono::Utc>> = row.get("created_at");
    let updated_at: Option<chrono::DateTime<chrono::Utc>> = row.get("updated_at");

    assert!(created_at.is_some(), "created_at should be set");
    assert!(updated_at.is_some(), "updated_at should be set");
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_updated_at_trigger() {
    let pool = get_test_pool().await;
    cleanup_test_data(&pool).await;

    // Insert a user
    let row = sqlx::query(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, created_at, updated_at",
    )
    .bind("Trigger Test")
    .bind("trigger@example.com")
    .fetch_one(&pool)
    .await
    .expect("Insert failed");

    let user_id: i32 = row.get("id");
    let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
    let initial_updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");

    // Wait a bit to ensure timestamp difference
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Update the user
    sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
        .bind("Trigger Test Updated")
        .bind(user_id)
        .execute(&pool)
        .await
        .expect("Update failed");

    // Fetch the updated user
    let updated_row =
        sqlx::query("SELECT created_at, updated_at FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .expect("Select failed");

    let updated_created_at: chrono::DateTime<chrono::Utc> = updated_row.get("created_at");
    let updated_updated_at: chrono::DateTime<chrono::Utc> = updated_row.get("updated_at");

    assert_eq!(
        updated_created_at, created_at,
        "created_at should not change"
    );
    assert!(
        updated_updated_at > initial_updated_at,
        "updated_at should be more recent"
    );

    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_user_select() {
    let pool = get_test_pool().await;
    cleanup_test_data(&pool).await;

    // Insert test data
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("Select Test")
        .bind("select@example.com")
        .execute(&pool)
        .await
        .expect("Insert failed");

    // Select the user
    let row = sqlx::query("SELECT id, name, email FROM users WHERE email = $1")
        .bind("select@example.com")
        .fetch_one(&pool)
        .await
        .expect("Select failed");

    let name: String = row.get("name");
    let email: String = row.get("email");

    assert_eq!(name, "Select Test");
    assert_eq!(email, "select@example.com");

    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_primary_key_constraint() {
    let pool = get_test_pool().await;

    let has_pk = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT FROM information_schema.table_constraints 
            WHERE table_name = 'users' 
            AND constraint_type = 'PRIMARY KEY'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("Query failed");

    assert!(has_pk, "Users table should have a primary key");
    cleanup_test_data(&pool).await;
}
