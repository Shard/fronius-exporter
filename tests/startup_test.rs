#[test]
fn test_binary_exists() {
    // Just verify the binary can be built
    let output = std::process::Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to execute cargo build");
    
    assert!(output.status.success(), "Failed to build binary");
    assert!(std::path::Path::new("target/release/fronius-metrics").exists());
}

#[test]
fn test_binary_help() {
    // Ensure binary is built first
    std::process::Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to build");
    
    // Test that the binary at least starts (it will fail due to missing env var)
    let output = std::process::Command::new("target/release/fronius-metrics")
        .env("DEFAULT_NETWORK", "invalid")  // Invalid CIDR should cause quick failure
        .output()
        .expect("Failed to execute binary");
    
    // We expect it to panic on invalid CIDR, which is fine - it means the binary runs
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid CIDR format") || stderr.contains("panic"));
}