//! Rust Basic API
//!
//! A production-ready REST API built with Rust, Axum, and `PostgreSQL`.

pub mod config;
pub mod error;
pub mod models;
pub mod repository;
pub mod routes;

use sqlx::PgPool;

/// Application state shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    /// `PostgreSQL` connection pool
    pub pool: PgPool,
}
