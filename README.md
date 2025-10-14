# rust-basic-api

A production-ready REST API built with Rust and the Axum framework, featuring database connectivity, structured logging, and comprehensive error handling.

## Features

- 🚀 **Modern Axum Framework** - Built with Axum 0.7 for high-performance HTTP services
- 🔒 **Type-Safe Configuration** - Environment-based configuration with validation
- 📊 **Structured Logging** - Production-grade observability with `tracing`
- 🛡️ **Robust Error Handling** - Custom error types with proper HTTP response mapping
- 🐳 **Docker Support** - Optimized multi-stage Dockerfile for production deployments
- ✅ **Zero Lint Warnings** - Strict clippy configuration following AWS SDK best practices
- 🧪 **Comprehensive Testing** - Unit tests with proper isolation

## Quick Start

### Prerequisites

- Rust 1.83 or later
- PostgreSQL database
- Docker (optional)

### Local Development

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rust-basic-api
   ```

2. **Configure environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your database connection details
   ```

3. **Run the server**
   ```bash
   cargo run
   ```

4. **Test the health endpoint**
   ```bash
   curl http://localhost:3000/health
   # Expected response: {"status":"OK"}
   ```

### Docker Deployment

```bash
# Build the Docker image
docker build -t rust-basic-api .

# Run the container
docker run -p 3000:3000 \
  -e DATABASE_URL=postgresql://user:password@host:5432/dbname \
  -e SERVER_PORT=3000 \
  -e RUST_LOG=info \
  rust-basic-api
```

## Configuration

All configuration is done via environment variables:

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | PostgreSQL connection string | - | Yes |
| `SERVER_PORT` | Port for HTTP server | `3000` | No |
| `RUST_LOG` | Logging level (error/warn/info/debug/trace) | `info` | No |

### Example Configuration

```bash
DATABASE_URL=postgresql://user:password@localhost:5432/rust_basic_api
SERVER_PORT=3000
RUST_LOG=info
```

## API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/health` | GET | Health check endpoint | `{"status":"OK"}` |

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace --all-features

# Run tests with sequential execution (for environment isolation)
cargo test --workspace --all-features -- --test-threads=1
```

### Code Quality Checks

```bash
# Format check
cargo fmt --all -- --check

# Linting with pedantic checks
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic

# Security scanning
gitleaks detect --no-git
trivy fs . --severity HIGH,CRITICAL
```

### Code Formatting

```bash
# Format code
cargo fmt --all
```

## Project Structure

```
rust-basic-api/
├── src/
│   ├── main.rs           # Application entry point and server setup
│   ├── config.rs         # Configuration management
│   ├── error.rs          # Custom error types
│   ├── models/           # Data models (ready for expansion)
│   ├── routes/           # API route handlers
│   │   └── mod.rs        # Health check endpoint
│   └── repository/       # Database interaction layer (ready for expansion)
├── Cargo.toml            # Dependencies and project metadata
├── clippy.toml           # Linting configuration (AWS SDK best practices)
├── Dockerfile            # Multi-stage production build
├── .env.example          # Environment variable template
├── .dockerignore         # Docker build optimization
└── README.md             # This file
```

## Dependencies

### Core Dependencies
- **axum** (0.7) - Web framework
- **tokio** (1.x) - Async runtime
- **serde** (1.x) - Serialization/deserialization
- **sqlx** (0.8) - Async SQL toolkit with PostgreSQL support
- **tracing** (0.1) - Structured logging

### Error Handling
- **anyhow** (1.0) - Application-level error handling
- **thiserror** (1.0) - Custom error types

### Configuration
- **dotenvy** (0.15) - Environment variable loading

### Development Dependencies
- **tower** (0.5) - Service utilities for testing
- **http-body-util** (0.1) - HTTP body utilities
- **serial_test** (3.0) - Test isolation for environment variables

## Coding Standards

This project follows strict coding standards:

- **Zero lint warnings** - All clippy warnings must be addressed
- **Comprehensive testing** - High test coverage with proper isolation
- **No mock data** - Real implementations with proper error handling
- **Parameterized configuration** - All values configurable via environment
- **Structured logging** - Use `tracing` macros, not `println!`
- **Security first** - No secrets in code, regular security scanning

See `coding-guidelines.md` and `github-guidelines.md` for detailed standards.

## CI/CD

The project uses GitHub Actions for continuous integration:

- **Formatting checks** - Ensures code is properly formatted
- **Linting** - Clippy with pedantic checks and zero warnings policy
- **Testing** - Runs all unit tests
- **Coverage** - Measures and reports test coverage
- **Security scanning** - Checks for vulnerabilities and secrets

## License

[Add your license information here]

## Contributing

1. Create a feature branch from `main`
2. Make your changes following the coding guidelines
3. Ensure all quality gates pass locally
4. Submit a pull request with proper labels

## Support

For issues or questions, please open an issue on GitHub.
