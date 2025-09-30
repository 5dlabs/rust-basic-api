# syntax=docker/dockerfile:1

FROM rust:1.70 as builder
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build application
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim as runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

ENV APP_USER=app
RUN useradd -r -u 1000 -m -d /app -s /usr/sbin/nologin "$APP_USER"

WORKDIR /app

COPY --from=builder /app/target/release/rust-basic-api /app/rust-basic-api

RUN chown -R "$APP_USER":"$APP_USER" /app && chmod 755 /app/rust-basic-api

USER "$APP_USER"

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
ENV RUST_LOG=info

EXPOSE 3000

CMD ["./rust-basic-api"]
