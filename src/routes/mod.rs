use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, http::StatusCode};
    use hyper::body::to_bytes;
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn health_check_returns_ok() {
        let app = create_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("handler succeeds");

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body()).await.expect("body to bytes");

        assert_eq!(body.as_ref(), b"OK");
    }
}
