# rust-basic-api

A production-ready REST API built with Rust, Axum framework, and PostgreSQL database connectivity.

## 🚀 Features

- **Modern Async Framework**: Built with Axum 0.7 and Tokio runtime
- **Database Integration**: PostgreSQL support via SQLx with compile-time checked queries
- **Structured Logging**: Production-grade logging using `tracing` and `tracing-subscriber`
- **Configuration Management**: Environment-based configuration with `.env` file support
- **Error Handling**: Comprehensive error handling with `anyhow` and `thiserror`
- **Docker Support**: Multi-stage Dockerfile for optimized production deployments
- **Development Ready**: Docker Compose setup for local development with PostgreSQL

## 📋 Prerequisites

- Rust 1.70 or later (edition 2024)
- Cargo (comes with Rust)
- PostgreSQL database (optional - can use Docker Compose)
- Docker and Docker Compose (optional, for containerized setup)

## 🛠️ Project Structure

```
rust-basic-api/
├── src/
│   ├── main.rs           # Application entry point and HTTP server
│   ├── config.rs         # Configuration management from environment
│   ├── error.rs          # Error types and HTTP response handling
│   ├── models/           # Data models (future tasks)
│   │   └── mod.rs
│   ├── routes/           # API route handlers (future tasks)
│   │   └── mod.rs
│   └── repository/       # Database interaction layer (future tasks)
│       └── mod.rs
├── Cargo.toml            # Dependencies and project metadata
├── Dockerfile            # Multi-stage production Docker build
├── docker-compose.yml    # Local development environment
├── .env.example          # Example environment variables
└── clippy.toml          # Linting configuration
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the project root (or set environment variables):

```bash
DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_basic_api
SERVER_PORT=3000
RUST_LOG=info
```

See `.env.example` for a complete example.

### Configuration Fields

- **`DATABASE_URL`** (required): PostgreSQL connection string
- **`SERVER_PORT`** (optional, default: 3000): HTTP server port
- **`RUST_LOG`** (optional, default: info): Log level (trace, debug, info, warn, error)

## 🚀 Getting Started

### Local Development

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd rust-basic-api
   ```

2. **Set up environment variables**:
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run the application**:
   ```bash
   cargo run
   ```

5. **Test the health endpoint**:
   ```bash
   curl http://localhost:3000/health
   # Expected response: OK
   ```

### Using Docker Compose (Recommended for Development)

The easiest way to get started is using Docker Compose, which includes PostgreSQL:

1. **Start all services**:
   ```bash
   docker-compose up
   ```

2. **Test the API**:
   ```bash
   curl http://localhost:3000/health
   ```

3. **Stop services**:
   ```bash
   docker-compose down
   ```

### Using Docker (Production)

1. **Build the Docker image**:
   ```bash
   docker build -t rust-basic-api .
   ```

2. **Run the container**:
   ```bash
   docker run -p 3000:3000 \
     -e DATABASE_URL=postgresql://user:password@host:5432/dbname \
     -e SERVER_PORT=3000 \
     -e RUST_LOG=info \
     rust-basic-api
   ```

## 📡 API Endpoints

### Health Check

- **Endpoint**: `GET /health`
- **Description**: Returns the health status of the API
- **Response**: `200 OK` with body `"OK"`
- **Example**:
  ```bash
  curl http://localhost:3000/health
  ```

*Additional endpoints will be added in future tasks.*

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for specific workspace features
cargo test --workspace --all-features
```

## 🔍 Code Quality

### Format Code

```bash
# Check formatting
cargo fmt --all -- --check

# Auto-format code
cargo fmt --all
```

### Linting

```bash
# Run clippy with pedantic lints
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
```

The project uses a custom `clippy.toml` configuration based on AWS SDK Rust best practices.

## 📦 Dependencies

### Core Dependencies

- **axum** (0.7): Web application framework
- **tokio** (1.x): Async runtime with full features
- **serde** (1.x): Serialization/deserialization
- **serde_json** (1.x): JSON support
- **sqlx** (0.7): Async SQL toolkit with PostgreSQL support
- **tracing** (0.1): Application-level tracing
- **tracing-subscriber** (0.3): Tracing subscriber implementations
- **dotenv** (0.15): Environment variable loading from `.env` files
- **anyhow** (1.x): Flexible error handling
- **thiserror** (1.x): Derive macros for custom error types

## 🏗️ Architecture

### Application Layers

1. **HTTP Layer** (`main.rs`): 
   - Axum router and server configuration
   - Middleware setup
   - Request/response handling

2. **Configuration Layer** (`config.rs`):
   - Environment variable loading
   - Application configuration management
   - Defaults and validation

3. **Error Layer** (`error.rs`):
   - Custom error types
   - HTTP status code mapping
   - Error response formatting

4. **Data Layer** (future tasks):
   - Models: Domain entities
   - Repository: Database operations
   - Routes: HTTP handlers

### Async Runtime

The application uses Tokio as its async runtime with the following configuration:
- Full feature set enabled
- Multi-threaded work-stealing scheduler
- Async I/O for network and database operations

## 🐳 Docker

### Multi-Stage Build

The Dockerfile uses a multi-stage build process:

1. **Builder Stage**: 
   - Uses `rust:1.80` base image
   - Compiles the application in release mode
   - Produces optimized binary

2. **Runtime Stage**:
   - Uses `debian:buster-slim` for minimal size
   - Only includes the compiled binary
   - Exposes port 3000

### Docker Compose Services

- **api**: Rust API service (port 3000)
- **postgres**: PostgreSQL 15 database (port 5432)
  - Health checks enabled
  - Persistent volume for data
  - Automatic initialization

## 📝 Development Guidelines

### Code Style

- Follow Rust idioms and best practices
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Document public APIs with `///` comments
- Include module-level documentation with `//!`

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use `?` operator for error propagation
- Avoid `unwrap()` in production code (tests are ok)
- Provide meaningful error messages

### Logging

- Use `tracing` macros instead of `println!`
- Log levels: `trace!`, `debug!`, `info!`, `warn!`, `error!`
- Include context in log messages
- Avoid logging sensitive information

## 🔒 Security

- Never commit secrets or credentials to version control
- Use environment variables for sensitive configuration
- Keep dependencies updated
- Review security advisories regularly

## 🚧 Future Development

This project is part of a multi-task implementation plan:

- ✅ **Task 1**: Project Setup and Configuration (Current)
- 🔲 **Task 2**: Database Setup and Migrations
- 🔲 **Task 3**: API Server Implementation with CRUD operations
- 🔲 **Task 4**: User Authentication and Authorization

## 📄 License

[Add your license information here]

## 🤝 Contributing

[Add contribution guidelines here]

## 📧 Contact

[Add contact information here]

---

**Built with ❤️ using Rust and Axum**
