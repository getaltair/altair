# Conventions

Shared rules for every AI agent working in this repository. Read this file before any GitHub or git operation.

## Source Of Truth

`docs/initial-build.md` defines the accepted v0.1 behavior, scope, technical foundation, and engineering baseline.

Do not reinterpret an excluded feature as a prerequisite for accepted behavior. Record scope changes before implementing them.

## Conventional Commits And Semantic Versioning

Every commit MUST follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).

Use `<type>(<scope>): <description>`. A space after the colon is mandatory.

| Type | Version effect |
|---|---|
| `feat` | Minor |
| `fix`, `perf` | Patch |
| `docs`, `chore`, `style`, `refactor`, `test` | None unless breaking |
| `BREAKING CHANGE:` or `!` | Major |

Do not attribute commits or pull requests to an AI agent.

## Always Green And Shift Left

Always Green means Preflight and CI pass before forward work continues.

Defects cost less to fix during development than during integration or production. Fix red gates before adding more work.

Preflight is `just preflight`. It MUST run tests, lint checks, and production builds for both applications.

CI green means `gh pr checks` reports passing checks whenever a pull request is open.

## Discovered Defects

Treat every reproducible gate failure as a defect.

1. Fix a trivial, data-only, single-file failure immediately.
2. Write `specs/bugs/BUG-*.md` before fixing a non-trivial failure.
3. Log blocked reproduction in the bug specification and stop affected forward work.

Keep a discovered fix in a separate commit.

### Banned Dismissals

| Banned phrase | Required behavior |
|---|---|
| Pre-existing issue | Fix or log it. |
| Unrelated to this session | Fix or log it. |
| Not introduced by my changes | Reproduce, isolate, then fix or log it. |
| Out of scope | Fix or log any red gate before continuing. |

## Planning Output

Store every planning artifact under `specs/`.

| Purpose | Location |
|---|---|
| Scope | `specs/product/SCOPE_LATEST.yaml` |
| Vision | `specs/product/VISION_LATEST.yaml` |
| Glossary | `specs/product/GLOSSARY_LATEST.yaml` |
| Session state | `specs/state.yaml` |
| Release plan | `specs/release-plan.yaml` |
| Execution status | `specs/execution-status.yaml` |
| Planning status | `specs/planning-status.yaml` |
| Architecture | `specs/tech-architecture/` |
| Decisions | `specs/adr/` |
| Bugs | `specs/bugs/` |
| Verification | `specs/verifications/` |

`specs/state.yaml` contains `workflow_mode: solo-git`. Treat it as the canonical integration-mode signal.

## Architecture

Organize application behavior by domain. Keep `projects`, `tasks`, `notes`, `resources`, and `recall` explicit.

Keep shared infrastructure separate from domain behavior. Infrastructure includes PostgreSQL, Garage, Authentik, and inference clients.

Keep one Rust application with ordinary modules. Do not add services, brokers, gateways, or plugin systems speculatively.

Keep authoritative content in PostgreSQL and Garage. Treat retrieval indexes and vectors as rebuildable derived data.

Use stable record identities and explicit bidirectional relationships. Never copy records to represent links.

## Stack Conventions

### Vue And TypeScript

- Use Vue 3 Composition API with `<script setup>` for new components.
- Order single-file components as `<script setup>`, `<template>`, then `<style scoped>`.
- Use Pinia only for shared client state.
- Use composables for reusable client behavior.
- Use named exports in composables.
- Keep TypeScript strict and avoid untyped boundary data.
- Validate server responses at the application boundary.
- Keep Android layouts touch-usable rather than shrinking desktop layouts.
- Use Vitest for units and Vue Test Utils for components.
- Use Playwright for critical browser workflows.

### Rust And Axum

- Keep domain logic independent of Axum extractors, SQLx rows, and provider payloads.
- Expose domain-oriented API resources and commands.
- Use SQLx migrations for every schema change.
- Use transactions for multi-write behavior that must succeed atomically.
- Validate API and integration payloads at their boundaries.
- Return consistent, typed application errors without exposing secrets.
- Test domain behavior with unit tests and boundaries with integration tests.

## Data And Editing Safety

- Distinguish unsaved drafts, acknowledged saved state, and cached server responses.
- Report success only after server acknowledgement.
- Detect conflicting edits instead of silently overwriting content.
- Preserve competing content for explicit user resolution.
- Make removal reversible before final deletion.
- Keep Markdown as canonical note content.
- Keep attachment metadata and object references consistent during backup and restore.
- Preserve embedding model identity, revision, dimensions, and content revision.

## Dependency And Lockfile Safety

Use package-manager commands to add, remove, and update dependencies.

NEVER modify `Cargo.lock`, `package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`, or another lockfile by hand.

Do not delete a lockfile to avoid a targeted dependency update. Use the package manager's targeted update command.

Review generated dependency and lockfile changes. Run Preflight after dependency changes.

## Defensive Code Categories

### Timeout

Bound inference and external-service waits. Support cancellation where the caller can abandon obsolete work.

### Retry

Retry only transient and idempotent operations. Persist indexing work before retrying it.

Use bounded attempts and backoff. Do not retry validation, authentication, or permanent failures.

### Graceful Degradation

Keep editing, saving, search, and direct relationships usable when semantic inference fails.

Show degraded state honestly. Never claim that no relevant content exists when semantic retrieval is unavailable.

Rate limiting and circuit breakers are not currently required categories. Add them only for a concrete risk.

## Hard Stops

- NEVER bypass Authentik or application sessions for remotely exposed personal data.
- NEVER expose provider tokens to the browser.
- NEVER make saves depend on GPU availability.
- NEVER hide a cloud inference fallback behind API compatibility.
- NEVER generate autonomous links, edits, count changes, or task transitions.
- NEVER treat missing quantity as zero or one.
- NEVER merge possible duplicate resources automatically.
- NEVER discard a draft while opening or reusing a suggested record.
- NEVER implement excluded v0.1 features without an accepted scope change.
