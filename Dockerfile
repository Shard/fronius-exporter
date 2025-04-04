FROM rust:latest as builder
WORKDIR /opt/fronius/
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /opt/fronius/
COPY --from=builder /opt/fronius/target/release/fronius-metrics .
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

CMD ["./fronius-metrics"]
