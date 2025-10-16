# Multi-stage build for Rust REST API

# Build stage
FROM rust:1.70 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install required runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Expose the application port
EXPOSE 3000

# Run the binary
CMD ["./rust-basic-api"]
