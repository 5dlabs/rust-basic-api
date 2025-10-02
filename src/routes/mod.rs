use axum::{routing::get, Router};

#[allow(dead_code)]

    Router::new().route("/health", get(|| async { "OK" }))
}
