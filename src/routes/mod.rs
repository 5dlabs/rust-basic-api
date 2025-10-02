use axum::routing::get;
use axum::Router;

pub fn user_routes() -> Router {
    Router::new()
        .route("/login", get(login_handler))
        .route("/register", get(register_handler))
}

async fn login_handler() -> &'static str {
    "Login endpoint"
}

async fn register_handler() -> &'static str {
    "Register endpoint"
}
