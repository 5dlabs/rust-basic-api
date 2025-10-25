#!/usr/bin/env bash
#
# Test Database Setup Script
#
# This script manages the PostgreSQL test database container lifecycle.
# It handles container creation, database initialization, and health checks.
#
# Usage:
#   ./scripts/setup_test_db.sh [start|stop|restart|status]
#
# Options:
#   start   - Start the PostgreSQL test container (default)
#   stop    - Stop and remove the test container
#   restart - Restart the test container
#   status  - Check the status of the test container
#
# Environment:
#   The script reads configuration from .env.test if available.

set -euo pipefail

# Configuration
CONTAINER_NAME="rust_api_test_db"
POSTGRES_USER="${POSTGRES_USER:-postgres}"
# Set POSTGRES_PASSWORD environment variable before running this script
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-changeme}"
POSTGRES_DB="${POSTGRES_DB:-rust_api_test}"
POSTGRES_PORT="${POSTGRES_PORT:-5432}"
POSTGRES_VERSION="${POSTGRES_VERSION:-16-alpine}"
MAX_RETRIES=30
RETRY_INTERVAL=1

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $*"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

# Check if Docker is available
check_docker() {
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed or not in PATH"
        log_error "Please install Docker: https://docs.docker.com/get-docker/"
        exit 1
    fi

    if ! docker info &> /dev/null; then
        log_error "Docker daemon is not running"
        log_error "Please start Docker and try again"
        exit 1
    fi
}

# Check if container exists
container_exists() {
    docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"
}

# Check if container is running
container_running() {
    docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"
}

# Wait for PostgreSQL to be ready
wait_for_postgres() {
    log_info "Waiting for PostgreSQL to be ready..."
    local retries=0

    while [ $retries -lt $MAX_RETRIES ]; do
        if docker exec "$CONTAINER_NAME" pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB" &> /dev/null; then
            log_info "PostgreSQL is ready!"
            return 0
        fi

        retries=$((retries + 1))
        if [ $retries -lt $MAX_RETRIES ]; then
            echo -n "."
            sleep $RETRY_INTERVAL
        fi
    done

    echo ""
    log_error "PostgreSQL failed to become ready after ${MAX_RETRIES} seconds"
    return 1
}

# Start the test database container
start_container() {
    check_docker

    if container_running; then
        log_info "Test database container is already running"
        return 0
    fi

    if container_exists; then
        log_info "Starting existing test database container..."
        docker start "$CONTAINER_NAME" > /dev/null
    else
        log_info "Creating new test database container..."
        
        # Check if port is already in use
        if lsof -Pi ":${POSTGRES_PORT}" -sTCP:LISTEN -t >/dev/null 2>&1 ; then
            log_error "Port ${POSTGRES_PORT} is already in use"
            log_error "Please stop the service using this port or change POSTGRES_PORT in .env.test"
            exit 1
        fi

        docker run -d \
            --name "$CONTAINER_NAME" \
            -e POSTGRES_USER="$POSTGRES_USER" \
            -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
            -e POSTGRES_DB="$POSTGRES_DB" \
            -p "${POSTGRES_PORT}:5432" \
            "postgres:${POSTGRES_VERSION}" \
            > /dev/null

        log_info "Container created successfully"
    fi

    # Wait for database to be ready
    if wait_for_postgres; then
        log_info "Test database is ready at localhost:${POSTGRES_PORT}"
        log_info "Database: ${POSTGRES_DB}"
        log_info "User: ${POSTGRES_USER}"
        return 0
    else
        log_error "Failed to start test database"
        return 1
    fi
}

# Stop the test database container
stop_container() {
    check_docker

    if ! container_exists; then
        log_info "Test database container does not exist"
        return 0
    fi

    if container_running; then
        log_info "Stopping test database container..."
        docker stop "$CONTAINER_NAME" > /dev/null
        log_info "Container stopped"
    else
        log_info "Container is not running"
    fi

    log_info "Removing test database container..."
    docker rm "$CONTAINER_NAME" > /dev/null
    log_info "Container removed"
}

# Restart the test database container
restart_container() {
    log_info "Restarting test database..."
    stop_container
    start_container
}

# Show container status
show_status() {
    check_docker

    if ! container_exists; then
        log_info "Test database container: ${RED}NOT CREATED${NC}"
        return 1
    fi

    if container_running; then
        log_info "Test database container: ${GREEN}RUNNING${NC}"
        echo ""
        docker ps --filter "name=${CONTAINER_NAME}" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
        return 0
    else
        log_info "Test database container: ${YELLOW}STOPPED${NC}"
        return 1
    fi
}

# Main command dispatcher
main() {
    local command="${1:-start}"

    case "$command" in
        start)
            start_container
            ;;
        stop)
            stop_container
            ;;
        restart)
            restart_container
            ;;
        status)
            show_status
            ;;
        *)
            log_error "Unknown command: $command"
            echo ""
            echo "Usage: $0 [start|stop|restart|status]"
            echo ""
            echo "Commands:"
            echo "  start   - Start the PostgreSQL test container (default)"
            echo "  stop    - Stop and remove the test container"
            echo "  restart - Restart the test container"
            echo "  status  - Check the status of the test container"
            exit 1
            ;;
    esac
}

main "$@"
