//! Database integration tests
//!
//! Tests for verifying database schema, migrations, and basic operations.

use sqlx::PgPool;

/// Helper function to setup test database
async fn setup() -> PgPool {
    dotenv::from_filename(".env.test").ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create test database pool");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

#[tokio::test]
async fn test_users_table_exists() {
    let pool = setup().await;

    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public'
            AND table_name = 'users'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to check if users table exists");

    assert!(exists, "Users table should exist");
}

#[tokio::test]
async fn test_users_table_structure() {
    let pool = setup().await;

    // Check all required columns exist with correct types
    let columns = sqlx::query_as::<_, (String, String)>(
        "SELECT column_name, data_type 
         FROM information_schema.columns 
         WHERE table_name = 'users' 
         AND table_schema = 'public'
         ORDER BY ordinal_position",
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch table structure");

    assert_eq!(columns.len(), 5, "Users table should have 5 columns");

    let expected_columns = [
        ("id", "integer"),
        ("name", "character varying"),
        ("email", "character varying"),
        ("created_at", "timestamp with time zone"),
        ("updated_at", "timestamp with time zone"),
    ];

    for (expected, actual) in expected_columns.iter().zip(columns.iter()) {
        assert_eq!(
            expected.0, actual.0,
            "Column name mismatch: expected {}, got {}",
            expected.0, actual.0
        );
        assert_eq!(
            expected.1, actual.1,
            "Column type mismatch for {}: expected {}, got {}",
            expected.0, expected.1, actual.1
        );
    }
}

#[tokio::test]
async fn test_email_index_exists() {
    let pool = setup().await;

    let indexes = sqlx::query_scalar::<_, String>(
        "SELECT indexname 
         FROM pg_indexes 
         WHERE tablename = 'users' 
         AND indexname IN ('idx_users_email', 'idx_users_created_at')",
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch indexes");

    assert!(
        indexes.contains(&"idx_users_email".to_string()),
        "Email index should exist"
    );
    assert!(
        indexes.contains(&"idx_users_created_at".to_string()),
        "Created_at index should exist"
    );
}

#[tokio::test]
async fn test_user_insertion() {
    let pool = setup().await;

    // Use a transaction for test isolation
    let mut tx = pool.begin().await.expect("Failed to begin transaction");

    let result = sqlx::query_scalar::<_, i32>(
        "INSERT INTO users (name, email) 
         VALUES ($1, $2) 
         RETURNING id",
    )
    .bind("Test User")
    .bind("insertion_test@example.com")
    .fetch_one(&mut *tx)
    .await
    .expect("Failed to insert user");

    assert!(result > 0, "Inserted user should have positive ID");

    // Rollback transaction to clean up
    tx.rollback().await.expect("Failed to rollback");
}

#[tokio::test]
async fn test_email_unique_constraint() {
    let pool = setup().await;

    // Use a transaction for test isolation
    let mut tx = pool.begin().await.expect("Failed to begin transaction");

    // First insert should succeed
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("User 1")
        .bind("unique_test@example.com")
        .execute(&mut *tx)
        .await
        .expect("First insert should succeed");

    // Second insert with same email should fail
    let result = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("User 2")
        .bind("unique_test@example.com")
        .execute(&mut *tx)
        .await;

    assert!(
        result.is_err(),
        "Second insert with duplicate email should fail"
    );

    // Rollback transaction to clean up
    tx.rollback().await.expect("Failed to rollback");
}

#[tokio::test]
async fn test_updated_at_trigger() {
    let pool = setup().await;

    // Use a transaction for test isolation
    let mut tx = pool.begin().await.expect("Failed to begin transaction");

    // Insert a user
    let id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
    )
    .bind("Trigger Test")
    .bind("trigger_test@example.com")
    .fetch_one(&mut *tx)
    .await
    .expect("Failed to insert user");

    // Get initial timestamps
    let (created_at, updated_at_initial) =
        sqlx::query_as::<_, (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            "SELECT created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .expect("Failed to fetch initial timestamps");

    // Wait to ensure time difference (PostgreSQL has microsecond precision)
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Update the user
    sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
        .bind("Updated Name")
        .bind(id)
        .execute(&mut *tx)
        .await
        .expect("Failed to update user");

    // Get updated timestamps
    let (_, updated_at_final) = sqlx::query_as::<
        _,
        (chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>),
    >("SELECT created_at, updated_at FROM users WHERE id = $1")
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .expect("Failed to fetch final timestamps");

    assert!(
        updated_at_final > updated_at_initial,
        "updated_at should be updated by trigger. Initial: {updated_at_initial}, Final: {updated_at_final}"
    );
    assert!(
        updated_at_final > created_at,
        "updated_at should be later than created_at"
    );

    // Rollback transaction to clean up
    tx.rollback().await.expect("Failed to rollback");
}

#[tokio::test]
async fn test_default_timestamps() {
    let pool = setup().await;

    // Use a transaction for test isolation
    let mut tx = pool.begin().await.expect("Failed to begin transaction");

    // Insert without specifying timestamps
    let id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
    )
    .bind("Default Timestamps")
    .bind("defaults_test@example.com")
    .fetch_one(&mut *tx)
    .await
    .expect("Failed to insert user");

    // Fetch the user
    let (created_at, updated_at) = sqlx::query_as::<
        _,
        (
            Option<chrono::DateTime<chrono::Utc>>,
            Option<chrono::DateTime<chrono::Utc>>,
        ),
    >("SELECT created_at, updated_at FROM users WHERE id = $1")
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .expect("Failed to fetch user");

    assert!(created_at.is_some(), "created_at should have default value");
    assert!(updated_at.is_some(), "updated_at should have default value");

    // Rollback transaction to clean up
    tx.rollback().await.expect("Failed to rollback");
}

#[tokio::test]
async fn test_not_null_constraints() {
    let pool = setup().await;

    // Use a transaction for test isolation
    let mut tx = pool.begin().await.expect("Failed to begin transaction");

    // Try to insert without name
    let result = sqlx::query("INSERT INTO users (email) VALUES ($1)")
        .bind("no-name@example.com")
        .execute(&mut *tx)
        .await;

    assert!(result.is_err(), "Insert without name should fail");

    // Try to insert without email
    let result = sqlx::query("INSERT INTO users (name) VALUES ($1)")
        .bind("No Email")
        .execute(&mut *tx)
        .await;

    assert!(result.is_err(), "Insert without email should fail");

    // Rollback transaction to clean up
    tx.rollback().await.expect("Failed to rollback");
}
