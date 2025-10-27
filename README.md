# Rust Basic API

A production-ready REST API built with Axum framework, featuring database connectivity, structured logging, and containerization support.

## Features

- **Axum Framework**: Modern async web framework built on tokio
- **Database Support**: PostgreSQL integration via SQLx
- **Structured Logging**: Comprehensive tracing with configurable log levels
- **Error Handling**: Type-safe error handling with anyhow and thiserror
- **Configuration Management**: Environment-based configuration with dotenv
- **Docker Support**: Multi-stage Docker builds for optimized production images
- **Health Check Endpoint**: Built-in health check at `/health`

## Prerequisites

- Rust 1.70 or later
- PostgreSQL database
- Docker (optional, for containerization)

## Quick Start

### Local Development

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rust-basic-api
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Build the project**
   ```bash
   cargo build
   ```

4. **Run tests**
   ```bash
   cargo test --workspace --all-features
   ```

5. **Start the server**
   ```bash
   cargo run
   ```

   The server will start on `http://localhost:3000`

### Using Docker Compose

```bash
docker-compose up
```

This will start both the API server and a PostgreSQL database.

## Configuration

The application is configured via environment variables:

- `DATABASE_URL`: PostgreSQL connection string (Required)
- `SERVER_PORT`: HTTP server port (default: 3000)
- `RUST_LOG`: Log level configuration (default: `rust_basic_api=info,tower_http=debug`)
- `DB_MAX_CONNECTIONS`: pool maximum connections (default: 10)
- `DB_MIN_CONNECTIONS`: pool minimum idle connections (default: 1)
- `DB_CONNECT_TIMEOUT_SECS`: connect timeout in seconds (default: 5)
- `DB_IDLE_TIMEOUT_SECS`: idle timeout in seconds (default: 300)
- `DB_ACQUIRE_TIMEOUT_SECS`: acquire timeout in seconds (default: 30)

### Example Configuration

```env
DATABASE_URL=<your-postgres-connection-string>
SERVER_PORT=3000
RUST_LOG=rust_basic_api=info,tower_http=debug
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=1
DB_CONNECT_TIMEOUT_SECS=5
DB_IDLE_TIMEOUT_SECS=300
DB_ACQUIRE_TIMEOUT_SECS=30
```

## API Endpoints

### Health Check

- **GET** `/health`
  - Returns: `"OK"`
  - Description: Health check endpoint to verify server is running

## Project Structure

```
rust-basic-api/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs            # Library entry (for integration tests)
│   ├── config.rs         # Configuration management
│   ├── app_state.rs      # Shared application state
│   ├── error.rs          # Error types and handling
│   ├── models/           # Data models
│   │   └── mod.rs
│   ├── routes/           # API route handlers
│   │   └── mod.rs
│   └── repository/       # Database interaction layer
│       ├── mod.rs
│       └── test_utils.rs  # Test helpers
├── migrations/            # SQLx migrations
│   └── 001_initial_schema.sql
├── tests/
│   └── database_integration.rs  # Ignored by default; requires DB
├── Cargo.toml            # Project dependencies
├── clippy.toml           # Clippy linting configuration
├── .env.example          # Environment variables template
├── Dockerfile            # Multi-stage Docker build
└── docker-compose.yml    # Docker compose configuration
```

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace --all-features

# Run tests with coverage (requires cargo-tarpaulin or cargo-llvm-cov)
cargo llvm-cov --workspace --all-features --fail-under-lines 95
```

### Code Quality

This project enforces strict code quality standards:

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy with pedantic lints
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
```

### Linting Configuration

The project uses a custom `clippy.toml` based on AWS SDK Rust best practices:
- Disallows direct `SystemTime::now()` usage (use Clock abstraction for testability)
- Requires `tracing` macros instead of `println!`, `eprintln!`, or `dbg!`
- Enforces complexity limits and function parameter counts

## Docker

### Build Docker Image

```bash
docker build -t rust-basic-api .
```

### Run Docker Container

```bash
docker run -p 3000:3000 \
  -e DATABASE_URL=<your-postgres-connection-string> \
  rust-basic-api
```

## Dependencies

Key dependencies:
- `axum` - Web framework
- `tokio` - Async runtime
- `sqlx` - Database toolkit
- `serde` - Serialization framework
- `tracing` - Structured logging
- `anyhow` - Error handling
- `thiserror` - Custom error types

## License

[Your License Here]

## Contributing

[Your Contributing Guidelines Here]
