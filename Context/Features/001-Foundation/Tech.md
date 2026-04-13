# Tech Plan: Foundation

**Spec:** `Context/Features/001-Foundation/Spec.md`
**Stacks involved:** Rust/Axum, SvelteKit 2/Svelte 5, Kotlin/Android, PostgreSQL, Docker Compose, GitHub Actions

---

## Architecture Overview

Foundation produces no user-facing behaviour — it is the substrate. By the end of this step every stack compiles, every service runs, migrations are applied, and OIDC login works end-to-end. All subsequent steps build on top of this.

The monorepo layout follows ADR-007 exactly:

```
altair/
├── apps/
│   ├── server/          ← Cargo workspace (server + worker binaries)
│   ├── web/             ← Bun / SvelteKit 2 / Svelte 5
│   └── android/         ← Kotlin / Jetpack Compose / Gradle
├── packages/
│   └── contracts/       ← Placeholder README only (populated in Step 2)
├── infra/
│   ├── compose/         ← docker-compose.yml + .env.example
│   ├── migrations/      ← sqlx numbered SQL files
│   └── scripts/         ← seed.sql
├── .github/
│   └── workflows/
│       └── ci.yml
└── mise.toml
```

No top-level Bun workspace. Each app is independent. CI runs per-stack jobs in parallel with a shared smoke-test job that depends on all of them.

---

## Key Decisions

### Decision 1: Cargo workspace location

**Options considered:**
- **Option A: Repo root** — `Cargo.toml` at repo root with `[workspace]` listing `apps/server` and any future Rust crates. Common for monorepos with multiple Rust crates.
- **Option B: `apps/server/`** — Self-contained Cargo workspace inside the server directory, consistent with ADR-007's "each app manages its own build toolchain" principle.

**Chosen:** Option B — `apps/server/`

**Rationale:** ADR-007 explicitly states "each app manages its own build toolchain" and there are no other Rust crates in v1. A root-level `Cargo.toml` would suggest Rust is privileged at the top level, breaking the structural symmetry with Bun and Gradle. If a future Rust worker or CLI is needed, it slots in as a workspace member inside `apps/server/` without touching the repo root.

**Workspace layout inside `apps/server/`:**
```
apps/server/
├── Cargo.toml           ← workspace manifest; members = ["server", "worker"]
├── server/
│   ├── Cargo.toml       ← [[bin]] crate: altair-server
│   └── src/
│       └── main.rs
└── worker/
    ├── Cargo.toml       ← [[bin]] crate: altair-worker
    └── src/
        └── main.rs
```

**Related ADRs:** ADR-007

---

### Decision 2: Zitadel — shared vs dedicated PostgreSQL

**Options considered:**
- **Option A: Separate Postgres container for Zitadel** — Dedicated `postgres-zitadel` container alongside `postgres-altair`. Clean isolation; independent upgrade paths. Adds ~200MB RAM and one more container.
- **Option B: Single Postgres container, two databases** — One `postgres` container hosts both `altair_db` and `zitadel_db`. Zitadel's `--db` config points at the `zitadel_db` database. Saves ~200MB RAM on minimum hardware (ADR-002: 4GB target is already tight).
- **Option C: External Postgres only (no containerised DB)** — Requires user to provision their own Postgres; too much friction for self-hosting.

**Chosen:** Option B — single Postgres container, two logical databases

**Rationale:** ADR-002 puts the minimum deployment target at 4GB, where every ~200MB matters. Two logical databases inside one container achieves full schema isolation (Zitadel manages its own migrations; Altair migrations never touch `zitadel_db`) while staying within the memory budget. The Compose `depends_on` healthcheck ensures Altair's migrations only run after Postgres is healthy.

**Compose service names:** `postgres` (shared), `altair_db` (Altair migrations target), `zitadel_db` (Zitadel migration target).

**Related ADRs:** ADR-002, ADR-006

---

### Decision 3: PowerSync container configuration

**Options considered:**
- **Option A: Environment variables only** — All PowerSync config as `environment:` entries in Compose. Simple, but sync rules require structured YAML that cannot be expressed as flat env vars.
- **Option B: YAML config file + env vars for secrets** — `infra/compose/powersync.yml` defines sync rules, database connection, and JWT config. Credentials (DB password, JWT secret) injected via env vars. Config file mounted as a volume.

