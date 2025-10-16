//! Integration test for health endpoint

use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::test]
async fn test_health_endpoint_basic() {
    let app = Router::new().route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let response = reqwest::get(format!("http://{addr}/health"))
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let text = response.text().await.unwrap();
    assert_eq!(text, "OK");
}

#[tokio::test]
async fn test_health_endpoint_multiple_requests() {
    let app = Router::new().route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Make multiple concurrent requests
    let mut handles = vec![];
    for _ in 0..5 {
        let addr_clone = addr;
        let handle = tokio::spawn(async move {
            reqwest::get(format!("http://{addr_clone}/health"))
                .await
                .unwrap()
        });
        handles.push(handle);
    }

    for handle in handles {
        let response = handle.await.unwrap();
        assert_eq!(response.status(), 200);
        let text = response.text().await.unwrap();
        assert_eq!(text, "OK");
    }
}

#[tokio::test]
async fn test_404_for_unknown_routes() {
    let app = Router::new().route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let response = reqwest::get(format!("http://{addr}/unknown"))
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}
