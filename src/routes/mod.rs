//! Routes module - HTTP route registration and handlers
//!
//! This module contains all route handlers and the main route registration function.

pub mod user_routes;

use crate::AppState;
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

// Re-export user route handlers for convenience
pub use user_routes::{create_user, delete_user, get_user, get_users, update_user};

/// Health check endpoint
///
/// # Endpoint
/// `GET /health`
///
/// # Returns
/// Plain text "OK" with 200 status code
async fn health_check() -> &'static str {
    "OK"
}

/// Register all application routes
///
/// # Arguments
/// * `state` - Application state containing database pool
///
/// # Returns
/// Configured router with all routes and middleware
///
/// # Routes
/// - `GET /health` - Health check endpoint
/// - `GET /users` - List all users
/// - `GET /users/:id` - Get user by ID
/// - `POST /users` - Create new user
/// - `PUT /users/:id` - Update user
/// - `DELETE /users/:id` - Delete user
pub fn register_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route(
            "/users",
            get(user_routes::get_users).post(user_routes::create_user),
        )
        .route(
            "/users/:id",
            get(user_routes::get_user)
                .put(user_routes::update_user)
                .delete(user_routes::delete_user),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state.pool)
}
