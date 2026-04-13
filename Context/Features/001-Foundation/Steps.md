# Implementation Steps: Foundation

**Spec:** `Context/Features/001-Foundation/Spec.md`
**Tech:** `Context/Features/001-Foundation/Tech.md`

---

## Progress

- **Status:** Complete
- **Current task:** —
- **Last milestone:** M7 (FINAL) + Phase 8 post-review tasks complete (2026-04-13)

---

## Team Orchestration

### Team Members

- **builder-rust**
  - Role: Rust workspace scaffolding, server skeleton, health endpoint, sqlx offline setup
  - Agent Type: backend-engineer
  - Resume: true

- **builder-web**
  - Role: SvelteKit scaffolding, Tailwind setup, OIDC redirect wiring
  - Agent Type: frontend-specialist
  - Resume: true

- **builder-android**
  - Role: Android project scaffold and initial Compose dependencies
  - Agent Type: general-purpose
  - Resume: false

- **builder-infra**
  - Role: Docker Compose stack, Postgres init, migrations, seed data, Zitadel setup script, CI pipeline
  - Agent Type: backend-engineer
  - Resume: true

- **validator**
  - Role: Quality validation — read-only inspection of completed work against Spec assertions
  - Agent Type: quality-engineer
  - Resume: false

---

## Tasks

### Phase 1: Repo Skeleton

- [ ] S001: Update `mise.toml` with toolchain pins (Bun stable, `java temurin-17`); create `packages/contracts/README.md` placeholder describing future contents (entity-types.json, relation-types.json, sync-streams.json, TypeScript/Kotlin/Rust bindings)
  - **Assigned:** builder-infra
  - **Depends:** none
  - **Parallel:** false

🏁 **MILESTONE 1: Repo skeleton ready**
Verify: `packages/contracts/README.md` exists; `mise.toml` has Bun and Java pins.
**Contracts:**
- `mise.toml` — toolchain pins consumed by all CI jobs
- `packages/contracts/README.md` — placeholder consumed by Step 2 planning

---

### Phase 2: App Scaffolding (parallel)

- [ ] S002: Scaffold Rust Cargo workspace at `apps/server/`. Create workspace `Cargo.toml` listing members `["server", "worker"]`. Inside `apps/server/`, run `cargo init --name altair-server server` and `cargo init --name altair-worker worker`. Add `apps/server/rust-toolchain.toml` pinning the `stable` channel to the current stable release.
  - **Assigned:** builder-rust
  - **Depends:** S001
  - **Parallel:** true

- [ ] S003: Scaffold SvelteKit app at `apps/web/` by running `bun create svelte@latest apps/web`. Select: skeleton project, TypeScript, no additional tools (added manually in S008). Verify `bun run dev` starts.
  - **Assigned:** builder-web
  - **Depends:** S001
  - **Parallel:** true

- [ ] S004: Scaffold Android project at `apps/android/`. **This task requires running Android Studio interactively.** Settings: Language = Kotlin, Min SDK = API 26 (Android 8.0), Template = Empty Activity (Jetpack Compose), Build system = Gradle with Kotlin DSL. Commit the full result including `gradlew`, `gradlew.bat`, and `gradle/wrapper/`. Verify `./gradlew build` succeeds from `apps/android/`.
  - **Assigned:** builder-android
  - **Depends:** S001
  - **Parallel:** true
  - **Note:** No CLI equivalent for full Compose project bootstrapping (IC-4). Android Studio wizard is the canonical tool.

- [ ] S005: Create `infra/compose/docker-compose.yml` skeleton with all five services declared (postgres, postgres-init, zitadel, mongodb, powersync, garage) and correct `depends_on` + healthcheck stubs. Create `infra/compose/.env.example` with all required variable names and placeholder values. Create `infra/compose/powersync.yml` config template with `${VARIABLE}` placeholders for all credentials. No secrets in any committed file (SEC-5).
  - **Assigned:** builder-infra
  - **Depends:** S001
  - **Parallel:** true

🏁 **MILESTONE 2: All stacks scaffolded**
Verify: S002–S005 all complete. `cargo build` compiles (FA-001 partial — no deps yet). `bun run dev` starts (FA-002 partial). `./gradlew build` succeeds (FA-003). `docker compose config` validates without errors.

