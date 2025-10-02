use axum::Router;

async fn login_handler() -> &'static str {
    "Login handler placeholder"
} // TODO: Implement actual logic
async fn register_handler() -> &'static str {
    "Register handler placeholder"
} // TODO: Implement actual logic

pub fn user_routes() -> Router {
    Router::new()
        .route("/login", get(login_handler))
        .route("/register", get(register_handler))
}