**Chosen:** Option B — YAML config file with env var secrets

**Rationale:** PowerSync's sync rules definition (bucket parameters, table mappings) is a structured document — it cannot be expressed as flat env vars. Separating structure (config file) from secrets (env vars) also satisfies invariant SEC-5 (no hardcoded secrets). The config file is committed to source control with `${VARIABLE}` placeholders; the `.env` file supplies the values locally and CI supplies them via Actions secrets.

**Files:**
- `infra/compose/powersync.yml` — PowerSync config template (committed)
- `infra/compose/.env.example` → `.env` (gitignored, docs-only skeleton committed)

**Note for Step 1:** The PowerSync config must be present for the container to start, but sync rules themselves (stream definitions) are defined in Step 4. Step 1 provides a minimal valid config with no stream definitions — enough to pass the Docker Compose smoke test (FA-004).

**Related ADRs:** ADR-003

---

### Decision 4: Minimum Zitadel configuration scope for Step 1

**Options considered:**
- **Option A: Web + Android OIDC applications** — Configure both clients upfront. Avoids revisiting Zitadel config in Step 8.
- **Option B: Web-only OIDC application** — Configure just enough to get web login working (assertion FA-009). Android redirect URIs added in Step 8 when the Android client actually exists.

**Chosen:** Option B — web-only for Step 1

**Rationale:** Android doesn't exist yet. Configuring Android AppAuth redirect URIs now requires knowing the final package name and schemes before the Android project is even scaffolded. Deferring avoids premature binding. Zitadel config is additive — adding an Android application in Step 8 takes minutes.

**Step 1 Zitadel setup (scripted in `infra/scripts/zitadel-setup.sh`):**
1. Create organisation: `Altair`
2. Create project: `altair-server`
3. Create web application (OIDC, PKCE, confidential):
   - Client ID documented in `.env.example`
   - Redirect URI: `http://localhost:5173/auth/callback` (SvelteKit dev)
   - Post-logout URI: `http://localhost:5173`
4. Create service user for machine-to-machine (PowerSync JWT validation)
5. Output config values to stdout for copy-paste into `.env`

This script is idempotent via Zitadel's management API and can be re-run on a fresh stack.

**Related ADRs:** ADR-006

---

### Decision 5: users table schema — password_hash vs OIDC identity

**Options considered:**
- **Option A: Keep `password_hash` per ERD (`05-erd.md`)** — Aligns with the ERD document written before ADR-006.
- **Option B: Replace `password_hash` with `oidc_sub`** — Reflects the accepted ADR-006 decision: Zitadel owns credentials; Altair stores only the OIDC subject identifier.

**Chosen:** Option B — `oidc_sub` replaces `password_hash`

**Rationale:** ADR-006 is accepted and supersedes the ERD for auth design. Altair's Axum server is a relying party, not an identity provider. It never sees passwords. The `users` table is an application profile store — linked to Zitadel via the `sub` claim from the JWT access token. The ERD must be considered an outdated draft for this column.

**`users` table schema for initial migration:**

| Column | Type | Notes |
|---|---|---|
| `id` | UUID PK | Client-generated |
| `oidc_sub` | TEXT UNIQUE NOT NULL | OIDC subject identifier (`sub` claim from JWT) |
| `email` | VARCHAR(255) UNIQUE NOT NULL | From OIDC ID token `email` claim |
| `display_name` | VARCHAR(100) NOT NULL | From OIDC `name` or `preferred_username` claim |
| `created_at` | TIMESTAMPTZ NOT NULL | `DEFAULT now()` |
| `updated_at` | TIMESTAMPTZ NOT NULL | Trigger-maintained |
| `deleted_at` | TIMESTAMPTZ NULL | Soft delete |

**Follow-up required:** The ERD document (`05-erd.md`) should be updated to remove `password_hash` and add `oidc_sub`. Tracked as an open question resolved by this decision.

**Related ADRs:** ADR-006

---

### Decision 6: CI job structure