---

### Phase 3: Dependencies and Initial Code

- [ ] S006: Add Rust server dependencies via `cargo add` (run from `apps/server/server/`):
  `tokio --features full`, `axum`, `tower`, `tower-http --features cors,trace`, `serde --features derive`, `serde_json`, `sqlx --features postgres,uuid,chrono,runtime-tokio-native-tls`, `tracing`, `tracing-subscriber --features env-filter`, `dotenvy`, `uuid --features v4`, `chrono --features serde`, `thiserror`, `anyhow`.
  Dev deps: `tokio-test`.
  Implement initial `src/main.rs` (tokio entrypoint, bind addr from env), `src/config.rs` (Config struct from env), `src/error.rs` (AppError stub), `src/routes/health.rs` (`GET /health` → 200 `{"status":"ok"}`).
  - **Assigned:** builder-rust
  - **Depends:** S002
  - **Parallel:** false

- [ ] S006-T: Test health endpoint (`GET /health` returns 200 with `{"status":"ok"}`; server binds to configured port; invalid route returns 404; missing `DATABASE_URL` env var produces a startup error, not a panic)
  - **Assigned:** builder-rust
  - **Depends:** S006

- [ ] S007: Add worker binary entry point in `apps/server/worker/src/main.rs` (logs "altair-worker started" and exits 0 — stub only). Run `cargo sqlx prepare --workspace` from `apps/server/` against the running Compose Postgres to generate the `.sqlx/` cache directory. Commit `.sqlx/`. Add `SQLX_OFFLINE=true` note to `.env.example`.
  - **Assigned:** builder-rust
  - **Depends:** S006
  - **Parallel:** false
  - **Note:** Requires Compose Postgres running locally to generate `.sqlx/` cache. `cargo sqlx prepare` must be re-run whenever a new `sqlx::query!` macro is added.

- [ ] S008: From `apps/web/`, run `bun add -d tailwindcss @tailwindcss/vite`. Configure Tailwind in `vite.config.ts`. Add `@tailwindcss/vite` plugin. Replace default `+page.svelte` with a minimal placeholder: "Altair — coming soon." Verify `bun run check` passes and `bun run build` completes.
  - **Assigned:** builder-web
  - **Depends:** S003
  - **Parallel:** true

- [ ] S009: In `apps/android/`, add initial Jetpack Compose and Koin dependencies via the `libs.versions.toml` version catalog and `build.gradle.kts` `dependencies {}` block:
  `androidx.compose.ui:ui`, `androidx.compose.material3:material3`, `androidx.navigation:navigation-compose`, `androidx.lifecycle:lifecycle-viewmodel-compose`, `io.insert-koin:koin-android`, `io.insert-koin:koin-androidx-compose`.
  Sync Gradle and verify `./gradlew build` still passes.
  - **Assigned:** builder-android
  - **Depends:** S004
  - **Parallel:** true

🏁 **MILESTONE 3: All stacks compile with dependencies**
Verify FA-001 (`cargo build` clean), FA-002 (`bun run dev` starts), FA-003 (`./gradlew build` passes).
**Contracts:**
- `apps/server/server/src/routes/health.rs` — health route used by smoke-test CI job
- `apps/server/server/src/config.rs` — Config struct; DATABASE_URL field consumed by migrations task
- `apps/web/vite.config.ts` — Tailwind config consumed by web CI job
- `.sqlx/` — sqlx offline cache; must be present before CI rust job runs with `SQLX_OFFLINE=true`

---

### Phase 4: Infra — Compose Stack and Migrations

- [ ] S010: Complete `infra/compose/docker-compose.yml`:
  - `postgres`: `postgres:16-alpine`, single container, creates both `altair_db` and `zitadel_db`. Healthcheck: `pg_isready -U ${POSTGRES_USER}`.
  - `postgres-init`: one-shot `postgres:16-alpine` sidecar that runs `createdb zitadel_db` after Postgres healthy. `restart: no`.
  - `zitadel`: `ghcr.io/zitadel/zitadel:latest`. Depends on `postgres-init`. Configured with `--masterkey` from env, `--db postgres` pointing at `zitadel_db`. Exposes port 8080.
  - `mongodb`: `mongo:7`. No auth required for local dev. Healthcheck: `mongosh --eval "db.runCommand({ping:1})"`.
  - `powersync`: `public.ecr.aws/powersync/powersync-service:latest`. Mounts `./powersync.yml`. Depends on `mongodb` and `postgres`.
  - `garage`: `dxflrs/garage:v1`. S3-compatible storage. Exposes ports 3900 (API) and 3902 (admin).
  All credentials from `.env` via `env_file`. Pin each image to a specific digest or version tag. Update `.env.example` with all new variables.
  - **Assigned:** builder-infra
  - **Depends:** S005
  - **Parallel:** false

