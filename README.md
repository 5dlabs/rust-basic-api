# rust-basic-api

Rust-based REST API scaffold for Task 1. The project boots an Axum server with structured logging, environment-driven configuration, and a `/health` endpoint.

## Requirements
- Rust 1.70+
- Cargo
- PostgreSQL instance (local or remote)
- Docker & Docker Compose (optional, for container workflow)

## Configuration
Set the following environment variables (see `.env.example`):

- `DATABASE_URL` – PostgreSQL connection string (`postgresql://user:password@host:5432/db`)
- `SERVER_PORT` – port exposed by the HTTP server (defaults to `3000`)
- `RUST_LOG` – tracing filter (defaults to `info`)

You can copy the example file and edit it:

```bash
cp .env.example .env
```

## Running Locally
```bash
cargo run
# In another shell, verify
curl http://localhost:3000/health
```

Override the port when needed:

```bash
SERVER_PORT=8080 cargo run
```

## Tests & Quality Gates
```bash
cargo fmt
cargo clippy -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
cargo tarpaulin --engine llvm --workspace --all-features --skip-clean
```

> Install tarpaulin locally with `cargo install cargo-tarpaulin` before running the coverage command.

## Docker
Build the image:

```bash
docker build -t rust-basic-api .
```

Or run the full stack (API + PostgreSQL) with Docker Compose:

```bash
docker compose up --build
```

The API becomes available at `http://localhost:3000/health` with the database reachable on `localhost:5432`.
