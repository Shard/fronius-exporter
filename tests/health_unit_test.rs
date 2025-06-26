use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    routing::get,
};
use tower::ServiceExt;

async fn health() -> &'static str {
    "OK"
}

#[tokio::test]
async fn test_health_endpoint() {
    // Create a simple router with just the health endpoint
    let app = Router::new().route("/health", get(health));

    // Create a request
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    // Send the request
    let response = app.oneshot(request).await.unwrap();

    // Check the response
    assert_eq!(response.status(), StatusCode::OK);
    
    // Check the body
    let body_bytes = axum::body::to_bytes(response.into_body(), 1024)
        .await
        .unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert_eq!(body_str, "OK");
}

#[tokio::test]
async fn test_health_endpoint_returns_quickly() {
    let app = Router::new().route("/health", get(health));

    let start = tokio::time::Instant::now();
    
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let _response = app.oneshot(request).await.unwrap();
    
    let elapsed = start.elapsed();
    
    // Health endpoint should respond in microseconds
    assert!(elapsed.as_millis() < 10, "Health endpoint took too long: {:?}", elapsed);
}