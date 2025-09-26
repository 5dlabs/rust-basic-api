# rust-basic-api

A foundation for a production-ready REST API built with [Axum](https://github.com/tokio-rs/axum). The service exposes a health endpoint, loads configuration from the environment, initialises structured logging, and prepares a PostgreSQL connection pool via [`sqlx`](https://github.com/launchbadge/sqlx).

## Getting Started

1. Install the Rust toolchain (stable) and Docker (optional for containerised workflows).
2. Copy `.env.example` to `.env` and adjust the values for your environment:
   ```bash
   cp .env.example .env
   ```
3. Provide a reachable PostgreSQL instance that matches `DATABASE_URL` (for Docker usage a container is provisioned automatically).

### Environment Variables

| Variable | Description | Default in example |
| --- | --- | --- |
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://postgres:postgres@localhost:5432/rust_basic_api` |
| `DATABASE_MAX_CONNECTIONS` | Maximum pooled connections for `sqlx` | `5` |
| `SERVER_HOST` | Interface the HTTP server binds to | `0.0.0.0` |
| `SERVER_PORT` | Port exposed by the HTTP server | `3000` |
| `RUST_LOG` | Logging filter passed to `tracing-subscriber` | `info` |
| `POSTGRES_USER` | Username for dockerised PostgreSQL | `postgres` |
| `POSTGRES_PASSWORD` | Password for dockerised PostgreSQL | `postgres` |
| `POSTGRES_DB` | Database name for dockerised PostgreSQL | `rust_basic_api` |
| `DATABASE_PORT` | Host port used by dockerised PostgreSQL | `5432` |

### Local Development

```bash
cargo run
```

The API listens on `http://<SERVER_HOST>:<SERVER_PORT>`. Verify the health endpoint:

```bash
curl http://localhost:3000/health
```

### Quality Gates

Run the required quality gates before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
```

### Docker Workflow

Build and start the service alongside PostgreSQL:

```bash
docker-compose up --build
```

Once both containers are healthy, the health endpoint is reachable at `http://localhost:${SERVER_PORT:-3000}/health`.

To stop the stack:

```bash
docker-compose down -v
```

## Project Layout

```
src/
├── config.rs       # Environment-backed configuration loader
├── error.rs        # Shared error types and aliases
├── main.rs         # Application entrypoint and Axum bootstrap
├── models/         # Domain models
├── repository/     # Database connectivity helpers
└── routes/         # HTTP route definitions
```

Extend these modules when adding business logic, persistence, or additional endpoints.
