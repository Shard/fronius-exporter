#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use std::net::SocketAddr;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_health_endpoint_responds_immediately() {
        // Set required environment variable
        std::env::set_var("DEFAULT_NETWORK", "10.0.0.0/30");
        
        // Spawn the server in a background task
        let server_handle = tokio::spawn(async {
            fronius_metrics::run_server().await
        });
        
        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Test that health endpoint responds quickly
        let start = tokio::time::Instant::now();
        let response = timeout(
            Duration::from_secs(1),
            reqwest::get("http://localhost:8000/health")
        )
        .await
        .expect("Health endpoint should respond within 1 second")
        .expect("Failed to connect to health endpoint");
        
        let elapsed = start.elapsed();
        
        // Verify response
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.text().await.expect("Failed to read response body");
        assert_eq!(body, "OK");
        
        // Verify it responded quickly (should be under 100ms)
        assert!(elapsed.as_millis() < 100, "Health endpoint took too long: {:?}", elapsed);
        
        // Clean up
        server_handle.abort();
        std::env::remove_var("DEFAULT_NETWORK");
    }

    #[tokio::test]
    async fn test_health_endpoint_independent_of_metrics() {
        // Set environment to a subnet that won't have any inverters
        std::env::set_var("DEFAULT_NETWORK", "169.254.0.0/30");
        
        // Spawn the server
        let server_handle = tokio::spawn(async {
            fronius_metrics::run_server().await
        });
        
        // Give server time to start
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Test health endpoint
        let health_response = reqwest::get("http://localhost:8000/health")
            .await
            .expect("Failed to connect to health endpoint");
        
        assert_eq!(health_response.status(), StatusCode::OK);
        
        // Test that metrics endpoint also responds (with empty data)
        let metrics_response = reqwest::get("http://localhost:8000/metrics")
            .await
            .expect("Failed to connect to metrics endpoint");
        
        assert_eq!(metrics_response.status(), StatusCode::OK);
        let metrics_body = metrics_response.text().await.expect("Failed to read metrics");
        assert_eq!(metrics_body, "", "Metrics should be empty when no inverters found");
        
        // Clean up
        server_handle.abort();
        std::env::remove_var("DEFAULT_NETWORK");
    }
}