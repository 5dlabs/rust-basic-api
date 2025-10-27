# Build stage
FROM rust:1.90 AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code and migrations
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    passwd \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and group for running the app
# Security: avoid running as root inside the container
RUN useradd --system --home /nonexistent --shell /usr/sbin/nologin appuser

# Copy the binary from builder
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api
# Include migrations at runtime to allow safe startup migrations
COPY --from=builder /app/migrations /app/migrations

# Adjust ownership to non-root user
RUN chown -R appuser:appuser /app

# Drop privileges
USER appuser:appuser

# Expose the default port
EXPOSE 3000

# Set the binary as the entrypoint
ENTRYPOINT ["/app/rust-basic-api"]
