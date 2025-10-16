//! Rust Basic API Library
//!
//! This library provides the core functionality for a production-ready REST API
//! built with Axum, including configuration management, error handling, and routing.

pub mod config;
pub mod error;
pub mod models;
pub mod repository;
pub mod routes;

// Re-export commonly used types for convenience
pub use config::Config;
pub use error::{AppError, AppResult};
