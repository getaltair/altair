# ADR-010: Token Storage Pattern After OIDC Callback

## Status

Accepted

## Date

2026-04-12

## Context

The initial `/auth/callback` implementation returns `access_token` and `id_token` from the SvelteKit
`load` function, embedding them in the server-rendered HTML payload. This was scoped as a dev-only
scaffold to verify the end-to-end OIDC flow (FA-009).

The current pattern is not suitable for production or any code built on top of it:

1. Tokens appear in browser history (as part of the navigated URL's page payload)
2. Tokens are visible in reverse proxy and CDN access logs
3. Tokens are rendered in the DOM (`<pre>` block), accessible to any injected script
4. It establishes a precedent that tokens are page data — future developers may extend this pattern

The decision is: how tokens should be stored and passed to the client after the OIDC callback.

## Options Considered

### Option A: httpOnly session cookie (server-side session)

After token exchange, store the access token in an httpOnly `Set-Cookie` header (or a server-side
session keyed by an opaque session ID cookie). Redirect 302 to `/` or the intended destination.

**Pros:** Tokens never reach the browser DOM; XSS cannot read them; standard secure web pattern.
**Cons:** Requires a session store or encrypted cookie strategy; adds complexity.

### Option B: In-memory store + `window.__token` on client

Redirect after exchange, pass tokens via a transient server-side session to a client-side in-memory
store (never `localStorage`/`sessionStorage`).

**Pros:** Tokens not in DOM after handoff; refresh can be done silently.
**Cons:** More complex handoff; tokens lost on page reload (requires re-auth or silent refresh flow).

### Option C: Continue rendering tokens in page data

Keep current approach; acceptable for dev-only use, clean up before production.

**Cons:** The pattern becomes entrenched. New features (PowerSync connector, API calls) built before
the cleanup will assume tokens are available as page data and require rework.

## Decision

**Option A**: Replace the current callback page with a server-side redirect flow:

1. Callback `load` function exchanges the code for tokens
2. Tokens are stored in an httpOnly, `SameSite=Lax`, `Secure` (production) cookie
3. `redirect(302, '/')` is returned — no tokens appear in page data
4. Downstream code (PowerSync connector, API middleware) reads the token from the cookie

The specific cookie structure (single encrypted cookie vs. opaque session ID pointing to a
server-side store) is decided during Step 3 / Step 9 auth hardening. For the current scaffold
fix, a single signed httpOnly cookie is sufficient to unblock safe development.

The dev-only `<pre>` display of tokens is removed entirely — token verification during development
is done via the server logs (tracing) or browser DevTools (network tab → token endpoint response).

## Consequences

### Positive

- Tokens never appear in browser DOM or page history
- XSS cannot exfiltrate tokens via `document.cookie` (httpOnly)
- Correct pattern is established before any feature code builds on the auth scaffold
- Aligns with ADR-006's token model (short-lived access tokens, no server session state where possible)

### Negative

- Cookie-based token access requires SvelteKit hooks to extract and forward to the API
- HTTPS required for `Secure` cookie flag in production (already a requirement per ADR-006)
- Full session lifecycle (refresh, revocation) still deferred to Step 9

## Related

- ADR-006: Auth and Session Model
- ADR-009: OIDC state parameter (P1-007) — both must be applied together
- P1-008: Review finding that identified the token-in-DOM pattern
- FA-009: OIDC login end-to-end assertion (manual verification)
