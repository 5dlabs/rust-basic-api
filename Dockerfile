# syntax=docker/dockerfile:1

FROM rust:1.76 as builder
WORKDIR /app

# Build dependencies first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copy actual source and build release binary
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin/rust-basic-api

ENV RUST_LOG=info \
    SERVER_HOST=0.0.0.0 \
    SERVER_PORT=3000

EXPOSE 3000
CMD ["rust-basic-api"]
