# Multi-stage Dockerfile for optimized Rust builds

# Stage 1: Build the application
FROM rust:1.83 as builder

WORKDIR /app

# Copy manifests for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies and build
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Copy migrations directory
COPY migrations ./migrations

# Build the application with all features
RUN cargo build --release --workspace

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install required runtime dependencies
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for security
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Copy database migrations
COPY --from=builder /app/migrations /app/migrations

# Switch to non-root user
USER appuser

# Expose the application port
EXPOSE 3000

# Set the entrypoint
CMD ["/app/rust-basic-api"]
