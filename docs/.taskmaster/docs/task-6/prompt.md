# Implement Comprehensive Testing Framework

You are tasked with setting up a comprehensive testing framework for a Rust API project. This includes creating test utilities, configuring CI/CD pipelines, establishing code coverage reporting, and ensuring robust automated testing practices.

## Context
You are working on a Rust API that has already implemented:
- Database setup with PostgreSQL (Task 2)
- Core models and validation (Task 3)
- Repository layer (Task 4)
- API route handlers (Task 5)

Your task is to establish a testing framework that ensures code quality and reliability.

## Requirements

### 1. Test Utilities Module
Create `src/test_utils.rs` with:
- Factory functions for test data generation
- Helper methods for common test operations
- Fixture management utilities
- Test database setup helpers

### 2. Test Environment Configuration
Set up test-specific configuration:
- Create `.env.test` with test database credentials
- Configure separate test database
- Enable debug logging for tests
- Ensure test isolation

### 3. Test Database Automation
Create `scripts/setup_test_db.sh`:
- Automate PostgreSQL container management
- Handle database creation and migrations
- Ensure idempotent operations
- Provide health checks

### 4. Coverage Reporting
Configure code coverage:
- Add tarpaulin to dev-dependencies
- Create coverage generation script
- Generate HTML reports
- Set coverage thresholds

### 5. CI/CD Pipeline
Create `.github/workflows/ci.yml`:
- Run on push and pull requests
- Set up PostgreSQL service
- Install Rust toolchain
- Cache dependencies
- Run tests, linting, and formatting
- Generate coverage reports

### 6. Test Execution Script
Create `scripts/run_tests.sh`:
- Setup test environment
- Run all test suites
- Generate coverage reports
- Provide clear output

## Technical Specifications

### Dependencies to Add
```toml
[dev-dependencies]
tarpaulin = "0.25"
```

### File Structure
```
src/
└── test_utils.rs        # Test utilities module

scripts/
├── setup_test_db.sh     # Database setup script
└── run_tests.sh         # Test execution script

.github/workflows/
└── ci.yml               # CI/CD pipeline

.env.test                # Test environment config
```

### Test Patterns
1. **Unit Tests**: Use `#[cfg(test)]` modules
2. **Integration Tests**: Create files in `tests/` directory
3. **Database Tests**: Use `#[sqlx::test]` attribute
4. **API Tests**: Use `#[tokio::test]` with test router

## Implementation Steps

1. **Create test utilities**
   - Factory functions for models
   - Database setup helpers
   - Common assertions
   - Test fixtures

2. **Configure test environment**
   - Set up `.env.test`
   - Configure test database
   - Enable appropriate logging
   - Ensure isolation

3. **Automate database setup**
   - Create bash script for PostgreSQL
   - Handle container lifecycle
   - Run migrations automatically
   - Verify database readiness

4. **Set up coverage reporting**
   - Install tarpaulin
   - Configure coverage thresholds
   - Generate HTML reports
   - Integrate with CI

5. **Create CI/CD pipeline**
   - Configure GitHub Actions
   - Set up test matrix
   - Cache dependencies
   - Run quality checks

6. **Create test execution script**
   - Combine all test steps
   - Provide clear output
   - Generate reports
   - Handle errors gracefully

## Validation Criteria

Your implementation should:
- ✅ All existing tests pass
- ✅ Coverage reporting works
- ✅ CI pipeline runs successfully
- ✅ Test database setup is automated
- ✅ Test utilities are reusable
- ✅ Scripts are executable and documented
- ✅ Coverage meets minimum thresholds
- ✅ Tests run in isolation

## Code Quality Requirements

- Use clear, descriptive test names
- Document test utilities
- Keep tests independent
- Avoid test interdependencies
- Clean up test data
- Use appropriate assertions
- Follow Rust testing conventions

## Testing Strategy

### Unit Testing
- Test individual functions
- Mock external dependencies
- Focus on business logic
- Achieve high coverage

### Integration Testing
- Test API endpoints
- Test database operations
- Verify system integration
- Test error scenarios

### Performance Testing (Future)
- Benchmark critical paths
- Monitor response times
- Test under load
- Profile memory usage

## CI/CD Configuration

### GitHub Actions Workflow
```yaml
name: CI
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    # ... configuration
```

### Quality Checks
- Format check with `cargo fmt`
- Lint with `cargo clippy`
- Run all tests
- Generate coverage report
- Check minimum coverage

## Scripts Documentation

### setup_test_db.sh
- Starts PostgreSQL container if needed
- Creates test database
- Runs migrations
- Verifies database is ready

### run_tests.sh
- Calls setup_test_db.sh
- Runs cargo test
- Generates coverage report
- Displays results summary

## Notes
- Ensure test database is isolated from development
- Use transactions for test isolation
- Clean up test data after each run
- Monitor test execution time
- Keep tests maintainable and clear
- Document complex test setups

Begin by creating the test utilities module, then set up the test environment configuration, followed by the automation scripts and CI/CD pipeline. Ensure all components work together seamlessly to provide a robust testing framework.