# ADR-016: `is_admin` Not Included in JWT Claims

## Status

Accepted

## Date

2026-04-13

## Context

The `me` handler currently re-queries `is_admin` from the database on every request to an admin-gated route. As admin-gated routes grow, each would require the same DB round-trip. Including `is_admin: bool` in the JWT `Claims` struct would make authorization decisions zero-cost after token validation.

The tradeoff: if admin status is revoked, the old claim remains valid until the token expires (15 minutes maximum, per ADR-012).

## Decision

`is_admin` is **not** added to JWT claims. Reasons:

1. **Staleness window is acceptable**: A 15-minute window is a standard tradeoff for stateless JWT auth. For a self-hosted personal OS, admin revocation is rare and usually driven by the admin themselves — they can wait for the token to expire or force a logout.

2. **`/me` is the only admin-gated endpoint currently**: There is no performance pressure yet. Premature optimization here adds Claims complexity and a schema commitment before the access control model is fully defined.

3. **Future flexibility**: If admin roles become more granular (e.g. household admin vs. global admin), adding `is_admin` as a boolean now locks in a flat model. A permissions array or role enum in claims is a better fit once the model is understood.

**Revisit trigger**: If more than 3 admin-gated routes exist with per-request DB queries, reconsider embedding admin status in claims.

## Consequences

- Every admin-gated request requires one extra DB query for role checking.
- Token revocation is immediate for non-admin data (standard JWT expiry applies); admin revocation takes up to 15 minutes.
- The `Claims` struct remains lean: `sub`, `email`, `household_ids`, `iat`, `exp`.

## Relates To

- ADR-012 (built-in auth, token design)
- P4-029 (review finding)
