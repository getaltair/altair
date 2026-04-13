# ADR-009: OIDC State Parameter and CSRF Protection

## Status

Accepted

## Date

2026-04-12

## Context

ADR-006 specifies "CSRF protection on web session flows" as a required security control. The initial
`/auth/login` and `/auth/callback` implementation omits the OAuth 2.0 `state` parameter (RFC 6749 §10.12).

PKCE protects against authorization code interception by an attacker who obtains the code. It does
**not** protect against login CSRF, where an attacker crafts a malicious redirect that causes the
victim's browser to complete a code exchange for an attacker-controlled session. Without a validated
`state` parameter, this attack is possible even with PKCE enabled.

The decision is: how to implement CSRF protection for the OIDC authorization code flow.

## Options Considered

### Option A: `crypto.randomUUID()` state in httpOnly cookie

Generate a random UUID as `state` on the login page server, store it in an httpOnly cookie, include
it in the authorization request, and validate it matches in the callback.

**Pros:** Self-contained, no dependencies, consistent with existing `code_verifier` cookie pattern.

**Cons:** Two httpOnly cookies to manage; callback must validate both.

### Option B: Session library (e.g., `lucia`, `better-auth`)

Delegate state generation and validation (plus token storage) to a session library.

**Pros:** Handles CSRF, state, token storage, and session lifecycle together.

**Cons:** Introduces a dependency; the auth flow is currently intentionally minimal (dev-only scaffold).
Token storage is deferred to a later step; pulling in a session library now couples two concerns.

### Option C: Defer CSRF protection

Acceptable temporarily because the callback currently only displays tokens in a dev-only view; no
production session is established.

**Pros:** Unblocks other work.

**Cons:** The pattern becomes entrenched; another developer may build on it without noticing the gap.
ADR-006 explicitly requires CSRF protection.

## Decision

**Option A**: Add a `crypto.randomUUID()` state parameter to the authorization request, stored in an
httpOnly `state` cookie (same `SameSite=Lax; Path=/` settings as `code_verifier`). The callback
validates that the `state` query parameter matches the cookie before proceeding with the token
exchange.

This is implemented alongside the fix for P1-008 (token storage), which will replace the current
dev-only page with a proper redirect flow. The `state` cookie is deleted after validation, mirroring
the `code_verifier` cookie cleanup.

The session library decision (Option B) is deferred to the Step 3 / Step 9 auth hardening work,
when token storage, refresh, and session lifecycle are fully implemented.

## Consequences

### Positive

- Login CSRF attack vector closed before any production session logic is built on the scaffold
- Consistent with existing cookie pattern (`code_verifier`)
- No new dependencies

### Negative

- Two cookies per login attempt (manageable)
- State validation adds a check to the callback that must be maintained

## Related

- ADR-006: Auth and Session Model (requires CSRF protection)
- P1-007: Review finding that identified the missing `state` parameter
- P1-008: Token storage ADR (ADR-010) — both fixes should be applied together