**Options considered:**
- **Option A: Single monolithic job** — All stacks run sequentially in one job. Simple YAML but slow (Android builds alone can take 5–10 minutes) and fails atomically (one stack failure masks others).
- **Option B: Per-stack parallel jobs + integration smoke test** — Separate jobs for Rust, Web, Android. Smoke test job depends on all three. Parallelism reduces wall-clock time; per-stack failures are independently reported.

**Chosen:** Option B — parallel per-stack jobs

**Rationale:** ADR-007 explicitly notes "CI will need per-app build pipelines (not a single unified build)." Parallel jobs cut total CI time and surface failures in isolation. The smoke test (Docker Compose up + `sqlx migrate run` + seed data + health check) runs only after all build jobs pass.

**CI matrix:**

| Job | Triggers | Steps |
|---|---|---|
| `rust` | push/PR to main | `cargo build`, `cargo clippy -- -D warnings`, `cargo test` |
| `web` | push/PR to main | `bun install`, `bun run check`, `bun run build` |
| `android` | push/PR to main | `./gradlew build lint` |
| `smoke-test` | after rust + web pass | Docker Compose up, wait healthy, `sqlx migrate run`, seed, health check |

Android job runs on `ubuntu-latest` with JDK 17; uses Gradle caching. Rust job uses `actions/cache` on `~/.cargo` and `target/`. Web job uses `bun/setup-bun` action.

---

### Decision 7: sqlx compile-time query checking mode

**Options considered:**
- **Option A: Always require live DB** — `sqlx` validates queries against a running Postgres at compile time. Most accurate but requires DB in every CI job, including Rust compilation.
- **Option B: Offline mode (`sqlx prepare`)** — Developer runs `cargo sqlx prepare` to generate `.sqlx/` cache. CI uses `SQLX_OFFLINE=true` for compilation. Live DB only in the dedicated smoke-test job.

**Chosen:** Option B — offline mode for compilation, live DB in smoke test

**Rationale:** The `rust` CI job should compile quickly without a Postgres dependency. Offline mode caches query metadata in `.sqlx/` (committed to source control). The smoke test job handles real DB validation via `sqlx migrate run`. This matches the Rust conventions in `.claude/rules/rust-axum.md` and avoids ordering dependencies between the `rust` and `smoke-test` CI jobs.

**Requirement:** `.sqlx/` directory is committed. CI sets `SQLX_OFFLINE=true` in the `rust` job.

---

## Stack-Specific Details

### Rust / Axum (`apps/server/`)

**Scaffold command:**
```bash
mkdir -p apps/server && cd apps/server
cargo init --name altair-server server
cargo init --name altair-worker worker
# Create workspace Cargo.toml manually (workspace manifest, not a crate)
```

**Initial server dependencies (via `cargo add` inside `apps/server/server/`):**

| Crate | Purpose |
|---|---|
| `tokio` (features: full) | Async runtime |
| `axum` | Web framework |
| `tower` | Middleware |
| `tower-http` (features: cors, trace) | HTTP middleware |
| `serde` (features: derive) | Serialization |
| `serde_json` | JSON |
| `sqlx` (features: postgres, uuid, chrono, runtime-tokio-native-tls) | DB access + migrations |
| `tracing` | Structured logging |
| `tracing-subscriber` | Log formatting |
| `dotenvy` | `.env` loading |
| `uuid` (features: v4) | UUID generation |
| `chrono` (features: serde) | DateTime types |
| `thiserror` | Error types |
| `anyhow` | Application-level error handling |

**Dev dependencies:** `tokio-test`

**Initial structure (Step 1 only — skeleton, no domain logic):**
```
server/src/
├── main.rs          ← tokio::main, router assembly, bind
├── config.rs        ← Config struct from env vars
├── error.rs         ← AppError enum (stub)
└── routes/
    └── health.rs    ← GET /health → 200 OK
```

**Worker binary (`apps/server/worker/`):** Empty main that prints "worker started" and exits. Fleshed out in Step 11.

**Rust toolchain:** Pinned via `apps/server/rust-toolchain.toml` (`stable` channel, specific version). This pin also covers CI via `dtolnay/rust-toolchain` action.

---

### SvelteKit / Svelte 5 (`apps/web/`)

