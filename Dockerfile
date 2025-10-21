# Build stage
FROM rust:1.70 as builder

# Create a new empty shell project
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.toml

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./rust-basic-api"]
