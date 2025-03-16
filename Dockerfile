FROM rust:latest
WORKDIR /opt/fronius/
COPY . .
RUN cargo build --release
CMD ./target/release/fronius-metrics
