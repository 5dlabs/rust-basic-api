# Multi-stage Dockerfile for Rust Basic API
# Stage 1: Build the application
FROM rust:1.82-bookworm as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates=20230311 \
    libssl3=3.0.15-1~deb12u1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Expose the application port
EXPOSE 3000

# Set environment variables with defaults
ENV RUST_LOG=info
ENV SERVER_PORT=3000

# Run the application
CMD ["/app/rust-basic-api"]
