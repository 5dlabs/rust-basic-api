# Multi-stage Dockerfile for optimized Rust builds

# Stage 1: Build the application
FROM rust:1.90 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install required runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Change ownership of the application directory
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose the application port
EXPOSE 3000

# Set the entrypoint
ENTRYPOINT ["/app/rust-basic-api"]
