# Rust Basic API

A production-ready REST API built with Rust, using the Axum web framework and PostgreSQL for data persistence.

## Features

- 🚀 Built with [Axum](https://github.com/tokio-rs/axum) web framework
- 🔄 Async runtime powered by [Tokio](https://tokio.rs/)
- 🗄️ PostgreSQL database integration via [SQLx](https://github.com/launchbadge/sqlx)
- 📝 Structured logging with [tracing](https://github.com/tokio-rs/tracing)
- 🔧 Environment-based configuration with [dotenvy](https://github.com/allan2/dotenvy)
- 🐳 Docker containerization with multi-stage builds
- ✅ Comprehensive test coverage
- 🔒 Security-focused with maintained dependencies

## Quick Start

### Prerequisites

- Rust 1.70 or later
- PostgreSQL 14+ (or use Docker Compose)
- Docker (optional, for containerized deployment)

### Local Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/5dlabs/rust-basic-api.git
   cd rust-basic-api
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env and set your DATABASE_URL
   ```

3. **Run with Docker Compose** (recommended)
   ```bash
   docker-compose up
   ```

4. **Or run locally**
   ```bash
   cargo run
   ```

5. **Test the health endpoint**
   ```bash
   curl http://localhost:3000/health
   # Expected: OK
   ```

## Configuration

The application is configured via environment variables:

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | PostgreSQL connection string |
| `SERVER_PORT` | No | `3000` | HTTP server port |
| `RUST_LOG` | No | `info` | Logging level |

Example `.env` file:
```env
DATABASE_URL="postgres://localhost:5432/dbname"
SERVER_PORT=3000
RUST_LOG=rust_basic_api=info,tower_http=debug
```

## Development

### Run Tests
```bash
cargo test --workspace --all-features
```

### Check Code Quality
```bash
# Format
cargo fmt --all -- --check

# Lint (pedantic)
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
```

### Build for Production
```bash
cargo build --release
```

## Docker

### Build Image
```bash
docker build -t rust-basic-api:latest .
```

### Run Container
```bash
docker run -p 3000:3000 \
  -e DATABASE_URL=your_db_url \
  rust-basic-api:latest
```

## Project Structure

```
rust-basic-api/
├── src/
│   ├── main.rs           # Application entry point
│   ├── config.rs         # Configuration management
│   ├── error.rs          # Error types and handling
│   ├── models/           # Data models
│   ├── routes/           # API route handlers
│   └── repository/       # Database interaction layer
├── Cargo.toml            # Dependencies
├── Dockerfile            # Multi-stage Docker build
├── docker-compose.yml    # Development environment
└── .env.example          # Environment template
```

## API Endpoints

### Health Check
- **GET** `/health` - Returns service health status
  ```bash
  curl http://localhost:3000/health
  # Response: OK
  ```

## Testing

The project includes comprehensive test coverage with multiple test layers:
- **Unit Tests**: Testing individual functions and modules
- **Integration Tests**: Testing database operations and API endpoints
- **Test Utilities**: Reusable test helpers and factory functions
- **Code Coverage**: Automated coverage reporting with cargo-llvm-cov

### Quick Test Commands

Run all tests:
```bash
cargo test --workspace --all-features
```

Run with coverage:
```bash
./scripts/run_tests.sh
```

### Test Infrastructure

#### 1. Test Database Setup
The project uses a dedicated PostgreSQL test database for integration tests.

First, set up your test environment:
```bash
# Copy test environment template
cp .env.test.example .env.test
# Edit .env.test and set your test database credentials
```

Then start the test database:
```bash
# Start test database
./scripts/setup_test_db.sh start

# Check status
./scripts/setup_test_db.sh status

# Stop test database
./scripts/setup_test_db.sh stop
```

The test database runs in a Docker container on port 5432 with credentials specified in `.env.test`.

#### 2. Test Execution Script
The `scripts/run_tests.sh` script provides automated test execution with coverage:

```bash
# Run all tests with coverage
./scripts/run_tests.sh

# Run without database setup
./scripts/run_tests.sh --no-setup

# Set coverage threshold
./scripts/run_tests.sh --fail-under 90

# Clean coverage artifacts
./scripts/run_tests.sh --clean
```

Coverage reports are generated in `./coverage/html/index.html`.

#### 3. Test Utilities
The project provides test utilities in `src/test_utils.rs`:

```rust
use rust_basic_api::test_utils::factories::*;

// Create test user
let user = create_test_user(1);

// Create user request
let request = create_user_request(1);

// Create update request
let update = update_user_request(1);
```

#### 4. Continuous Integration
GitHub Actions runs automated tests on every push and pull request:
- Code formatting checks
- Clippy linting (pedantic mode)
- Unit tests
- Integration tests with PostgreSQL
- Code coverage reporting

See `.github/workflows/ci.yml` for the complete CI pipeline.

## License

Licensed under either of:
- MIT License
- Apache License, Version 2.0

at your option.

## Contributing

Contributions are welcome! Please follow the project's coding guidelines and ensure all tests pass before submitting a pull request.