**Scaffold command:**
```bash
bun create svelte@latest apps/web
# Select: SvelteKit skeleton, TypeScript, no additional tools (added manually)
```

**Post-scaffold additions via `bun add`:**

| Package | Type | Purpose |
|---|---|---|
| `@sveltejs/adapter-auto` | dev | Adapter (switched to `adapter-static` in Step 9 for prod) |
| `tailwindcss` | dev | Utility CSS |
| `@tailwindcss/vite` | dev | Tailwind Vite plugin |

**Note:** PowerSync web SDK added in Step 4. Auth client library (OIDC) added in Step 9. Step 1 scaffolds a working skeleton only.

**Step 1 state:** Default SvelteKit skeleton with `bun run dev` working, `/` route returning a placeholder page, Tailwind wired up.

---

### Kotlin / Android (`apps/android/`)

**Scaffold approach:** Android Studio project wizard creates the initial project. This is the canonical tool for Android project creation — no CLI equivalent for full Gradle/Compose bootstrapping. The resulting project (including `gradlew` wrapper) is committed to source control.

**Minimum project configuration:**
- Language: Kotlin
- Minimum SDK: API 26 (Android 8.0) — consistent with `09-PLAT-001-android.md`
- Template: Empty Activity (Jetpack Compose)
- Build system: Gradle with Kotlin DSL (`build.gradle.kts`)

