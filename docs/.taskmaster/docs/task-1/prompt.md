# Autonomous Agent Prompt: Project Setup and Configuration

You are a senior Rust developer tasked with initializing a new REST API project using the Axum web framework. Your goal is to create a production-ready foundation with proper structure, dependency management, and containerization.

## Your Mission
Set up a complete Rust API project from scratch with all necessary dependencies, configuration management, and Docker support. The project should follow Rust best practices and be ready for immediate development of API endpoints.

## Required Actions

1. **Initialize the Rust project**:
   - Create a new binary Rust project named `rust-basic-api`
   - Set up the Cargo.toml with all required dependencies

2. **Configure all dependencies** in Cargo.toml:
   - Web framework: axum 0.6+
   - Async runtime: tokio with full features
   - Serialization: serde with derive, serde_json
   - Database: sqlx 0.6+ with PostgreSQL, TLS, chrono, and JSON support
   - Logging: tracing and tracing-subscriber
   - Configuration: dotenv
   - Error handling: anyhow and thiserror

3. **Create the project structure**:
   ```
   src/
   ├── main.rs
   ├── config.rs
   ├── error.rs
   ├── models/mod.rs
   ├── routes/mod.rs
   └── repository/mod.rs
   ```

4. **Implement configuration management**:
   - Create a Config struct in config.rs
   - Load DATABASE_URL and SERVER_PORT from environment
   - Use dotenv for local development
   - Provide sensible defaults where appropriate

5. **Implement the main application**:
   - Set up tracing/logging with environment-based log levels
   - Create an Axum router with a /health endpoint
   - Configure the server to bind to the configured port
   - Implement proper error handling with anyhow

6. **Create containerization files**:
   - Multi-stage Dockerfile for optimized production images
   - docker-compose.yml with API and PostgreSQL services
   - Proper environment variable configuration
   - Volume mounts for PostgreSQL data persistence

7. **Create environment configuration**:
   - .env.example with all required variables documented
   - .env for local development (should be gitignored)

## Expected Deliverables

### Files to Create:
- `Cargo.toml` - Fully configured with all dependencies
- `src/main.rs` - Application entry point with server setup
- `src/config.rs` - Configuration management module
- `src/error.rs` - Error type definitions (can be minimal for now)
- `src/models/mod.rs` - Models module placeholder
- `src/routes/mod.rs` - Routes module placeholder
- `src/repository/mod.rs` - Repository module placeholder
- `Dockerfile` - Multi-stage build for production
- `docker-compose.yml` - Local development environment
- `.env.example` - Environment variable template
- `.env` - Local environment configuration

### Code Quality Requirements:
- Use proper Rust idioms and conventions
- Include appropriate error handling
- Add meaningful log statements
- Ensure all code compiles without warnings
- Follow the principle of least privilege for configuration

## Validation Criteria
Your implementation will be considered complete when:
1. `cargo build` completes successfully
2. `cargo run` starts the server without errors
3. `curl http://localhost:3000/health` returns "OK"
4. `docker-compose up` successfully starts both services
5. Logs show proper initialization messages
6. Environment variables are correctly loaded

## Important Notes
- This is the foundation for a larger API project, so make design decisions that will scale
- Prefer explicit error handling over unwrapping
- Use async/await patterns consistently
- Ensure the database URL format is compatible with sqlx
- The health check endpoint should not require database connectivity

Begin by creating the project structure and proceed systematically through each requirement. Focus on creating a solid foundation that subsequent tasks can build upon.