# Multi-stage build for Rust REST API

# Build stage
FROM rust:1.82 as builder

WORKDIR /app

# Install build dependencies required by sqlx and other crates
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release --locked

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Expose the application port
EXPOSE 3000

# Run the binary
CMD ["./rust-basic-api"]
