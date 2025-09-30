# rust-basic-api

An Axum-based REST API starter service wired for PostgreSQL, structured logging, and containerized deployment. The stack is production-oriented with Tokio for async runtime, tracing for observability, and SQLx for database access.

## Features
- Async HTTP server with [Axum](https://github.com/tokio-rs/axum)
- Environment-driven configuration loading (dotenv + env vars)
- Structured logging via `tracing`/`tracing-subscriber`
- SQLx PostgreSQL connection pool with configurable sizing
- Ready-to-run Dockerfile and `docker-compose.yml`

## Prerequisites
- Rust 1.70+
- Cargo
- PostgreSQL (local instance or managed service)
- Docker (optional, for containerized runs)

## Configuration
All runtime configuration is provided via environment variables. Copy `.env.example` to `.env` and adjust values as needed:

| Variable | Description | Default |
| --- | --- | --- |
| `DATABASE_URL` | PostgreSQL connection string | _(required)_ |
| `SERVER_HOST` | Interface to bind HTTP server | `0.0.0.0` |
| `SERVER_PORT` | Port to bind HTTP server | `3000` |
| `DATABASE_POOL_MAX_CONNECTIONS` | Max SQLx pool size | `5` |
| `DATABASE_POOL_MIN_CONNECTIONS` | Min SQLx pool size | `1` |
| `DATABASE_POOL_ACQUIRE_TIMEOUT_SECONDS` | Pool acquire timeout (seconds) | `10` |
| `RUST_LOG` | Tracing filter | `info` |

## Running Locally

```bash
# Ensure dependencies are installed
cargo fetch

# Format, lint, test
cargo fmt
cargo clippy --all-targets --all-features
cargo test --all-features

# Run the API (requires DATABASE_URL env var)
cargo run
```

Health check:

```bash
curl http://localhost:3000/health
```

## Docker

```bash
# Build image
docker build -t rust-basic-api .

# Run container (requires database URL)
docker run --rm \
  -p 3000:3000 \
  -e DATABASE_URL="postgresql://postgres:postgres@host.docker.internal:5432/rust_basic_api" \
  rust-basic-api
```

### Docker Compose

```bash
docker compose up --build
```

The compose file provisions PostgreSQL alongside the API and waits for the database to become healthy before starting the service.

## Project Structure

```
src/
├── config.rs        # Environment configuration loader
├── error.rs         # Configuration error types
├── main.rs          # Application entry point
├── models/          # Domain models (expanding in future tasks)
├── repository/      # Database connectivity and helpers
└── routes/          # HTTP route handlers
```

## Next Steps
- Add database migrations and schema management (`sqlx migrate`)
- Expand API routes and domain models
- Introduce observability exports (metrics, structured logs aggregation)

---

_Task 1 implements the baseline service scaffolding as described in `task/task.md` and `task/acceptance-criteria.md`._
