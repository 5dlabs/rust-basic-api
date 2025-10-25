#!/usr/bin/env bash
#
# Test Execution Script with Coverage
#
# This script runs the full test suite with code coverage reporting.
# It handles test database setup, test execution, and coverage report generation.
#
# Usage:
#   ./scripts/run_tests.sh [OPTIONS]
#
# Options:
#   --no-setup      Skip test database setup (assumes database is already running)
#   --html-only     Generate HTML report without running tests
#   --fail-under N  Fail if coverage is below N% (default: 70)
#   --clean         Clean coverage artifacts before running
#   --help          Show this help message
#
# Environment:
#   DATABASE_URL    PostgreSQL connection string (from .env.test)
#   RUST_LOG        Logging level (from .env.test)

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
COVERAGE_DIR="${PROJECT_ROOT}/coverage"
FAIL_UNDER=70
SKIP_SETUP=false
HTML_ONLY=false
CLEAN=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

log_step() {
    echo ""
    echo -e "${BLUE}==>${NC} $*"
    echo ""
}

# Show usage information
show_help() {
    cat << EOF
Test Execution Script with Coverage

Usage:
  $0 [OPTIONS]

Options:
  --no-setup      Skip test database setup (assumes database is already running)
  --html-only     Generate HTML report without running tests
  --fail-under N  Fail if coverage is below N% (default: 70)
  --clean         Clean coverage artifacts before running
  --help          Show this help message

Examples:
  # Run all tests with coverage
  $0

  # Run tests without setting up database
  $0 --no-setup

  # Run tests with 90% coverage threshold
  $0 --fail-under 90

  # Clean and run tests
  $0 --clean

Environment:
  The script loads configuration from .env.test if available.

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --no-setup)
                SKIP_SETUP=true
                shift
                ;;
            --html-only)
                HTML_ONLY=true
                shift
                ;;
            --fail-under)
                FAIL_UNDER="$2"
                shift 2
                ;;
            --clean)
                CLEAN=true
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                echo ""
                show_help
                exit 1
                ;;
        esac
    done
}

# Load test environment
load_test_env() {
    local env_file="${PROJECT_ROOT}/.env.test"
    
    if [ -f "$env_file" ]; then
        log_info "Loading test environment from .env.test"
        # Export variables from .env.test
        set -a
        # shellcheck disable=SC1090
        source "$env_file"
        set +a
    else
        log_warn ".env.test not found, using defaults"
    fi
}

# Check required tools
check_dependencies() {
    log_step "Checking dependencies"

    # Check for cargo
    if ! command -v cargo &> /dev/null; then
        log_error "cargo is not installed"
        log_error "Please install Rust: https://rustup.rs/"
        exit 1
    fi

    # Check for coverage tool (prefer cargo-llvm-cov, fallback to tarpaulin)
    if command -v cargo-llvm-cov &> /dev/null; then
        log_info "Using cargo-llvm-cov for coverage"
        echo "COVERAGE_TOOL=llvm-cov" > /tmp/coverage_tool
    elif command -v cargo-tarpaulin &> /dev/null; then
        log_info "Using cargo-tarpaulin for coverage"
        echo "COVERAGE_TOOL=tarpaulin" > /tmp/coverage_tool
    else
        log_warn "No coverage tool found (cargo-llvm-cov or cargo-tarpaulin)"
        log_info "Installing cargo-llvm-cov..."
        cargo install cargo-llvm-cov --quiet || {
            log_error "Failed to install cargo-llvm-cov"
            exit 1
        }
        echo "COVERAGE_TOOL=llvm-cov" > /tmp/coverage_tool
    fi
}

# Setup test database
setup_database() {
    if [ "$SKIP_SETUP" = true ]; then
        log_info "Skipping database setup (--no-setup flag)"
        return 0
    fi

    log_step "Setting up test database"
    
    if [ -x "${SCRIPT_DIR}/setup_test_db.sh" ]; then
        "${SCRIPT_DIR}/setup_test_db.sh" start || {
            log_error "Failed to setup test database"
            exit 1
        }
    else
        log_warn "setup_test_db.sh not found or not executable"
        log_warn "Assuming test database is already running"
    fi
}

# Clean coverage artifacts
clean_coverage() {
    if [ "$CLEAN" = true ]; then
        log_step "Cleaning coverage artifacts"
        rm -rf "$COVERAGE_DIR"
        cargo clean
        log_info "Coverage artifacts cleaned"
    fi
}

# Run tests with coverage using llvm-cov
run_coverage_llvm_cov() {
    log_step "Running tests with cargo-llvm-cov"

    mkdir -p "$COVERAGE_DIR"

    # Run tests with coverage
    cargo llvm-cov \
        --workspace \
        --all-features \
        --html \
        --output-dir "$COVERAGE_DIR" \
        --fail-under-lines "$FAIL_UNDER" || {
        log_error "Tests failed or coverage below ${FAIL_UNDER}%"
        return 1
    }

    log_info "Coverage report generated at: ${COVERAGE_DIR}/html/index.html"
}

# Run tests with coverage using tarpaulin
run_coverage_tarpaulin() {
    log_step "Running tests with cargo-tarpaulin"

    mkdir -p "$COVERAGE_DIR"

    # Run tests with coverage
    cargo tarpaulin \
        --workspace \
        --all-features \
        --out Html \
        --output-dir "$COVERAGE_DIR" \
        --fail-under "$FAIL_UNDER" \
        --timeout 300 || {
        log_error "Tests failed or coverage below ${FAIL_UNDER}%"
        return 1
    }

    log_info "Coverage report generated at: ${COVERAGE_DIR}/tarpaulin-report.html"
}

# Run tests with coverage
run_tests() {
    if [ "$HTML_ONLY" = true ]; then
        log_info "Skipping test execution (--html-only flag)"
        return 0
    fi

    # Determine which coverage tool to use
    local coverage_tool
    if [ -f /tmp/coverage_tool ]; then
        coverage_tool=$(cat /tmp/coverage_tool | cut -d'=' -f2)
    else
        coverage_tool="llvm-cov"
    fi

    case "$coverage_tool" in
        llvm-cov)
            run_coverage_llvm_cov
            ;;
        tarpaulin)
            run_coverage_tarpaulin
            ;;
        *)
            log_error "Unknown coverage tool: $coverage_tool"
            exit 1
            ;;
    esac
}

# Display summary
show_summary() {
    log_step "Test Summary"
    
    echo -e "${GREEN}✓${NC} Tests completed successfully"
    echo -e "${GREEN}✓${NC} Coverage threshold met (≥${FAIL_UNDER}%)"
    
    if [ -d "$COVERAGE_DIR" ]; then
        echo ""
        echo "Coverage reports:"
        find "$COVERAGE_DIR" -name "*.html" -type f | while read -r report; do
            echo "  - file://$(realpath "$report")"
        done
    fi
    
    echo ""
    log_info "All tests passed! 🎉"
}

# Main execution
main() {
    parse_args "$@"

    cd "$PROJECT_ROOT"

    echo ""
    echo "========================================="
    echo "  Rust Basic API - Test Runner"
    echo "========================================="

    load_test_env
    check_dependencies
    clean_coverage
    setup_database
    run_tests
    show_summary

    echo ""
    echo "========================================="
    echo ""
}

# Trap errors
trap 'log_error "Script failed on line $LINENO"' ERR

main "$@"
