//! API routes module
//!
//! This module contains all HTTP route handlers and endpoint definitions.

use crate::app_state::AppState;
use axum::Router;

/// Build the application router with all routes
pub fn build_routes() -> Router<AppState> {
    Router::new()
}
