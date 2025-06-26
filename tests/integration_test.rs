use std::process::{Command, Child};
use std::thread;
use std::time::Duration;

fn start_server() -> Child {
    Command::new("cargo")
        .args(&["run", "--release"])
        .env("DEFAULT_NETWORK", "169.254.0.0/30")  // Use link-local subnet that won't have inverters
        .env("RUST_LOG", "info")
        .spawn()
        .expect("Failed to start server")
}

#[test]
fn test_server_starts_and_responds() {
    // Start the server
    let mut server = start_server();
    
    // Give it time to start
    thread::sleep(Duration::from_secs(3));
    
    // Test health endpoint
    let health_output = Command::new("curl")
        .args(&["-s", "http://localhost:8000/health"])
        .output()
        .expect("Failed to execute curl");
    
    assert!(health_output.status.success());
    assert_eq!(String::from_utf8_lossy(&health_output.stdout), "OK");
    
    // Test metrics endpoint (should be empty initially)
    let metrics_output = Command::new("curl")
        .args(&["-s", "http://localhost:8000/metrics"])
        .output()
        .expect("Failed to execute curl");
    
    assert!(metrics_output.status.success());
    assert_eq!(String::from_utf8_lossy(&metrics_output.stdout), "");
    
    // Kill the server
    server.kill().expect("Failed to kill server");
}

#[test] 
fn test_health_endpoint_responds_immediately() {
    let mut server = start_server();
    
    // Test that health endpoint responds within 1 second of startup
    let start = std::time::Instant::now();
    
    // Poll health endpoint until it responds or timeout
    let mut responded = false;
    for _ in 0..10 {
        thread::sleep(Duration::from_millis(100));
        
        let result = Command::new("curl")
            .args(&["-s", "-m", "1", "http://localhost:8000/health"])
            .output();
            
        if let Ok(output) = result {
            if output.status.success() && String::from_utf8_lossy(&output.stdout) == "OK" {
                responded = true;
                break;
            }
        }
    }
    
    let elapsed = start.elapsed();
    assert!(responded, "Health endpoint did not respond");
    assert!(elapsed.as_secs() < 2, "Health endpoint took too long to respond: {:?}", elapsed);
    
    server.kill().expect("Failed to kill server");
}