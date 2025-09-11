# Task 1: Project Setup and Configuration

## Overview
Initialize a new Rust API project with Axum web framework, PostgreSQL database integration, and proper project structure following best practices.

## Technical Requirements

### Dependencies
The project requires the following Rust crates:
- **axum** (0.6.0+): Modern, ergonomic web framework
- **tokio** (1.x with full features): Async runtime
- **serde** (1.x with derive): Serialization/deserialization
- **serde_json** (1.x): JSON support
- **sqlx** (0.6.x): Async SQL toolkit with PostgreSQL support
- **tracing** (0.1.x): Application-level tracing
- **tracing-subscriber** (0.3.x): Tracing subscriber implementation
- **dotenv** (0.15.x): Environment variable management
- **anyhow** (1.0.x): Flexible error handling
- **thiserror** (1.0.x): Custom error types

### Project Structure
```
rust-basic-api/
├── Cargo.toml
├── Cargo.lock
├── .env
├── .env.example
├── Dockerfile
├── docker-compose.yml
├── src/
│   ├── main.rs         # Application entry point
│   ├── config.rs       # Configuration management
│   ├── error.rs        # Error types and handling
│   ├── models/         # Data models
│   │   └── mod.rs
│   ├── routes/         # HTTP route handlers
│   │   └── mod.rs
│   └── repository/     # Database access layer
│       └── mod.rs
```

## Implementation Steps

### Step 1: Initialize Cargo Project
```bash
cargo new rust-basic-api --bin
cd rust-basic-api
```

### Step 2: Configure Cargo.toml
Update `Cargo.toml` with all required dependencies:

```toml
[package]
name = "rust-basic-api"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.6"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.6", features = ["runtime-tokio-rustls", "postgres", "chrono", "json"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dotenv = "0.15"
anyhow = "1.0"
thiserror = "1.0"
```

### Step 3: Create Directory Structure
```bash
mkdir -p src/{models,routes,repository}
touch src/{config.rs,error.rs}
touch src/models/mod.rs
touch src/routes/mod.rs
touch src/repository/mod.rs
```

### Step 4: Implement Configuration Module
Create `src/config.rs` with environment-based configuration:

```rust
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL")?;
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
            
        Ok(Config {
            database_url,
            server_port,
        })
    }
}
```

### Step 5: Implement Main Application
Create `src/main.rs` with basic server setup:

```rust
mod config;
mod error;
mod models;
mod routes;
mod repository;

use config::Config;
use std::net::SocketAddr;
use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Load configuration
    let config = Config::from_env()?;
    
    // Build application router
    let app = Router::new()
        .route("/health", axum::routing::get(health_check));
    
    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
```

### Step 6: Create Environment Configuration
Create `.env.example`:
```env
DATABASE_URL=postgresql://user:password@localhost/dbname
SERVER_PORT=3000
RUST_LOG=info
```

Create `.env` with actual values for local development.

### Step 7: Create Dockerfile
```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-basic-api /usr/local/bin/rust-basic-api
CMD ["rust-basic-api"]
```

### Step 8: Create Docker Compose Configuration
```yaml
version: '3.8'

services:
  api:
    build: .
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/rust_api
      - SERVER_PORT=3000
      - RUST_LOG=info
    depends_on:
      - db

  db:
    image: postgres:15
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=rust_api
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

## Error Handling Patterns
This task establishes the foundation for error handling using `anyhow` for application errors and `thiserror` for custom domain errors. The error module will be expanded in subsequent tasks.

## Testing Approach
1. **Build Verification**: `cargo build` should complete without errors
2. **Health Check**: Server should respond to GET /health with "OK"
3. **Environment Variables**: Verify configuration loads correctly
4. **Docker Integration**: Container should build and run successfully
5. **Logging**: Tracing output should appear in console

## Dependencies for Next Tasks
This task creates the foundation that all subsequent tasks will build upon:
- Task 2: Will extend the error handling system
- Task 3: Will add database connection pool
- Task 4: Will implement user models
- Task 5-8: Will add routes and repository implementations