- [ ] S011: Write Zitadel setup script `infra/scripts/zitadel-setup.sh`. Uses Zitadel management REST API (curl). Steps: (1) wait for Zitadel ready at `/debug/healthz/ready`; (2) create organisation `Altair`; (3) create project `altair-server`; (4) create web OIDC application with PKCE, redirect URI `http://localhost:5173/auth/callback`, post-logout URI `http://localhost:5173`; (5) create service user for PowerSync JWT validation; (6) print generated client ID and JWKS URL to stdout for copy to `.env`. Script is idempotent — safe to re-run. Update `.env.example` with `ZITADEL_CLIENT_ID` and `ZITADEL_JWKS_URL` variable names.
  - **Assigned:** builder-infra
  - **Depends:** S010
  - **Parallel:** true

- [ ] S012: Write Phase 1 sqlx migrations in `infra/migrations/` (naming: `YYYYMMDDHHMMSS_description.sql`). Four files:
  1. `20260412000001_create_updated_at_trigger.sql` — PL/pgSQL trigger function `set_updated_at()` + `-- migrate:down` drops it.
  2. `20260412000002_create_users.sql` — `users` table with `oidc_sub TEXT UNIQUE NOT NULL` (not `password_hash`; per ADR-006 and Tech.md Decision 5). Apply `set_updated_at` trigger. Include `-- migrate:down`.
  3. `20260412000003_create_households.sql` — `households` table with `owner_id FK → users`. Apply trigger. Include `-- migrate:down`.
  4. `20260412000004_create_household_memberships.sql` — `household_memberships` table, UNIQUE on `(household_id, user_id)`. Apply trigger. Include `-- migrate:down`.
  Columns match `05-erd.md` exactly except `users.password_hash` → `users.oidc_sub` (Tech.md Decision 5).
  - **Assigned:** builder-infra
  - **Depends:** S010
  - **Parallel:** true

- [ ] S012-T: Verify migrations apply cleanly (`sqlx migrate run`), all tables and columns exist per schema, `updated_at` trigger fires on UPDATE, rollback (`sqlx migrate revert`) cleanly removes all tables. (FA-005, FA-006, FA-007, FA-010)
  - **Assigned:** builder-infra
  - **Depends:** S012

- [ ] S013: Write `infra/scripts/seed.sql`. Inserts one dev user record into `users` with a known `oidc_sub` value matching the local Zitadel dev setup (e.g. `oidc_sub = 'dev-user-001'`), email `dev@altair.local`, display name `Dev User`. Seed is idempotent via `INSERT ... ON CONFLICT DO NOTHING`. Verify `SELECT count(*) FROM users` returns ≥ 1 after applying. (FA-008)
  - **Assigned:** builder-infra
  - **Depends:** S012
  - **Parallel:** false

🏁 **MILESTONE 4: Compose stack up and migrations applied**
Verify FA-004 (`docker compose up` — all services healthy), FA-005 (`sqlx migrate run` exit 0), FA-006 (`users` table with correct columns), FA-007 (`households` and `household_memberships` tables exist), FA-008 (seed produces ≥ 1 user), FA-010 (rollback cleans up).
**Contracts:**
- `infra/compose/docker-compose.yml` — service names and ports consumed by CI smoke-test job
- `infra/compose/.env.example` — variable reference consumed by smoke-test setup and README
- `infra/migrations/` — migration files consumed by smoke-test `sqlx migrate run` step
- `infra/scripts/seed.sql` — consumed by smoke-test seed verification step
- `infra/scripts/zitadel-setup.sh` — consumed by OIDC wiring task (S016)

---

### Phase 5: CI Pipeline

