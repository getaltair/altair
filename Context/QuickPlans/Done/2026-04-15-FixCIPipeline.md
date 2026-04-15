# Quick Plan: Fix CI Pipeline

**Task:** Fix six confirmed bugs across `.github/workflows/ci.yml`, `infra/scripts/seed.sql`, and `apps/android/`

**Goal:** All CI jobs (Rust, Web, Android, Validate Contracts, Smoke Test) pass green

**Source:** CI run https://github.com/getaltair/altair/actions/runs/24467692499

---

## Bugs Found

### Bug 1 — Rust: `sqlx-cli` not installed before `cargo sqlx prepare --check`

**File:** `.github/workflows/ci.yml`, "Check sqlx offline cache" step

**Error:** `error: no such command: 'sqlx'` — `cargo-sqlx` CLI is not installed in the `rust` job.
The `smoke-test` job installs it, but the `rust` job does not.

**Fix:** Add an install step before the check: `cargo install sqlx-cli --no-default-features --features postgres`

---

### Bug 2 — Web: `PUBLIC_API_BASE_URL` not set at build time

**File:** `.github/workflows/ci.yml`, "Build" step in the `web` job

**Error:** `[MISSING_EXPORT] "PUBLIC_API_BASE_URL" is not exported by virtual:env/static/public`
SvelteKit validates static public env vars at build time. The var is defined in `apps/web/.env.example`
but no `.env` is created in CI before the build runs.

**Fix:** Add a step to copy `.env.example` to `.env` before `bun run build`.

---

### Bug 3 — Android: `gradle-wrapper.jar` not in git

**File:** `apps/android/gradle/wrapper/` — only `gradle-wrapper.properties` is tracked

**Error:** `Error: Unable to access jarfile apps/android/gradle/wrapper/gradle-wrapper.jar`

The `gradlew` bootstrap script requires the JAR to be present. Standard Android convention is to
commit it alongside `gradle-wrapper.properties`.

**Fix:** Generate and commit `gradle-wrapper.jar` using the local Gradle installation:
```bash
cd apps/android && gradle wrapper --gradle-version 9.3.1
git add gradle/wrapper/gradle-wrapper.jar
```

---

### Bug 4 — Smoke test waits for Zitadel, which no longer exists

**File:** `.github/workflows/ci.yml`, lines 171–186

The `smoke-test` job waits for Zitadel at `http://localhost:8081/debug/healthz`. But
`infra/compose/docker-compose.yml` has no Zitadel service — it was replaced by built-in auth
in migration `000005_migrate_users_to_builtin_auth` (ADR-012). This step waits 40×10s = 400s then fails.

**Fix:** Remove the "Wait for Zitadel" step entirely.

---

### Bug 5 — Seed SQL references dropped column `oidc_sub`

**File:** `infra/scripts/seed.sql`

Migration 000005 drops `oidc_sub` and adds `password_hash NOT NULL`. The seed still inserts
`(id, oidc_sub, email, display_name)` with `ON CONFLICT (oidc_sub)`. Fails at apply time.

**Fix:** Rewrite the seed to match the post-005 schema: include `password_hash` (placeholder
argon2id hash for CI), conflict on `email`, remove the Zitadel comment.

---

### Bug 6 — Migration rollback test reverts 4 of 28 migrations

**File:** `.github/workflows/ci.yml`, lines 222–230

The FA-010 rollback step loops `for i in 1 2 3 4` (4 reverts), then asserts 0 public tables remain.
There are 28 migrations; 24 stay applied after 4 reverts — assertion always fails.

**Fix:** Replace the fixed loop with a dynamic `while` that reverts until nothing remains:
```bash
while DATABASE_URL=... sqlx migrate revert --source infra/migrations; do :; done
```

---

## Approach

Apply in this order (minimize conflicts):

1. `apps/android/` — generate and commit `gradle-wrapper.jar` (local Gradle command)
2. `.github/workflows/ci.yml` — install sqlx-cli in the `rust` job
3. `.github/workflows/ci.yml` — copy `.env.example` before web build
4. `.github/workflows/ci.yml` — remove Zitadel wait step
5. `.github/workflows/ci.yml` — fix rollback loop
6. `infra/scripts/seed.sql` — rewrite to match current schema

---

## Verification

- `git ls-files apps/android/gradle/wrapper/` shows both `.properties` and `.jar`
- `git grep -n zitadel .github/` returns no hits
- Seed SQL: `\i infra/scripts/seed.sql` against a migrated local DB runs without error
- Rollback step loops dynamically (not a fixed count)
- `PUBLIC_API_BASE_URL` is set before `bun run build`
- sqlx-cli install step precedes `cargo sqlx prepare --check` in the `rust` job

---

## Risks

- `gradle wrapper` command requires Gradle pre-installed locally; if not available, download the JAR from Gradle's CDN using the SHA256 in `gradle-wrapper.properties` (`b266d5ff...`)
- The argon2id placeholder in seed.sql must satisfy `TEXT NOT NULL`; the smoke test never authenticates so any non-null string passes
- Dynamic rollback while-loop handles future migration additions automatically
