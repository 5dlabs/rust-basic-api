# Acceptance Criteria: Project Setup and Configuration

## Required Deliverables

### 1. Project Structure ✓
- [ ] Rust project initialized with `cargo new rust-basic-api --bin`
- [ ] All required directories created under `src/`:
  - [ ] `models/` directory with `mod.rs`
  - [ ] `routes/` directory with `mod.rs`
  - [ ] `repository/` directory with `mod.rs`
- [ ] Core module files created:
  - [ ] `src/config.rs`
  - [ ] `src/error.rs`
  - [ ] `src/main.rs`

### 2. Dependency Configuration ✓
- [ ] `Cargo.toml` contains all required dependencies:
  - [ ] axum = "0.6" or later
  - [ ] tokio with "full" features
  - [ ] serde with "derive" feature
  - [ ] serde_json
  - [ ] sqlx with PostgreSQL and required features
  - [ ] tracing and tracing-subscriber
  - [ ] dotenv
  - [ ] anyhow
  - [ ] thiserror

### 3. Configuration Module ✓
- [ ] `Config` struct defined with:
  - [ ] `database_url: String` field
  - [ ] `server_port: u16` field
- [ ] `Config::from_env()` method implemented
- [ ] Environment variables properly loaded using dotenv
- [ ] Default port (3000) provided when SERVER_PORT not set
- [ ] Proper error handling for missing DATABASE_URL

### 4. Main Application ✓
- [ ] Tracing/logging initialized with configurable log levels
- [ ] Configuration loaded from environment
- [ ] Axum router created with `/health` endpoint
- [ ] Server binds to configured port (0.0.0.0:{port})
- [ ] Health check returns "OK" response
- [ ] Proper error propagation using anyhow::Result

### 5. Environment Configuration ✓
- [ ] `.env.example` file created with:
  - [ ] DATABASE_URL example
  - [ ] SERVER_PORT example
  - [ ] RUST_LOG example
- [ ] `.env` file created for local development

### 6. Docker Configuration ✓
- [ ] Multi-stage Dockerfile created:
  - [ ] Builder stage using rust:1.70 or later
  - [ ] Dependency caching optimization
  - [ ] Final stage using debian:bookworm-slim
  - [ ] Binary properly copied to final image
- [ ] `docker-compose.yml` created with:
  - [ ] API service configuration
  - [ ] PostgreSQL service configuration
  - [ ] Proper networking between services
  - [ ] Volume for PostgreSQL data persistence
  - [ ] Environment variables properly set

## Test Cases

### Test Case 1: Local Build and Run
```bash
# Expected: Successful build
cargo build

# Expected: Server starts on port 3000
cargo run

# Expected: Returns "OK"
curl http://localhost:3000/health
```

### Test Case 2: Environment Variable Loading
```bash
# Create .env with custom port
echo "DATABASE_URL=postgresql://test:test@localhost/test" > .env
echo "SERVER_PORT=8080" >> .env

# Expected: Server starts on port 8080
cargo run

# Expected: Returns "OK"
curl http://localhost:8080/health
```

### Test Case 3: Docker Build
```bash
# Expected: Image builds successfully
docker build -t rust-basic-api .

# Expected: Container runs
docker run -p 3000:3000 \
  -e DATABASE_URL=postgresql://test:test@localhost/test \
  rust-basic-api
```

### Test Case 4: Docker Compose
```bash
# Expected: Both services start
docker-compose up -d

# Expected: API service is healthy
docker-compose ps

# Expected: Returns "OK"
curl http://localhost:3000/health

# Expected: PostgreSQL is accessible
docker-compose exec db psql -U postgres -c "SELECT 1"
```

### Test Case 5: Logging Verification
```bash
# Set debug logging
export RUST_LOG=debug
cargo run

# Expected: Debug level logs appear in console
# Including "Listening on 0.0.0.0:3000" or similar
```

## Performance Criteria
- [ ] Application starts in < 2 seconds
- [ ] Health check responds in < 100ms
- [ ] Docker image size < 100MB (final stage)
- [ ] Memory usage < 50MB at idle

## Security Checklist
- [ ] No hardcoded credentials in source code
- [ ] `.env` file is in `.gitignore`
- [ ] Database password not logged
- [ ] Server binds to 0.0.0.0 (not localhost) for container compatibility

## Documentation Requirements
- [ ] README.md includes:
  - [ ] Project setup instructions
  - [ ] Environment variable documentation
  - [ ] Docker usage instructions
  - [ ] Development workflow guide

## Definition of Done
- [ ] All code compiles without warnings
- [ ] All test cases pass
- [ ] Docker image builds and runs successfully
- [ ] Health endpoint responds correctly
- [ ] Logging works at different levels
- [ ] Project structure follows Rust conventions
- [ ] Code is formatted with `cargo fmt`
- [ ] No clippy warnings with `cargo clippy`