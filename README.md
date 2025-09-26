# rust-basic-api

Rust-based Axum service providing the foundation for production-ready APIs. The project boots an HTTP server, exposes a health check endpoint, and is wired for PostgreSQL access using `sqlx` with Tokio runtime.

## Prerequisites
- Rust toolchain (1.77 or newer) with `cargo`
- PostgreSQL instance (optional for boot, required for database operations)
- Docker and Docker Compose (optional, for containerized workflows)

## Configuration
1. Copy the example environment configuration:
   ```bash
   cp .env.example .env
   ```
2. Update values as needed:
   - `DATABASE_URL` – PostgreSQL connection string (no defaults baked into the code)
   - `SERVER_PORT` – Port the Axum server listens on (default `3000`)
   - `DATABASE_MAX_CONNECTIONS` – Size of the SQLx connection pool (default `5`)
   - `RUST_LOG` – Tracing filter (default `info`)

## Running Locally
```bash
cargo run
```
The server starts on `0.0.0.0:<SERVER_PORT>` and exposes `GET /health`, returning `OK` when the service is up.

## Quality Gates
All changes must pass the following before submission:
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
```

## Testing
Unit tests validate configuration parsing and the health endpoint. Run with:
```bash
cargo test --workspace --all-features
```

## Docker Workflow
Build the container image locally:
```bash
docker build -t rust-basic-api .
```

Launch the full stack (API + PostgreSQL) with Docker Compose:
```bash
docker compose up --build
```

The API becomes available at `http://localhost:${SERVER_PORT:-3000}/health` once the containers are healthy. PostgreSQL runs on `localhost:${POSTGRES_PORT:-5432}` with credentials from your `.env` file.

## Project Structure
```
src/
  config.rs        # Environment-driven configuration loader
  error.rs         # Config error types
  models/          # Shared data structures (application state)
  repository/      # Database layer utilities
  routes/          # HTTP routes and handlers
```

## Next Steps
The scaffolding is ready for feature development: add domain models, repositories, and routes as future tasks require. Configuration and containerization are in place to support production-focused enhancements.
