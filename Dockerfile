FROM rustlang/rust:nightly AS builder
WORKDIR /app

# Copy all files
COPY . .

# Build the application with nightly (which supports edition 2024)
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin/rust-basic-api

# Set environment variables
ENV DATABASE_URL=postgres://user:password@host/dbname
ENV SERVER_PORT=3000

# Expose port
EXPOSE 3000

CMD ["rust-basic-api"]
