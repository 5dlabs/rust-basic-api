//! Comprehensive error handling tests

use axum::response::IntoResponse;
use rust_basic_api::error::{AppError, AppResult};

#[test]
fn test_app_result_ok() {
    let result: AppResult<i32> = Ok(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_app_result_err() {
    let result: AppResult<i32> = Err(AppError::config("test error"));
    assert!(result.is_err());
}

#[test]
fn test_config_error_with_string() {
    let error = AppError::config(String::from("test string error"));
    assert!(matches!(error, AppError::Config(_)));
}

#[test]
fn test_config_error_with_str() {
    let error = AppError::config("test str error");
    assert!(matches!(error, AppError::Config(_)));
}

#[test]
fn test_database_error_types() {
    let errors = vec![
        sqlx::Error::RowNotFound,
        sqlx::Error::ColumnNotFound("test".to_string()),
    ];

    for err in errors {
        let app_error = AppError::from(err);
        assert!(matches!(app_error, AppError::Database(_)));
    }
}

#[test]
fn test_error_display() {
    let error = AppError::config("configuration failed");
    let display = format!("{}", error);
    assert!(display.contains("Configuration error"));
    assert!(display.contains("configuration failed"));
}

#[test]
fn test_database_error_display() {
    let error = AppError::Database(sqlx::Error::RowNotFound);
    let display = format!("{}", error);
    assert!(display.contains("Database error"));
}

#[test]
fn test_error_response_status_codes() {
    let config_err = AppError::config("test");
    let response = config_err.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);

    let db_err = AppError::Database(sqlx::Error::RowNotFound);
    let response = db_err.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_error_from_sqlx() {
    let sql_error = sqlx::Error::RowNotFound;
    let _app_error: AppError = sql_error.into();
}

#[test]
fn test_multiple_config_errors() {
    let errors = vec![
        AppError::config("error 1"),
        AppError::config("error 2"),
        AppError::config("error 3"),
    ];

    for (i, err) in errors.iter().enumerate() {
        let display = format!("{}", err);
        assert!(display.contains(&format!("error {}", i + 1)));
    }
}
