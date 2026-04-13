# ADR-013: First-User Admin Registration — TOCTOU Race Mitigation

## Status

Accepted

## Date

2026-04-13

## Context

The register handler determines whether a new user should be admin by issuing a `SELECT COUNT(*)` query and then a separate `INSERT`. Under concurrent load two registrations can both observe `count == 0` and both insert with `is_admin = true, status = 'active'`, producing multiple admin accounts.

This is a TOCTOU (time-of-check / time-of-use) race. Severity is low in practice for a self-hosted personal operating system — the window is narrow and concurrent first-registration is unlikely — but the invariant "exactly one first-user admin" must hold.

## Decision

Consolidate the count-check and insert into a single atomic SQL operation using a subquery. The INSERT determines admin/pending status in one round-trip:

```sql
INSERT INTO users (email, display_name, password_hash, is_admin, status)
SELECT
    $1, $2, $3,
    (SELECT COUNT(*) = 0 FROM users WHERE deleted_at IS NULL) AS is_admin,
    CASE WHEN (SELECT COUNT(*) = 0 FROM users WHERE deleted_at IS NULL)
         THEN 'active' ELSE 'pending' END AS status
RETURNING id, is_admin, status
```

This removes the separate count query and makes the admin determination atomic with the insert. No serializable transaction or advisory lock is needed — the subquery evaluates inside the same statement execution.

**Implementation deferred to S029** (auth integration test task). Until then, the TOCTOU window exists but is accepted given the deployment context.

## Consequences

- The separate `SELECT COUNT(*)` query in the register handler is removed.
- The insert returns `is_admin` and `status` via `RETURNING`, eliminating the subsequent re-query.
- The handler does not need a `BEGIN / COMMIT` wrapping.
- If a race does occur before this is implemented, both users become admin — recoverable via a direct DB update.

## Relates To

- ADR-012 (built-in auth design)
- P4-017 (review finding)
