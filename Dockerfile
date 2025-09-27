FROM rust:1.70 as builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() { println!(\"build time\"); }" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build application
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install --no-install-recommends -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-basic-api /app/

ENV RUST_LOG=info
EXPOSE 3000
CMD ["/app/rust-basic-api"]
