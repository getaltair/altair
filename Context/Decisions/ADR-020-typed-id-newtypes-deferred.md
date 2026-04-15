# ADR-020: Typed ID Newtypes — Deferred

**Status:** Deferred  
**Date:** 2026-04-15  
**Deciders:** Engineering  
**Tags:** rust, type-safety, api-design

---

## Context

Throughout the tracking domain, entity IDs are passed as bare `Uuid` values. Function signatures like:

```rust
pub async fn get_location(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    location_id: Uuid,
) -> Result<TrackingLocation, AppError>
```

are vulnerable to argument transposition bugs — swapping `user_id` and `household_id` at the call site compiles without error. At sufficient scale, typed ID newtypes (e.g., `struct UserId(Uuid)`, `struct HouseholdId(Uuid)`) make such bugs impossible at compile time.

---

## Decision

Do not introduce typed ID newtypes in the tracking domain at this time. Continue using bare `Uuid` for all entity IDs.

---

## Rationale

- The tracking domain has 6 sub-modules with consistent `(pool, user_id, household_id, entity_id)` argument ordering. The ordering is established and followed uniformly — the transposition risk is low in practice.
- Introducing newtypes across the codebase is a large, cross-cutting change. It would require: new wrapper types, `From`/`Into` impls, sqlx type registrations, and updates to every handler, service, and test.
- The auth and core domains also use bare `Uuid` — piecemeal adoption in tracking alone would create inconsistency.
- No transposition bug has been observed or reported.

---

## Consequences

**Positive:**
- No additional complexity or boilerplate.
- Codebase remains consistent with existing auth and core domain patterns.

**Negative:**
- No compile-time protection against argument transposition in multi-ID function signatures.
- If adopted later, migration will touch every service and handler file.

---

## Revisit Conditions

Reconsider this decision if:
- A transposition bug is found in production or tests.
- The codebase grows to a point where the argument ordering is no longer uniformly obvious.
- A future Rust ecosystem shift makes newtype wrappers with sqlx significantly less boilerplate-heavy.

---

## Alternatives Considered

### Full typed newtype adoption
Define `UserId(Uuid)`, `HouseholdId(Uuid)`, `LocationId(Uuid)`, etc. with `#[derive(sqlx::Type)]`. Makes transposition a compile error. Deferred due to migration cost and cross-domain consistency concerns.

### Phantom type parameters
Use `Id<User>`, `Id<Household>` with phantom generic. Same benefits as newtypes, more complexity. Also deferred.

---

## Related

- P6-027 review finding (PR feat/tracking-domain, 2026-04-15)
- ADR-015: DB/API Type Separation
