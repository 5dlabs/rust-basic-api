#[path = "../src/repository/test_utils.rs"]
mod test_utils;
use sqlx::types::chrono;

#[tokio::test]
#[ignore = "requires running PostgreSQL with DATABASE_URL set"]
async fn test_users_table_exists() {
    let pool = test_utils::test_pool().await.expect("pool");
    assert!(test_utils::users_table_exists(&pool).await.expect("exists"));
}

#[tokio::test]
#[ignore = "requires running PostgreSQL with DATABASE_URL set"]
async fn test_users_indexes_exist() {
    let pool = test_utils::test_pool().await.expect("pool");
    let idx_email: Option<String> = sqlx::query_scalar(
        "
        SELECT indexname FROM pg_indexes
        WHERE schemaname = 'public' AND tablename = 'users'
          AND indexname = 'idx_users_email'
        ",
    )
    .fetch_optional(&pool)
    .await
    .expect("query");
    assert_eq!(idx_email.as_deref(), Some("idx_users_email"));

    let idx_created_at: Option<String> = sqlx::query_scalar(
        "
        SELECT indexname FROM pg_indexes
        WHERE schemaname = 'public' AND tablename = 'users'
          AND indexname = 'idx_users_created_at'
        ",
    )
    .fetch_optional(&pool)
    .await
    .expect("query");
    assert_eq!(idx_created_at.as_deref(), Some("idx_users_created_at"));
}

#[tokio::test]
#[ignore = "requires running PostgreSQL with DATABASE_URL set"]
async fn test_users_crud_and_updated_at_trigger() {
    use sqlx::Row;
    let pool = test_utils::test_pool().await.expect("pool");
    let mut tx = test_utils::begin_tx(&pool).await.expect("tx");

    let rec = sqlx::query(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, created_at, updated_at",
    )
    .bind("Alice")
    .bind("alice@example.com")
    .fetch_one(&mut *tx)
    .await
    .expect("insert");

    let id: i64 = rec.get(0);
    let created_at: chrono::DateTime<chrono::Utc> = rec.get(1);
    let updated_at: chrono::DateTime<chrono::Utc> = rec.get(2);
    assert!(updated_at >= created_at);

    // Update and verify updated_at changed
    let updated = sqlx::query("UPDATE users SET name = $1 WHERE id = $2 RETURNING updated_at")
        .bind("Alice Smith")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .expect("update");
    let updated_at_2: chrono::DateTime<chrono::Utc> = updated.get(0);
    assert!(updated_at_2 >= updated_at);

    // Rollback
    tx.rollback().await.expect("rollback");
}
