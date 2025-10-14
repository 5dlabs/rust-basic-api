//! Rust Basic API Library
//!
//! A production-ready REST API library built with Axum framework.

pub mod config;
pub mod error;
pub mod models;
pub mod repository;
pub mod routes;

use sqlx::PgPool;

/// Application state shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub pool: PgPool,
}
