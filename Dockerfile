# Multi-stage Dockerfile for optimized Rust builds

# Stage 1: Build the application
FROM rust:1.70 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies only (caching layer)
RUN cargo build --release

# Remove dummy files
RUN rm -rf src

# Copy actual source code
COPY . .

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

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Copy database migrations
COPY --from=builder /app/migrations /app/migrations

# Expose the application port
EXPOSE 3000

# Set the entrypoint
CMD ["/app/rust-basic-api"]
