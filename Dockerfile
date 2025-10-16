# Build stage
FROM rust:1.83 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Cache dependencies
RUN cargo build --release && rm -rf src

# Copy source tree
COPY src ./src

# Touch main.rs to ensure it's newer than the cached dependency build
RUN touch src/main.rs

# Build for release
RUN cargo build --release --locked

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install required dependencies for runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin/

# Expose port
EXPOSE 3000

# Run the binary
CMD ["rust-basic-api"]