- [ ] S014: Write `.github/workflows/ci.yml` with four jobs:
  - `rust`: `ubuntu-latest`; uses `dtolnay/rust-toolchain@stable`; caches `~/.cargo` and `apps/server/target`; runs `cargo build`, `cargo clippy -- -D warnings`, `cargo sqlx prepare --check`, `cargo test` (all from `apps/server/`). Sets `SQLX_OFFLINE=true`.
  - `web`: `ubuntu-latest`; uses `oven-sh/setup-bun`; runs `bun install`, `bun run check`, `bun run build` (from `apps/web/`).
  - `android`: `ubuntu-latest`; uses JDK 17 (`actions/setup-java`); caches Gradle; runs `./gradlew build lint` (from `apps/android/`).
  - `smoke-test`: `ubuntu-latest`; `needs: [rust, web]`; starts Compose stack (`docker compose -f infra/compose/docker-compose.yml up -d`); waits for all services healthy (poll `/debug/healthz/ready` for Zitadel, `pg_isready` for Postgres); runs `sqlx migrate run`; applies `infra/scripts/seed.sql`; hits `GET http://localhost:PORT/health` and asserts 200; runs `sqlx migrate revert`; tears down stack.
  Trigger: push and PR to `main`.
  - **Assigned:** builder-infra
  - **Depends:** S007, S008, S009, S013
  - **Parallel:** false

- [ ] S014-D: Update `README.md` — add CI badge, local dev setup section (prerequisites, `mise install`, `docker compose up`, `sqlx migrate run`, per-app start commands), and a note pointing to `infra/compose/.env.example` for environment setup.
  - **Assigned:** builder-infra
  - **Depends:** S014

🏁 **MILESTONE 5: CI pipeline green**
Verify FA-011 (CI runs and passes on push to `main`), FA-012 (`git grep` for known secret patterns returns no matches).
**Contracts:**
- `.github/workflows/ci.yml` — CI job structure consumed by S016 OIDC verification step
- `README.md` — local dev instructions consumed by onboarding

---

### Phase 6: OIDC Wiring

- [ ] S015: Run `infra/scripts/zitadel-setup.sh` against the local Compose stack to create the web OIDC application and service user. Copy the generated `ZITADEL_CLIENT_ID` and `ZITADEL_JWKS_URL` into the local `.env` (gitignored). Wire basic OIDC redirect in `apps/web/`: add a `/auth/login` route that redirects to the Zitadel authorization endpoint (PKCE flow, hardcoded client ID from env), and a `/auth/callback` route that receives the authorization code and displays the returned `access_token` in the browser (dev-only — no storage yet; storage and full auth flow implemented in Step 3 and Step 9). Verify the end-to-end login flow works locally against the Compose stack. (FA-009)
  - **Assigned:** builder-web
  - **Depends:** S011, S014
  - **Parallel:** false
  - **Note:** FA-009 is a manual verification step — automated E2E OIDC testing deferred to Step 9 (Web Client).

🏁 **MILESTONE 6: OIDC login works end-to-end**
Verify FA-009 (manual: SvelteKit → Zitadel login → callback → access token displayed).

---

### Phase 7: Validation

- [ ] S016: Full validation pass — verify all FA assertions against the completed scaffold:
  FA-001: `cargo build` in clean environment.
  FA-002: `bun run dev` starts and serves on expected port.
  FA-003: `./gradlew build` passes.
  FA-004: `docker compose up` — all services healthy.
  FA-005: `sqlx migrate run` exits 0.
  FA-006–FA-007: Table existence and column checks via psql.
  FA-008: `SELECT count(*) FROM users` ≥ 1 after seeding.
  FA-009: OIDC login end-to-end (manual verification).
  FA-010: `sqlx migrate revert` cleans up tables.
  FA-011: CI pipeline passes on `main`.
  FA-012: `git grep` for secrets returns no matches.
  Flag any test failures, missing contracts, or TODO stubs.
  - **Assigned:** validator
  - **Depends:** S015
  - **Parallel:** false

🏁 **MILESTONE 7 (FINAL): Foundation complete**
Verify all FA-001 through FA-012. All stacks compile. Full Compose stack runs. OIDC login works. CI is green. No secrets committed.

---

### Phase 8: Post-Review Tasks (from PR review P1)

