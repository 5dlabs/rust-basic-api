# syntax=docker/dockerfile:1.7

FROM rust:1.80-slim AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Caching layer for dependencies
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Copy actual sources and build
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin/rust-basic-api

ENV RUST_LOG=info
EXPOSE 3000

CMD ["/usr/local/bin/rust-basic-api"]
