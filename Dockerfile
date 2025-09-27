# Multi-stage build: Builder stage
FROM rust:1.70 as builder

WORKDIR /app

# Copy Cargo files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the release binary
RUN cargo build --release

# Runtime stage: Slim runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    wget \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN useradd -r -u 1000 -m -d /app -s /bin/bash app

WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Set ownership and permissions
RUN chmod +x /app/rust-basic-api && chown -R app:app /app

# Switch to non-root user
USER app

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

# Run the binary
CMD ["./rust-basic-api"]