# Build stage
FROM rust:1.83 AS builder
WORKDIR /app

# Copy dependency manifests first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy src to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

# Install CA certificates for HTTPS connections
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin/rust-basic-api

# Expose application port
EXPOSE 3000

# Run as non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app
USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD [ "sh", "-c", "nc -z localhost 3000 || exit 1" ]

CMD ["rust-basic-api"]