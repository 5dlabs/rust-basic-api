# rust-basic-api

A production-ready REST API built with Rust and Axum framework.

## Setup

1. **Install Rust**: https://rustup.rs/

2. **Create environment configuration**:
   
   Create a `.env` file in the project root:
   ```
   DATABASE_URL=<your-connection-string>
   SERVER_PORT=3000
   RUST_LOG=rust_basic_api=info,tower_http=debug
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test --workspace --all-features
   ```

5. **Run the server**:
   ```bash
   cargo run
   ```

6. **Test the health endpoint**:
   ```bash
   curl http://localhost:3000/health
   ```

## Docker

Build and run with Docker:

```bash
docker build -t rust-basic-api .
docker run -p 3000:3000 -e DATABASE_URL=<connection-string> rust-basic-api
```

## Project Structure

- `src/config.rs` - Configuration management
- `src/error.rs` - Error handling
- `src/routes/` - API route handlers
- `src/models/` - Data models
- `src/repository/` - Database layer

## Quality Checks

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic

# Tests
cargo test --workspace --all-features
```