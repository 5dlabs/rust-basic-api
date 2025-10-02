# rust-basic-api

A production-ready REST API built with Rust using the Axum web framework. This project provides a solid foundation for building scalable, asynchronous web services with PostgreSQL database integration.

## Features

- 🚀 **Async Runtime**: Built on Tokio for high-performance async I/O
- 🌐 **Web Framework**: Axum for ergonomic and type-safe HTTP routing
- 🗄️ **Database**: PostgreSQL integration via SQLx with compile-time query verification
- 📝 **Structured Logging**: Tracing-based observability
- 🐳 **Containerized**: Docker and Docker Compose support for easy deployment
- ⚙️ **Configuration**: Environment-based configuration management
- 🛡️ **Error Handling**: Custom error types with proper HTTP status mapping

## Prerequisites

- Rust 1.70 or later
- PostgreSQL 13+ (or use Docker Compose)
- Docker (optional, for containerized development)

## Quick Start

### Local Development

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd rust-basic-api
   ```

2. **Set up environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Run with Docker Compose (recommended):**
   ```bash
   docker-compose up
   ```
   This will start both PostgreSQL and the API service.

4. **Or run locally with Cargo:**
   ```bash
   # Ensure PostgreSQL is running and DATABASE_URL is set
   cargo run
   ```

5. **Test the health endpoint:**
   ```bash
   curl http://localhost:3000/health
   # Expected response: "OK"
   ```

## Configuration

Configuration is managed through environment variables. Create a `.env` file based on `.env.example`:

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `SERVER_PORT` | HTTP server port | `3000` |
| `RUST_LOG` | Log level (trace, debug, info, warn, error) | `info` |

### Example Configuration

```env
DATABASE_URL=postgresql://admin:password@localhost:5432/rust_basic_api
SERVER_PORT=3000
RUST_LOG=info
```

## Project Structure

```
rust-basic-api/
├── src/
│   ├── main.rs           # Application entry point
│   ├── config.rs         # Configuration management
│   ├── error.rs          # Error types and HTTP mapping
│   ├── models/           # Data models and DTOs
│   ├── routes/           # API route handlers
│   └── repository/       # Database interaction layer
├── Cargo.toml            # Rust dependencies
├── Dockerfile            # Multi-stage Docker build
├── docker-compose.yml    # Local development stack
└── .env.example          # Environment template
```

## Development

### Running Tests

```bash
cargo test --workspace --all-features
```

### Code Formatting

```bash
cargo fmt --all
```

### Linting

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Building for Production

```bash
cargo build --release
```

The optimized binary will be located at `target/release/rust-basic-api`.

## Docker

### Build Docker Image

```bash
docker build -t rust-basic-api:latest .
```

### Run with Docker

```bash
docker run -p 3000:3000 \
  -e DATABASE_URL=postgresql://user:pass@host:5432/db \
  -e RUST_LOG=info \
  rust-basic-api:latest
```

### Docker Compose

The provided `docker-compose.yml` includes:
- PostgreSQL database with health checks
- API service with automatic dependency management
- Persistent volume for database data

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f api

# Stop all services
docker-compose down

# Remove all data (including database)
docker-compose down -v
```

## API Endpoints

### Health Check

**GET** `/health`

Returns service health status.

**Response:**
```
OK
```

## Error Handling

The application uses custom error types that automatically map to appropriate HTTP status codes:

- `AppError::Database` → 500 Internal Server Error
- `AppError::NotFound` → 404 Not Found
- `AppError::BadRequest` → 400 Bad Request
- `AppError::Unauthorized` → 401 Unauthorized
- `AppError::Config` → 500 Internal Server Error

All errors return JSON responses:
```json
{
  "error": "Error message describing what went wrong"
}
```

## Architecture

### Layered Architecture

1. **Routes Layer** (`routes/`): HTTP request handling and routing
2. **Service Layer** (future): Business logic and orchestration
3. **Repository Layer** (`repository/`): Data access and persistence
4. **Models Layer** (`models/`): Data structures and domain entities

### Design Principles

- **Separation of Concerns**: Clear boundaries between layers
- **Type Safety**: Leverage Rust's type system for compile-time guarantees
- **Error Propagation**: Use Result types for explicit error handling
- **Async/Await**: Non-blocking I/O for concurrent request handling
- **Configuration Over Code**: Environment-driven configuration

## Dependencies

### Core Dependencies

- **axum**: Web application framework
- **tokio**: Async runtime
- **serde**: Serialization/deserialization
- **sqlx**: SQL toolkit with async support
- **tracing**: Structured logging and diagnostics

### Full Dependency List

See [`Cargo.toml`](Cargo.toml) for the complete list of dependencies.

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit your changes: `git commit -am 'Add new feature'`
4. Push to the branch: `git push origin feature/my-feature`
5. Submit a pull request

## License

[Specify your license here]

## Support

For issues and questions, please open an issue in the repository.

---

**Built with ❤️ using Rust and Axum**
