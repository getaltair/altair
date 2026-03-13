# Server Development Guide

## Prerequisites

- Rust toolchain (via [rustup](https://rustup.rs/))
- Docker & Docker Compose (for local Postgres)
- sqlx-cli:

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

## Database Setup

The server uses the Postgres instance defined in `apps/web/compose.yaml`.

```bash
# Start Postgres (from apps/web/)
docker compose up -d db

# Copy environment config (from apps/server/)
cp .env.example .env
```

sqlx-cli automatically loads `DATABASE_URL` from `.env`.

## Migration Commands

All commands run from `apps/server/`.

### Create a new migration

```bash
# Reversible migration (creates .up.sql and .down.sql)
sqlx migrate add -r <name>
```

This creates timestamped files in `migrations/`:

```
migrations/
├── YYYYMMDDHHMMSS_<name>.up.sql
└── YYYYMMDDHHMMSS_<name>.down.sql
```

### Apply migrations

```bash
# Run all pending migrations
sqlx migrate run

# Check migration status
sqlx migrate info
```

### Revert migrations

```bash
# Revert the most recent migration
sqlx migrate revert
```

### Reset database

```bash
# Drop and recreate the database
sqlx database drop
sqlx database create
sqlx migrate run
```

## Offline Mode (CI)

sqlx can compile-check queries without a live database by caching query metadata.

### Generate offline cache

```bash
# Requires a running database with current schema
cargo sqlx prepare --workspace
```

This creates a `.sqlx/` directory with cached query data. Commit `.sqlx/` to the repo.

### Build without a database

```bash
SQLX_OFFLINE=true cargo build
```

### Verify cache freshness in CI

```bash
cargo sqlx prepare --check --workspace
```

This fails if the cached metadata doesn't match the current queries — useful as a CI gate.

## Workflow Summary

1. Start Postgres: `docker compose up -d db` (from `apps/web/`)
2. Create migration: `sqlx migrate add -r <name>`
3. Write SQL in the generated `.up.sql` and `.down.sql` files
4. Apply: `sqlx migrate run`
5. Verify: `sqlx migrate info`
6. Update offline cache: `cargo sqlx prepare --workspace`
7. Commit migration files and `.sqlx/` directory