- [x] S017: Install Vitest in `apps/web/` — add `vitest` and `@vitest/ui` as dev dependencies, add `happy-dom` for Web Crypto API support. Add `test` and `test:run` scripts to `package.json`. Verify `bun run test:run` executes with zero failures on an empty suite.
  - **Source:** P1-005 (Critical — no tests can be written or run without this)
  - **Depends:** S015

- [x] S017-T: PKCE unit tests — create `apps/web/src/lib/auth/pkce.spec.ts`. Assert: output length of `generateCodeVerifier` is 86 chars; character set matches `[A-Za-z0-9\-_]` only; known-input/known-output round-trip through `generateCodeChallenge`. Gated on S017.
  - **Source:** P1-006 (Critical — base64url correctness unverified)
  - **Depends:** S017

- [x] S018-T: `AppError::IntoResponse` tests — add to `apps/server/server/src/error.rs` `#[cfg(test)]` module using the `tower::ServiceExt::oneshot` pattern from `routes/health.rs`. Cover: `NotFound` → 404, `Internal` → 500 with no internal message leaked in body.
  - **Source:** P1-014 (High — HTTP status contract for all clients is untested)
  - **Depends:** S006

- [x] S019-T: `Config::from_env` happy-path tests — add to `apps/server/server/src/config.rs` tests: happy path returns `Ok` with correct field values; `BIND_ADDR` env var absent falls back to `"0.0.0.0:8000"`. Use `serial_test` crate or `-- --test-threads=1` to serialize env-mutating tests.
  - **Source:** P1-022 (Low — `BIND_ADDR` default and happy-path regressions undetected)
  - **Depends:** S006

---

## Acceptance Criteria

- [ ] FA-001: `cargo build` succeeds for server crate
- [ ] FA-002: `bun run dev` starts SvelteKit dev server
- [ ] FA-003: `./gradlew build` succeeds for Android project
- [ ] FA-004: `docker compose up` — all five services report healthy
- [ ] FA-005: `sqlx migrate run` applies without errors
- [ ] FA-006: `users` table has correct columns including `oidc_sub` (not `password_hash`)
- [ ] FA-007: `households` and `household_memberships` tables exist
- [ ] FA-008: Seed produces ≥ 1 user record
- [ ] FA-009: OIDC login flow from SvelteKit to Zitadel returns a valid access token
- [ ] FA-010: `sqlx migrate revert` removes all created tables
- [ ] FA-011: CI pipeline passes on push to `main`
- [ ] FA-012: No secrets in any committed file
- [ ] All tests in S006-T and S012-T passing
- [ ] No TODO/FIXME stubs remaining in S006 implementation
- [ ] README.md updated with local dev setup instructions

---

## Validation Commands

```bash
# FA-001: Rust build
cd apps/server && cargo build

# FA-002: Web dev server
cd apps/web && bun run dev &
curl -f http://localhost:5173 && kill %1

# FA-003: Android build
cd apps/android && ./gradlew build

# FA-004: Compose stack health
cd infra/compose && docker compose up -d
docker compose ps  # all services: healthy

# FA-005 + FA-006 + FA-007: Migrations
DATABASE_URL=postgres://altair_user:${POSTGRES_PASSWORD}@localhost:5432/altair_db \
  sqlx migrate run --source infra/migrations

psql $DATABASE_URL -c "\d users"          # FA-006: verify oidc_sub column present
psql $DATABASE_URL -c "\dt"               # FA-007: verify households, household_memberships

# FA-008: Seed
psql $DATABASE_URL -f infra/scripts/seed.sql
psql $DATABASE_URL -c "SELECT count(*) FROM users"  # ≥ 1

# FA-009: OIDC (manual — open browser)
# Navigate to http://localhost:5173/auth/login
# Complete Zitadel login
# Verify /auth/callback displays access_token

# FA-010: Migration rollback
DATABASE_URL=... sqlx migrate revert --source infra/migrations
psql $DATABASE_URL -c "\dt"  # no tables remaining

# FA-011: CI
git push origin feat/scaffold  # observe GitHub Actions

# FA-012: Secret scan
git grep -rE "(password|secret|key)\s*=\s*['\"][^$\{]" -- . ':!*.example' ':!*.md'
```
