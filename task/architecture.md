# Architecture Notes — Task 1 (rust-basic-api)

## High-Level Overview
The `rust-basic-api` service is an Axum-based HTTP server layered around:
- **Configuration (`src/config.rs`)** — loads runtime settings from environment variables with defaults for host, port, and pool sizing. Missing or invalid values surface as typed `ConfigError`s.
- **Application State (`src/app_state.rs`)** — holds the shared configuration reference and a lazily connected SQLx `PgPool`, wrapped in an `Arc` for Axum state sharing.
- **HTTP Stack (`src/main.rs`, `src/routes/`)** — builds the router, wires the `/health` endpoint, and runs an `axum::Server` with graceful shutdown handling and tracing-based observability.
- **Database Abstraction (`src/repository/`)** — currently provides the connection pool factory; future data access code will live here.
- **Error Mapping (`src/error.rs`)** — unifies configuration/database/unknown errors into an `AppError` that renders JSON responses and integrates with Axum’s `IntoResponse`.

## Request Lifecycle
1. **Startup**
   - `main` initializes tracing (`tracing_subscriber` with `EnvFilter`).
   - Configuration loads via `Config::from_env()`, ensuring `DATABASE_URL` is present and parsing numeric/IP values.
   - `build_application` constructs the shared `AppState` and prepares the router.
2. **Routing**
   - `routes::create_router` registers `/health` and attaches `AppState` using Axum’s state API.
3. **Handler Execution**
   - `routes::health::health_check` receives `State<Arc<AppState>>`, inspects the SQLx pool, and returns `200 OK` (or `503` if the pool is closed). Structured logs capture the invocation and failure modes.
4. **Error Propagation**
   - Any `AppError` bubbled from handlers is serialized to JSON with an informative message and `500` status today (handlers can refine status codes as they expand).

## Configuration Surface
Environment variables (also captured in `.env.example`):
- `DATABASE_URL` *(required)* — connection string for PostgreSQL.
- `DATABASE_MAX_CONNECTIONS` *(default `5`)* — maximum SQLx pool size.
- `SERVER_HOST` *(default `0.0.0.0`)* — bind interface parsed as `IpAddr`.
- `SERVER_PORT` *(default `3000`)* — bind port.
- `RUST_LOG` *(default `info`)* — tracing filter string.

`docker-compose.yml` forwards these variables and provisions a companion PostgreSQL container for local development.

## Observability & Shutdown
- Tracing uses `tracing_subscriber::fmt` with environment-controlled filters.
- `run_application` drives Axum with `with_graceful_shutdown`, listening for Ctrl+C and, on Unix, SIGTERM (see `shutdown_listener` helpers in `main.rs`).

## Containerization
- Multi-stage Docker build (`Dockerfile`) compiles the release binary in an official Rust toolchain image, then copies it into a slim Debian runtime with `ca-certificates` installed.
- `docker-compose.yml` builds the API image, starts PostgreSQL, and wires environment variables/ports for a fully functional stack.

## Testing Strategy
- Unit tests cover configuration parsing edge cases, routing behaviour, lazy pool creation, and the HTTP server lifecycle via real Axum/hyper clients.
- Integration-style tests in `main.rs` spawn the server on ephemeral ports to assert `/health` responses and configuration-driven startup.

## Future Extension Points
- Expand `repository` with concrete query modules.
- Introduce additional routes and request/response models under `routes/` and `models/`.
- Layer middlewares (logging, authentication) once higher-level tasks require them.
