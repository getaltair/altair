# ADR-019: PATCH Update Semantics Are Set-Only — Nullable Fields Cannot Be Cleared

## Status
Accepted

## Date
2026-04-15

## Context

All five guidance domain update functions (quests, epics, routines, focus sessions, daily check-ins)
use `COALESCE($N, column)` for partial PATCH updates. When a client sends JSON `null` for a nullable
field, serde deserializes it as `None`, and `COALESCE(NULL, column)` silently preserves the existing
value.

This means users cannot clear fields like `due_date`, `description`, `initiative_id`, `epic_id`,
`mood`, or `notes` through a PATCH request — `null` (explicit clear intent) is indistinguishable from
"field not provided".

Three options were evaluated:

1. **Accept the limitation** — document that PATCHes are set-only; clearing requires DELETE+POST or
   a future extension.
2. **`Option<Option<T>>` with `serde_with::double_option`** — outer `None` = not provided, inner
   `None` = set to null. Requires SQL pattern change from `COALESCE` to explicit three-way
   `CASE`/`IS DISTINCT FROM` logic in every update query.
3. **JSON Merge Patch (RFC 7396)** — a key present with `null` value means clear it. Requires
   custom serde deserialization and the same SQL pattern changes as option 2.

## Decision

Accept the current limitation (option 1). PATCH semantics in the guidance domain are **set-only**:
a client may update a field to a new value, but cannot explicitly clear a nullable field to `NULL`
through a PATCH request.

The guidance domain fields affected (`due_date`, `description`, `mood`, `notes`, `initiative_id`,
`epic_id`) are additive-in-practice during v1 use. The complexity of `Option<Option<T>>` + external
crate dependency + SQL changes in every update query is disproportionate for the current scope.

If a future feature requires clearing (e.g., un-assigning a quest from an epic, clearing a due date),
the correct approach is to revisit this ADR and adopt `serde_with::double_option` at that time.

## Consequences

**Benefits:**
- Simpler update handler code — no three-way null distinction logic.
- No additional crate dependencies (`serde_with`).
- Consistent pattern across all five guidance modules.

**Trade-offs:**
- Clients cannot clear nullable guidance fields through PATCH. This must be documented in the API
  reference.
- The limitation applies globally across the codebase wherever COALESCE partial-update is used.

**Risks:**
- A client UI that needs to clear `due_date` (e.g., "remove deadline") cannot do so through PATCH.
  If this surfaces as a real user need, the fix requires changes to multiple service files and SQL
  queries simultaneously.

## Compatibility

**Checked against:** ADR-011 (AppError taxonomy), ADR-015 (DB-API type separation), ADR-003 (sync conflict resolution)

- ADR-011: Compatible — this ADR addresses update semantics, not error handling.
- ADR-015: Compatible — this ADR addresses partial update behavior, not DB/API type separation.
- ADR-003: Compatible — last-write-wins sync conflict resolution is independent of whether individual
  fields can be explicitly nulled through PATCH.

## Related
- **Feature:** Context/Features/005-GuidanceDomain/
- **Files affected:** `apps/server/server/src/guidance/*/service.rs` (all update functions)
- **Review finding:** P6-005 (PR-feat-guidance-domain-2026-04-15.md)
