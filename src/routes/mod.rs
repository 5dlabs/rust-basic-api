pub mod health;

use axum::Router;

use crate::AppState;

/// Build the top-level application router by composing individual route modules.
pub fn create_router() -> Router<AppState> {
    Router::new().merge(health::router())
}
