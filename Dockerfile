FROM rust:1.80 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin
ENV DATABASE_URL=postgres://user:password@host/dbname
ENV SERVER_PORT=3000
CMD ["rust-basic-api"]