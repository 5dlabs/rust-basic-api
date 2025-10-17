//! Error types and handling for the application.
//!
//! This module defines custom error types used throughout the application.

use std::fmt;

/// Application-level error type.
#[derive(Debug)]
#[allow(dead_code)] // Placeholder variants for future use
pub enum AppError {
    /// Configuration error
    Config(String),
    /// Database error
    Database(String),
    /// Internal server error
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(msg) => write!(f, "Configuration error: {msg}"),
            Self::Database(msg) => write!(f, "Database error: {msg}"),
            Self::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        Self::Config(err.to_string())
    }
}
