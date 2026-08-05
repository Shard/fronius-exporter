/// The binary must exit successfully and print its usage when asked for help.
#[test]
fn test_binary_help() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_fronius-metrics"))
        .arg("--help")
        .output()
        .expect("Failed to execute binary");

    assert!(
        output.status.success(),
        "binary --help should exit successfully"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage"),
        "--help should print usage information, got: {stdout}"
    );
    assert!(
        stdout.contains("DEFAULT_NETWORK"),
        "--help should document the network argument, got: {stdout}"
    );
}

/// An invalid CIDR passed through the supported CLI interface must fail fast
/// instead of starting the exporter and serving indefinitely.
#[test]
fn test_binary_rejects_invalid_cidr() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_fronius-metrics"))
        .arg("invalid-cidr")
        .output()
        .expect("Failed to execute binary");

    assert!(
        !output.status.success(),
        "binary should reject an invalid CIDR"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid CIDR format"),
        "expected Invalid CIDR format error, got: {stderr}"
    );
}
