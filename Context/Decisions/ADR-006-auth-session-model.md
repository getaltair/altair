# ADR-006: Auth and Session Model

## Status

Partially Superseded by [ADR-012](ADR-012-builtin-auth-argon2id-jwt.md)

> **What's superseded:** Identity provider choice (Zitadel/OIDC), token model (provider-issued JWTs), auth flow (OIDC authorization code + PKCE), security controls delegated to identity provider.
>
> **What remains active:** Authorization model — household_memberships table, user/household scoping on every query, RLS policies on PostgreSQL, PowerSync connector auth pattern (now using server-issued JWT instead of OIDC access token).

## Date

2026-04-12

## Context

Altair is multi-user by design. A single self-hosted instance serves a household — multiple users sharing some data (locations, shopping lists, chores) while keeping other data private (personal goals, notes). User isolation is a security requirement, not an afterthought.

The architecture spec describes initial username/password auth with Argon2id hashing and a refresh/access token model, with OIDC and 2FA as later additions.

However, multi-user is a core feature, not a later addition. Building simple auth first and bolting on OAuth later creates migration risk and security debt. Starting with OAuth/OIDC from day one provides a standard identity model that supports single-user, household, and future federation scenarios without rearchitecting.

## Decision

### OAuth 2.0 / OIDC with Self-Hosted Identity Provider

Use a self-hosted OIDC-compliant identity provider as the auth layer. The Axum server is a **relying party** (resource server), not an identity provider itself.

**Recommended providers (self-hosted, lightweight):**

| Provider | Language | RAM Footprint | Notes |
|----------|----------|--------------|-------|
| **Zitadel** | Go | ~200-400MB | All-in-one, Postgres-backed, lightweight |
| **Authentik** | Python | ~500MB+ | Feature-rich, heavier |
| **Keycloak** | Java | ~500MB-1GB | Enterprise-grade, heavy for Pi |

Zitadel is the recommended default for self-hosted deployments due to PostgreSQL backend (shared with Altair's DB) and reasonable memory footprint. Users may substitute any OIDC-compliant provider.

### Token Model

- **Access tokens** (JWT, short-lived ~15min) — included in API requests, validated by Axum middleware
- **Refresh tokens** (opaque, long-lived) — managed by the identity provider, used to obtain new access tokens
- **ID tokens** (JWT) — used during login flow to establish user identity

The Axum server validates JWT access tokens by checking signature against the OIDC provider's JWKS endpoint. No session state stored on the server.

### Auth Flow

**Web client:**
1. SvelteKit redirects to OIDC provider login page
2. User authenticates (username/password, 2FA if configured)
3. Provider redirects back with authorization code
4. SvelteKit exchanges code for tokens (PKCE flow)
5. Access token included in API requests and PowerSync connector

**Android client:**
1. AppAuth library opens system browser for OIDC login
2. Same flow as web (authorization code + PKCE)
3. Tokens stored in Android Keystore

**PowerSync connector:**
- Access token passed to PowerSync backend connector for authentication
- Connector refreshes token before expiry using refresh token

### Authorization Model

- OIDC provides **identity** (who you are)
- Altair server provides **authorization** (what you can access)
- `household_memberships` table maps users to households with roles
- Every domain query includes user/household scope check
- RLS policies on PostgreSQL enforce isolation at the database level

### Security Controls

- PKCE required for all authorization code flows (public clients)
- CSRF protection on web session flows
- Signed/encrypted tokens only
- HTTPS required in production
- Structured audit logging for auth events
- Rate limiting on auth endpoints (handled by identity provider)

## Consequences

### Positive

- Standard OIDC protocol — any compliant provider works
- Multi-user supported from day one, no migration needed
- 2FA, social login, passkeys available through the identity provider without custom code
- Stateless JWT validation on the API server — simple and scalable
- Android auth uses platform-standard AppAuth library
- Future federation (linking instances) is possible via OIDC

### Negative

- Identity provider adds one more service to the deployment stack (~200-400MB RAM for Zitadel)
- Deployment complexity increases — users must configure OIDC provider
- First-run setup requires creating an OIDC application and user — needs a guided setup flow
- Pi 4 (4GB) memory budget is tighter with identity provider added

### Neutral

- Simple username/password auth is still available — it's just managed by the identity provider, not custom Altair code
- Argon2id password hashing is handled by the identity provider, not the Axum server
- Single-user deployments still go through OIDC — slight over-engineering for solo use, but consistent model

## Notes

ADR-002 deployment targets should be revisited to account for the identity provider's memory footprint. The minimum 4GB tier becomes tighter; 8GB recommended tier absorbs it comfortably.
