# rust-basic-api

A production-ready REST API built with Rust and the Axum framework, featuring database connectivity, proper error handling, and containerization support.

## Environment Variables

The following environment variables are required or optional for configuration:

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | PostgreSQL connection string |
| `SERVER_PORT` | No | 3000 | Port on which the server will listen |
| `RUST_LOG` | No | info | Logging level (trace, debug, info, warn, error) |
| `DATABASE_POOL_MAX_CONNECTIONS` | No | 10 | Maximum number of database connections in the pool |
| `DATABASE_POOL_IDLE_TIMEOUT_SECS` | No | 30 | Timeout in seconds for idle database connections |
| `DATABASE_POOL_ACQUIRE_TIMEOUT_SECS` | No | 10 | Timeout in seconds for acquiring a database connection |

## Running the Application

### Using Cargo

```bash
# Set environment variables
export DATABASE_URL="<your-postgresql-connection-string>"

# Run the application
cargo run
```

### Using Docker

```bash
# Build the Docker image
docker build -t rust-basic-api .

# Run the container
docker run -p 3000:3000 \
  -e DATABASE_URL="<your-postgresql-connection-string>" \
  rust-basic-api
```

## API Endpoints

- `GET /health` - Health check endpoint that verifies database connectivity

## Development

### Running Tests

```bash
cargo test --workspace --all-features
```

### Code Quality Checks

```bash
# Format code
cargo fmt --all

# Run linting
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
```