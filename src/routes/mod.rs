//! HTTP route handlers and application routing configuration.

use axum::{extract::State, http::StatusCode, routing::get, Router};

use crate::models::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}

async fn health_check(State(state): State<AppState>) -> (StatusCode, &'static str) {
    if state.db_pool.is_closed() {
        (StatusCode::SERVICE_UNAVAILABLE, "DB pool closed")
    } else {
        (StatusCode::OK, "OK")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use hyper::body::to_bytes;
    use tower::ServiceExt;

    use crate::models::AppState;
    use crate::repository;

    #[tokio::test]
    async fn health_check_returns_ok() {
        let state = AppState {
            db_pool: repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
                .expect("pool creation should succeed"),
        };

        let app = router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body()).await.expect("body bytes");
        assert_eq!(body.as_ref(), b"OK");
    }

    #[tokio::test]
    async fn health_check_detects_closed_pool() {
        let pool = repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
            .expect("pool creation should succeed");
        pool.close().await;

        let state = AppState { db_pool: pool };
        let app = router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

        let body = to_bytes(response.into_body()).await.expect("body bytes");
        assert_eq!(body.as_ref(), b"DB pool closed");
    }

    #[tokio::test]
    async fn health_check_handles_get_method() {
        let state = AppState {
            db_pool: repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
                .expect("pool creation should succeed"),
        };

        let app = router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn health_check_rejects_post_method() {
        let state = AppState {
            db_pool: repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
                .expect("pool creation should succeed"),
        };

        let app = router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn router_handles_unknown_route() {
        let state = AppState {
            db_pool: repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
                .expect("pool creation should succeed"),
        };

        let app = router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/unknown")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn router_accepts_state() {
        let pool = repository::create_pool("postgres://user:pass@localhost:5432/db", 5)
            .expect("pool creation should succeed");

        let state = AppState { db_pool: pool };

        // Should not panic when creating router with state
        let _app = router(state);
    }
}
