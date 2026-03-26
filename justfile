# Default recipe: list available commands
default:
    @just --list

# ─── Setup ───────────────────────────────────────────────────

# Install all dependencies
setup:
    pnpm install
    cd apps/server && cargo check

# ─── Development ─────────────────────────────────────────────

# Start infrastructure (Postgres, MinIO, PowerSync)
infra-up:
    docker compose -f infra/compose/docker-compose.yml up -d

# Stop infrastructure
infra-down:
    docker compose -f infra/compose/docker-compose.yml down

# Start backend dev server
dev-server:
    cd apps/server && cargo run

# Start web dev server
dev-web:
    cd apps/web && pnpm dev

# Start Android dev (just opens the project — actual dev is in Android Studio)
dev-android:
    @echo "Open apps/android/ in Android Studio"

check-android:
    cd apps/android && ./gradlew build

# ─── Contracts ───────────────────────────────────────────────

# Generate typed constants from contract registries
codegen:
    node packages/contracts/codegen.mjs

# ─── Checks ─────────────────────────────────────────────────

# Run all checks
check: check-server check-web

check-server:
    cd apps/server && cargo check && cargo clippy

check-web:
    cd apps/web && pnpm check
