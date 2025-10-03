use axum::{routing::get, Router};

/// Login endpoint
/// This is a placeholder for future authentication implementation
async fn login_handler() -> &'static str {
    "Login endpoint - authentication not yet implemented"
}

/// Register endpoint
/// This is a placeholder for future user registration implementation
async fn register_handler() -> &'static str {
    "Register endpoint - user registration not yet implemented"
}

/// Configure user-related routes
pub fn user_routes() -> Router {
    Router::new()
        .route("/login", get(login_handler))
        .route("/register", get(register_handler))
}
