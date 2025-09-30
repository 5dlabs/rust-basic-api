# Rust Basic API

Production-ready Axum-based foundation service for building REST APIs backed by PostgreSQL. The service focuses on configuration-driven behaviour, structured logging, and container-first deployment.

## Features
- Async HTTP server built with Axum and Tokio
- Environment-driven configuration loader (supports `.env` files)
- Structured logging with `tracing`
- PostgreSQL connection pooling via `sqlx`
- Health check endpoint that validates database connectivity
- Multi-stage Docker build and docker-compose stack with PostgreSQL

## Getting Started

### Prerequisites
- Rust 1.80 or newer
- Cargo
- PostgreSQL instance (local or remote)
- Docker & Docker Compose (optional, for container workflows)

### Configuration
1. Copy the example environment file and adjust values as needed:
   ```bash
   cd rust-basic-api
   cp .env.example .env
   ```
2. Ensure `DATABASE_URL` points to a reachable PostgreSQL instance.
3. Optionally tweak connection pool parameters (`DATABASE_MAX_CONNECTIONS`, etc.).

### Running Locally
```bash
cd rust-basic-api
cargo run
```

The server binds to `SERVER_HOST:SERVER_PORT` (defaults to `0.0.0.0:3000`). Logs respect `RUST_LOG` filtering.

### Health Check
Once running, verify health and database connectivity:
```bash
curl http://localhost:3000/health
```
An `OK` response indicates both the HTTP server and the database are reachable.

### Testing & Linting
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
cargo test --workspace --all-features
```

### Docker Workflow
Build and run the containerised stack (API + PostgreSQL):
```bash
cd rust-basic-api
docker-compose up --build
```

The API will be available on `http://localhost:3000/health`. Default credentials are defined in `.env.example` and `docker-compose.yml`.

### Graceful Shutdown
Press `Ctrl+C` to stop the service. The server listens for the interrupt signal and completes inflight requests before exiting.

## Project Structure
```
rust-basic-api/
├── src/
│   ├── config.rs         # Environment/configuration loader
│   ├── error.rs          # Shared application error definitions
│   ├── main.rs           # Application entry point
│   ├── models/           # Data models (placeholders for future work)
│   ├── repository/       # Database connectivity helpers
│   └── routes/           # HTTP routes, including health check
├── .env.example          # Template for required environment variables
├── Dockerfile            # Multi-stage image build
└── docker-compose.yml    # Local development stack with PostgreSQL
```

## Next Steps
- Implement domain models and repositories for business logic
- Add integration tests leveraging the provided `test_support` helpers
- Extend routing to expose application-specific APIs
