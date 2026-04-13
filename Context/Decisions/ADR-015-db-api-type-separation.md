# ADR-015: Separate DB Row Types from API Response Types in Domain Modules

## Status

Accepted

## Date

2026-04-13

## Context

Core domain models (`Initiative`, `Tag`, `EntityRelation`) currently derive both `sqlx::FromRow` and `serde::Serialize`. This causes internal fields — `user_id` (redundant to the caller who already authenticated) and `deleted_at` (soft-delete implementation detail) — to leak into API responses.

The auth module already handles this correctly: `MeRow` is a local `#[derive(sqlx::FromRow)]` struct used only for the query, and `UserProfile` is the `#[derive(Serialize)]` response type. The same pattern must be applied consistently.

## Decision

All domain modules must maintain a strict separation:

- **DB row type**: `#[derive(sqlx::FromRow)]` only — never `Serialize`. Local to `service.rs` or defined in `models.rs` with a clear naming convention (e.g. `InitiativeRow`).
- **API response type**: `#[derive(Serialize)]` only — never `FromRow`. Exposes only the fields the caller should see (no `user_id`, no `deleted_at`, no internal state).
- A mapping step (e.g. `From<InitiativeRow> for Initiative`) converts between them.

The current domain models (`Initiative`, `Tag`, `EntityRelation`) violate this and must be refactored before their endpoints are consumed by clients.

**Implementation**: Refactor is deferred to the first feature that adds client-facing reads for these endpoints (likely Feature 005). Until then, the current behaviour leaks `user_id` and `deleted_at` in responses — acceptable since no clients consume these endpoints yet.

## Consequences

- Future domain modules must define separate row/response types from day one.
- Feature 005 must include a refactor of `core/initiatives`, `core/tags`, and `core/relations` models as part of its scope.
- The auth module's pattern (`MeRow` + `UserProfile`) is the canonical precedent.

## Relates To

- ADR-011 (AppError taxonomy — separation of concerns precedent)
- P4-019 (review finding)
