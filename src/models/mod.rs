//! Models module - data structures and business entities
//!
//! This module contains database models, DTOs, and validation utilities.

pub mod user;
pub mod validation;

// Re-export commonly used types for convenience
pub use user::{CreateUserRequest, UpdateUserRequest, User};
pub use validation::validate_request;
