// Repository module - database interaction layer
pub mod user_repository;

pub use user_repository::{SqlxUserRepository, UserRepository};
