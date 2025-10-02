FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin
EXPOSE 3000
CMD ["rust-basic-api"]