**Initial dependencies (added to `build.gradle.kts` via Android Studio's dependency management or `libs.versions.toml`):**

| Dependency | Purpose |
|---|---|
| `androidx.compose.ui:ui` | Compose UI |
| `androidx.compose.material3:material3` | Material 3 components |
| `androidx.navigation:navigation-compose` | Navigation |
| `androidx.lifecycle:lifecycle-viewmodel-compose` | ViewModel |
| `io.insert-koin:koin-android` | DI |
| `io.insert-koin:koin-androidx-compose` | Koin Compose integration |

**Note:** Room, PowerSync Kotlin SDK, AppAuth added in Steps 4 and 8.

---

### Docker Compose (`infra/compose/`)

**File:** `infra/compose/docker-compose.yml`

**Services:**

| Service | Image | Purpose | Notes |
|---|---|---|---|
| `postgres` | `postgres:16-alpine` | Shared Postgres (both databases) | Healthcheck: `pg_isready` |
| `postgres-init` | `postgres:16-alpine` | One-shot: creates `zitadel_db` database | `restart: no`; runs after postgres healthy |
| `zitadel` | `ghcr.io/zitadel/zitadel:latest` | OIDC provider | Depends on postgres-init; uses `zitadel_db` |
| `mongodb` | `mongo:7` | PowerSync internal store | Required by PowerSync; no Altair data here |
| `powersync` | `public.ecr.aws/powersync/powersync-service:latest` | Sync service | Config via mounted `powersync.yml` |
| `garage` | `dxflrs/garage:v1` | S3-compatible object storage | Used in Step 10; included now so stack is complete |

All service credentials loaded from `.env` via `env_file: ./.env`. No credentials in `docker-compose.yml`.

**Startup order:** `postgres` → `postgres-init` → `zitadel` + `mongodb` → `powersync`

---

### Migrations (`infra/migrations/`)

**Tooling:** `sqlx-cli` installed via `cargo install sqlx-cli --no-default-features --features postgres`

**Migration naming:** `YYYYMMDDHHMMSS_description.sql` per `05-erd.md` convention.

**Step 1 migration files (Phase 1 tables from ERD):**

```
infra/migrations/
├── 20260412000001_create_updated_at_trigger.sql     ← reusable trigger function
├── 20260412000002_create_users.sql
├── 20260412000003_create_households.sql
└── 20260412000004_create_household_memberships.sql
```

**`users` schema** uses `oidc_sub` (see Decision 5), not `password_hash`.

**Each migration file includes:**
- `-- migrate:up` SQL block
- `-- migrate:down` SQL block (rollback, invariant D-1)

**Seed data:** `infra/scripts/seed.sql` — inserts one dev user record with a known `oidc_sub` matching the Zitadel dev setup. Applied manually or via CI after `sqlx migrate run`.

**DATABASE_URL** format: `postgres://altair_user:${POSTGRES_PASSWORD}@localhost:5432/altair_db`

---

### `packages/contracts/` Placeholder

**Step 1 scope:** Directory + `README.md` only. No code, no JSON registries.

The README describes:
- What this package will contain (entity-types.json, relation-types.json, sync-streams.json)
- The code generation targets (TypeScript, Kotlin, Rust)
- That population is deferred to Step 2 (Shared Contracts)

---

### mise.toml

Updated to pin:
- `bun` — latest stable (e.g. `1.x`)
- `rust` — `stable` (defer exact pin to `rust-toolchain.toml` inside `apps/server/`)
- `java` — `temurin-17` (for Android Gradle in CI)
- `node` — not needed (Bun handles JS); omit

---

## Integration Points

The only cross-stack contract in Step 1 is the database URL. All other integration points (API routes, sync streams, auth tokens in clients) are established in later steps.

| Interface | Producer | Consumer | Step 1 scope |
|---|---|---|---|
| `DATABASE_URL` env var | `infra/compose/.env` | `apps/server/`, CI | Wired up in Compose |
| Zitadel OIDC discovery URL | Zitadel container | `apps/web/` (Step 9), `apps/server/` (Step 3) | Container running; URL documented in `.env.example` |
| Garage S3 endpoint | Garage container | `apps/server/` (Step 10) | Container running; credentials in `.env.example` |
| PowerSync service URL | PowerSync container | All clients (Step 4+) | Container running; URL documented |

---

## Risks & Unknowns

- **Risk:** Zitadel first-run initialisation on an empty database requires a specific startup sequence (master key, initial admin user). If the Compose healthcheck races, Altair's smoke test may run before Zitadel is ready.
  - **Mitigation:** Zitadel provides a `--init-projections` flag and exposes a `/debug/healthz/ready` endpoint. The smoke test job polls this endpoint before proceeding to FA-009.

- **Risk:** PowerSync Open Edition license terms or container image location may change. The `public.ecr.aws/powersync/powersync-service` image path is based on current documentation.
  - **Mitigation:** Pin the image to a specific digest in Compose. Verify the ECR path during implementation; use the verified path in Steps.md.

- **Risk:** Android CI on `ubuntu-latest` requires Android SDK. GitHub-hosted runners include the SDK but specific versions may require explicit `sdkmanager` setup.
  - **Mitigation:** Use `android-actions/setup-android` or confirm that the target compile SDK version is pre-installed on `ubuntu-latest`.

- **Risk:** `cargo sqlx prepare` must be re-run whenever a query is added or modified. Developers forgetting this breaks CI.
  - **Mitigation:** Add a CI check: `cargo sqlx prepare --check` in the `rust` job (fails if `.sqlx/` is stale). Document the workflow in the repo README.

- **Unknown:** Exact Zitadel management API calls needed for the `zitadel-setup.sh` script. Zitadel's API has changed across major versions.
  - **Resolution plan:** Use the Zitadel Go client or `zitadel-tools` CLI if available; otherwise use `curl` against the management REST API. Validate against whichever Zitadel image version is pinned in Compose.

---

## Testing Strategy

Step 1 has no unit-testable business logic. Testing is entirely structural and integration-based:

| Assertion | Test type | Where |
|---|---|---|
| FA-001: `cargo build` | CI build job | `rust` GitHub Actions job |
| FA-002: `bun run dev` | CI process check | `web` GitHub Actions job |
| FA-003: `./gradlew build` | CI build job | `android` GitHub Actions job |
| FA-004: All Compose services healthy | Integration | `smoke-test` CI job |
| FA-005–FA-008: Migrations + seed | Integration | `smoke-test` CI job (sqlx + psql assertions) |
| FA-009: OIDC login end-to-end | Manual | Local Compose stack; automated E2E deferred to Step 9 |
| FA-010: Migration rollback | Integration | `smoke-test` CI job (`sqlx migrate revert`) |
| FA-011: CI passes | CI gate | GitHub branch protection |
| FA-012: No committed secrets | Static scan | `git grep` / `trufflehog` in CI pre-check |
