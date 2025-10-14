# Multi-stage Docker build for Rust API
# Stage 1: Build the application
FROM rust:1.82 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies for PostgreSQL and SSL
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder stage
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["./rust-basic-api"]
