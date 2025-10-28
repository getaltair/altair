# CI/CD Pipeline Setup Guide

This document outlines requirements for setting up a CI/CD pipeline for the Altair monorepo.

## Current Setup

**Pre-commit Hooks**: The project uses pre-commit hooks for local validation:

- Dart formatting (`dart format --set-exit-if-changed`)
- Flutter analysis (`flutter analyze`)
- Markdown linting
- Secrets detection
- General file checks

**Location**: `.pre-commit-config.yaml` and `scripts/pre-commit/`

## CI/CD Pipeline Requirements

When implementing a full CI/CD pipeline (GitHub Actions, GitLab CI, etc.), include the following stages:

### 1. Code Quality Checks

```yaml
# Example: GitHub Actions
- name: Dart Format Check
  run: dart format --set-exit-if-changed .

- name: Flutter Analyze
  run: |
    for project in packages/altair-core packages/altair-auth packages/altair-ui packages/altair-db-service apps/altair_guidance; do
      cd $project
      flutter pub get
      flutter analyze
      cd -
    done
```

**Packages to check**:

- `packages/altair-core`
- `packages/altair-auth`
- `packages/altair-ui`
- `packages/altair-db-service`
- `apps/altair_guidance`

### 2. Unit Tests

Run unit tests for all packages:

```yaml
- name: Run Unit Tests
  run: |
    # Core packages
    cd packages/altair-core && flutter test && cd -
    cd packages/altair-auth && flutter test && cd -
    cd packages/altair-ui && flutter test && cd -
    cd packages/altair-db-service && flutter test && cd -

    # Apps
    cd apps/altair_guidance && flutter test && cd -
```

**Note**: Unit tests do NOT require SurrealDB. They use mocks and stubs.

### 3. Integration Tests (Optional)

Integration tests require a running SurrealDB instance.

#### Option A: Run with SurrealDB

```yaml
- name: Install SurrealDB
  run: curl -sSf https://install.surrealdb.com | sh

- name: Start SurrealDB
  run: |
    surreal start --bind 127.0.0.1:8000 --user altair --pass altair-local-dev file://./altair.db &
    sleep 5  # Wait for service to be ready

- name: Run Integration Tests
  run: |
    cd packages/altair-core
    flutter test test/repositories/task_repository_integration_test.dart
```

#### Option B: Skip Integration Tests in CI

Integration tests can be skipped in CI by using test tags:

```yaml
- name: Run Tests (Skip Integration)
  run: flutter test --exclude-tags=integration
```

**Tag integration tests** with:

```dart
@Tags(['integration'])
void main() {
  // Integration test code
}
```

### 4. Build Verification

Verify that apps build successfully:

```yaml
- name: Build Apps
  run: |
    cd apps/altair_guidance

    # Linux desktop build
    flutter build linux --release

    # Web build
    flutter build web --release

    # Android APK (requires Android SDK)
    # flutter build apk --release
```

## Database Service Considerations

### SurrealDB Installation in CI

If running integration tests, SurrealDB must be installed:

**Linux**:

```bash
curl -sSf https://install.surrealdb.com | sh
```

**macOS**:

```bash
brew install surrealdb/tap/surreal
```

**Docker** (recommended for CI):

```yaml
services:
  surrealdb:
    image: surrealdb/surrealdb:latest
    ports:
      - 8000:8000
    command: start --bind 0.0.0.0:8000 --user altair --pass altair-local-dev file://data/altair.db
```

### Test Database Isolation

Each test run should use a fresh database:

```bash
# Clean database before tests
rm -rf ./altair.db

# Start SurrealDB with fresh database
surreal start --bind 127.0.0.1:8000 --user altair --pass altair-local-dev file://./altair.db
```

### Health Check

Verify SurrealDB is ready before running tests:

```bash
# Wait for SurrealDB to be ready
until curl -f http://localhost:8000/health > /dev/null 2>&1; do
  echo "Waiting for SurrealDB..."
  sleep 1
done
```

## Recommended CI Workflow

### Pull Request Checks

1. **Fast checks** (always run):
   - Dart format
   - Flutter analyze
   - Unit tests (no database required)
   - Build verification

2. **Slow checks** (optional, or on-demand):
   - Integration tests (requires SurrealDB)
   - Platform-specific builds (Android, iOS, macOS, Windows)

### Main/Develop Branch

1. All PR checks
2. Full integration test suite
3. Platform builds
4. Deployment (if applicable)

## Example: GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  pull_request:
  push:
    branches: [main, develop]

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: subosito/flutter-action@v2
        with:
          flutter-version: '3.24.0'
          channel: 'stable'

      - name: Install Dependencies
        run: |
          for project in packages/altair-core packages/altair-auth packages/altair-ui packages/altair-db-service apps/altair_guidance; do
            cd $project && flutter pub get && cd -
          done

      - name: Format Check
        run: dart format --set-exit-if-changed .

      - name: Analyze
        run: ./scripts/pre-commit/flutter-analyze.sh

      - name: Unit Tests
        run: |
          cd packages/altair-core && flutter test && cd -
          cd packages/altair-db-service && flutter test && cd -
          cd apps/altair_guidance && flutter test && cd -

  integration:
    runs-on: ubuntu-latest
    if: github.event_name == 'push'  # Only on merged PRs
    steps:
      - uses: actions/checkout@v4
      - uses: subosito/flutter-action@v2

      - name: Install SurrealDB
        run: curl -sSf https://install.surrealdb.com | sh

      - name: Start Database
        run: |
          surreal start --bind 127.0.0.1:8000 --user altair --pass altair-local-dev file://./altair.db &
          sleep 5

      - name: Integration Tests
        run: |
          cd packages/altair-core
          flutter pub get
          flutter test test/repositories/task_repository_integration_test.dart
```

## Performance Considerations

### Caching

Cache dependencies to speed up CI:

```yaml
- uses: actions/cache@v3
  with:
    path: |
      ~/.pub-cache
      ${{ runner.tool_cache }}/flutter
    key: ${{ runner.os }}-pub-${{ hashFiles('**/pubspec.lock') }}
```

### Parallel Jobs

Run checks in parallel when possible:

```yaml
jobs:
  format:
    runs-on: ubuntu-latest
    # ... format checks

  analyze:
    runs-on: ubuntu-latest
    # ... analysis checks

  test:
    runs-on: ubuntu-latest
    # ... unit tests
```

## Migration from Pre-commit Only

**Current state**: Pre-commit hooks provide local validation
**Next step**: Add GitHub Actions for automated PR/merge checks

Benefits:

1. **Consistent environment** - All developers and CI use same checks
2. **Automated enforcement** - Can't merge PRs with failing checks
3. **Faster feedback** - CI runs on every push
4. **Platform coverage** - Test on Linux, macOS, Windows

## Testing the CI Pipeline

Before deploying to production CI:

1. **Run pre-commit hooks locally**:

   ```bash
   pre-commit run --all-files
   ```

2. **Test database service**:

   ```bash
   ./scripts/start-db.sh
   cd packages/altair-core
   flutter test test/repositories/task_repository_integration_test.dart
   ```

3. **Verify all packages**:

   ```bash
   ./scripts/pre-commit/flutter-analyze.sh
   ```

## Questions?

For CI/CD setup assistance:

- Check existing scripts in `scripts/pre-commit/`
- Review `.pre-commit-config.yaml` for current checks
- See `packages/altair-db-service/DEVELOPMENT.md` for database setup

---

**Last Updated**: 2025-01-27
**Maintained By**: Altair Development Team
