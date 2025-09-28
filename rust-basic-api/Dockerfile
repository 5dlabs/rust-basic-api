# syntax=docker/dockerfile:1

FROM rust:1.75 AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml ./
COPY Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Build actual project
COPY . ./
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin/rust-basic-api

ENV RUST_LOG=info
EXPOSE 3000

CMD ["rust-basic-api"]
