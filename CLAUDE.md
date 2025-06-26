# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Fronius Exporter is a Prometheus metrics exporter for Fronius solar inverters written in Rust. It automatically discovers inverters on a network subnet and exposes their metrics for monitoring with Grafana.

## Common Development Commands

### Building and Running
```bash
# Build the project
cargo build

# Run in development mode
RUST_LOG=debug DEFAULT_NETWORK=192.168.1.0/24 cargo run

# Build release version
cargo build --release

# Run tests
cargo test

# Format code (if rustfmt is installed)
cargo fmt

# Lint code (if clippy is installed)
cargo clippy
```

### Docker Operations
```bash
# Build Docker image
docker build -t fronius-exporter .

# Run the full monitoring stack (includes Prometheus & Grafana)
docker compose up -d --build

# View logs
docker compose logs -f fronius-exporter
```

## Architecture Overview

The application follows a simple async HTTP server pattern using Axum:

1. **Entry Point** (`src/main.rs`): Sets up the Axum router with a single `/metrics` endpoint and manages the inverter discovery background task.

2. **Service Discovery**: Every 5 minutes, scans the configured subnet to find Fronius inverters by attempting HTTP requests to known endpoints.

3. **Data Collection**: For each discovered inverter, concurrently fetches data from three endpoints:
   - `/components/BatteryManagementSystem/readable` (battery info)
   - `/components/cache/readable` (general metrics)
   - `/status/powerflow` (power flow data)

4. **Data Models** (`src/models.rs`): Strongly-typed Rust structs that map to the Fronius API JSON responses using serde.

5. **Metrics Conversion**: Uses `serde_prometheus` to automatically convert the structs into Prometheus exposition format.

## Key Implementation Details

- **Dual Inverter Support**: The system identifies "lefty" (with battery) and "righty" (without battery) inverters based on their capabilities.
- **Error Handling**: Uses `eyre` for error handling with proper context propagation.
- **Async Operations**: All HTTP requests and network operations are async using Tokio.
- **Environment Configuration**: Requires `DEFAULT_NETWORK` env var in CIDR format (e.g., "192.168.1.0/24").

## Testing Approach

- Tests are located in `/tests/` directory
- Mock JSON responses are available in `/mock/` for testing without real inverters
- Use `cargo test` to run all tests
- The HTTP mock test is currently marked as `#[ignore]` and needs implementation

## Deployment

The project includes:
- Dockerfile for containerization (multi-stage build)
- Docker Compose configuration with full monitoring stack
- Helm chart in `/charts/` for Kubernetes deployment
- Pre-configured Prometheus scraping and Grafana dashboards