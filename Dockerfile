# Base images are pinned to exact tags so a rebuild is reproducible; bump them
# deliberately. Builder and runtime are both Debian trixie so the binary links
# against the same glibc/libssl it runs with.
FROM rust:1.99.1-trixie AS builder
WORKDIR /opt/fronius/
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# --locked: build exactly the reviewed Cargo.lock (advisory fixes live there).
RUN cargo build --release --locked

FROM debian:trixie-20261020-slim
WORKDIR /opt/fronius/
COPY --from=builder /opt/fronius/target/release/fronius-metrics .
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

CMD ["./fronius-metrics"]
