FROM rust:1.77-bullseye AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo 'fn main() {}' > src/main.rs
RUN cargo build --release

COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install --no-install-recommends -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin/rust-basic-api

ENV RUST_LOG=${RUST_LOG:-info}
EXPOSE 3000

CMD ["rust-basic-api"]
