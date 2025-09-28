# rust-basic-api

Production-ready Axum starter service with PostgreSQL, structured logging, and container workflows.

## Getting Started

1. Enter the service directory:
   ```bash
   cd rust-basic-api
   ```
2. Copy the environment template and update the values as needed:
   ```bash
   cp .env.example .env
   ```
3. Export the variables (or rely on `dotenv`) and launch the API:
   ```bash
   cargo run
   ```

The server listens on the configured `SERVER_HOST`/`SERVER_PORT` (defaults to `0.0.0.0:3000`). Visit `http://localhost:3000/health` for a simple liveness check.

## Environment Variables

| Variable                    | Description                                      | Default (if unset)                      |
|-----------------------------|--------------------------------------------------|-----------------------------------------|
| `DATABASE_URL`              | PostgreSQL connection string                     | _required_                              |
| `DATABASE_MAX_CONNECTIONS`  | SQLx pool max connections                        | `5`                                     |
| `SERVER_HOST`               | Bind address for the HTTP server                 | `0.0.0.0`                               |
| `SERVER_PORT`               | Bind port for the HTTP server                    | `3000`                                  |
| `RUST_LOG`                  | Tracing filter (see `EnvFilter` syntax)          | `info`                                  |

## Tooling & Quality Gates

Before opening a PR, run the full set of gates from the project root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
# Enforce ≥95% line coverage
cargo llvm-cov --workspace --all-features --fail-under-lines 95
```

> Install the coverage tool once with `cargo install cargo-llvm-cov` (it downloads the LLVM toolchain on first use).

## Docker & Compose

Build a production image:

```bash
docker build -t rust-basic-api ./rust-basic-api
```

Run the stack with PostgreSQL via Compose (uses `.env` values if present):

```bash
cd rust-basic-api
docker compose up --build
```

The API will be reachable on `http://localhost:${SERVER_PORT:-3000}`.

## Repository Layout

```
rust-basic-api/
├── src/
│   ├── app_state.rs      # Shared application state (config + database)
│   ├── config.rs         # Environment-driven configuration loader
│   ├── error.rs          # Application + configuration error types
│   ├── main.rs           # Axum bootstrap and server entry point
│   ├── models/           # Domain model module (add submodules as needed)
│   ├── repository/       # Database pool factory + data access layer stubs
│   └── routes/           # HTTP route definitions (health check)
├── .env.example          # Configuration template
├── Cargo.toml
├── Dockerfile
└── docker-compose.yml
```

## Next Steps

- Extend the repository layer with concrete queries using the established `PgPool`.
- Add service endpoints and domain models under `routes/` and `models/`.
- Wire structured request/response logging and error mapping throughout handlers.
