# Builder stage
FROM rust:1.90-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config=1.8.1-1 \
    libssl-dev=3.0.15-1~deb12u1 \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates=20230311 \
    libssl3=3.0.15-1~deb12u1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./rust-basic-api"]
