# ADR-012: Built-in Auth with Argon2id + Server-Issued JWT

## Status

Accepted

## Date

2026-04-12

## Context

ADR-006 chose a self-hosted OIDC identity provider (Zitadel) as the auth layer. A Renaissance Architecture review identified this as over-engineered for v1:

1. **Disproportionate overhead.** Zitadel adds ~200-400MB RAM and a second Postgres database (`zitadel_db`) for a system that serves 1-5 household members. The identity provider is enterprise-grade software solving federation, social login, and multi-tenant identity — none of which v1 requires.

2. **Deployment friction.** Zitadel requires a 32-byte master key, `--tlsMode disabled` for local dev, a dedicated postgres-init container to create its database, and a guided first-run OIDC application setup. Every self-hosting user pays this complexity cost.

3. **The original plan was simpler.** The architecture spec described Argon2id password hashing with refresh/access JWTs. ADR-006 argued that "building simple auth first and bolting on OAuth later creates migration risk." In practice, the reverse is true for v1: the OIDC integration created three ADRs of security scaffolding (ADR-006, ADR-009, ADR-010) and multiple review findings before any domain feature was built.

4. **Docker Compose impact.** Removing Zitadel eliminates 2 of 6 services (Zitadel + postgres-init), reducing the deployment stack from 6 to 4 containers.

The authorization model from ADR-006 — household memberships, RLS policies, user/household scoping on every query — remains correct and unchanged. Only the identity layer is replaced.

## Decision

### Built-in Argon2id Password Auth + Server-Issued JWT

The Axum server is both identity provider and resource server for v1. Auth is handled entirely within the server crate.

### Password Hashing

- **Argon2id** with OWASP-recommended parameters (memory: 19MiB, iterations: 2, parallelism: 1)
- Parameters stored with the hash (PHC string format) — tunable without migration
- Use the `argon2` crate (pure Rust implementation)

### Token Model

| Token | Format | Lifetime | Storage (Web) | Storage (Android) |
|-------|--------|----------|---------------|-------------------|
| Access token | JWT (signed, HS256 or RS256) | 15 minutes | httpOnly cookie | Android Keystore |
| Refresh token | Opaque (stored server-side) | 7 days | httpOnly cookie | Android Keystore |

- Access tokens contain: `sub` (user UUID), `household_ids` (array), `iat`, `exp`
- Refresh tokens stored in a `refresh_tokens` table with device tracking and revocation support
- JWT signing key loaded from environment variable; RS256 preferred for production

### Auth Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/auth/register` | POST | Create account (Argon2id hash + initial household) |
| `/api/auth/login` | POST | Verify credentials, issue token pair |
| `/api/auth/refresh` | POST | Exchange refresh token for new token pair |
| `/api/auth/logout` | POST | Revoke refresh token, clear cookies |
| `/api/auth/me` | GET | Return current user profile |

### Auth Flow

**Web client:**
1. SvelteKit renders login form
2. Form submits credentials to `/api/auth/login`
3. Server validates password against Argon2id hash
4. Server issues JWT access token + refresh token in httpOnly cookies
5. Subsequent requests include cookies automatically
6. SvelteKit hooks extract JWT for server-side operations
7. PowerSync connector reads token from cookie for sync auth

**Android client:**
1. Login screen submits credentials to `/api/auth/login`
2. Server returns token pair in response body (not cookies)
3. Tokens stored in Android Keystore
4. Access token included in API request headers
5. PowerSync connector uses stored access token

### Auth Middleware

- Axum middleware extracts JWT from `Authorization: Bearer` header (Android) or httpOnly cookie (web)
- Validates signature, expiration, and claims
- Injects `AuthUser { user_id, household_ids }` into request extensions
- All domain routes require authenticated user — no public domain endpoints

### CSRF Protection

- Web: SvelteKit's built-in CSRF protection (origin checking on form submissions)
- Cookie-based auth uses `SameSite=Lax` to prevent cross-origin request attachment
- No OIDC state parameter needed (ADR-009 is fully superseded)

### Security Controls

- HTTPS required in production (`Secure` flag on cookies)
- Rate limiting on auth endpoints (server-side, not delegated to identity provider)
- Structured audit logging for auth events (login, logout, failed attempts)
- Password complexity requirements enforced at registration
- Refresh token rotation on each use (old token revoked)

### Docker Compose Changes

**Removed:**
- `zitadel` service (OIDC provider)
- `postgres-init` service (only existed to create `zitadel_db`)

**Resulting stack (4 services):**
| Service | Purpose |
|---------|---------|
| `postgres` | Primary database (single DB: `altair_db`) |
| `mongodb` | PowerSync metadata store |
| `powersync` | Offline-first sync service |
| `rustfs` | S3-compatible object storage |

### Migration to OIDC (v2)

Built-in auth is designed to not preclude OIDC:

- JWT claims structure (`sub`, `household_ids`) is compatible with OIDC ID token claims
- Adding an OIDC provider in v2 means: add `oidc_sub` column to `users`, implement OIDC login flow alongside password login, map OIDC `sub` claim to internal user
- Password auth remains available as a fallback — OIDC is additive, not a replacement
- The authorization model (household memberships, RLS, scoping) is identity-provider-agnostic

## Consequences

### Positive

- 2 fewer containers in deployment stack (~200-400MB RAM freed, reduced complexity)
- No external service configuration for auth — works out of the box
- First-run setup is: create an account, start using the app
- Auth code is in the server crate — debuggable, testable, no cross-service coordination
- Simpler mental model: one server handles everything
- Password hashing parameters are tunable without migration (PHC format)

### Negative

- No 2FA, social login, or passkeys in v1 — these require custom implementation or deferral to v2 OIDC
- Server is now responsible for password security (hashing, rate limiting, audit logging) — more surface area in application code
- Single-user and multi-user deployments both require password creation (minor friction vs. no auth)

### Neutral

- ADR-006's authorization model (household memberships, RLS, scoping) is unchanged
- The httpOnly cookie principle from ADR-010 carries forward
- PowerSync connector auth is simpler (JWT from server, not OIDC provider)

## Supersedes

- **ADR-006** (partially) — Identity/provider sections replaced. Authorization model (household memberships, RLS, scoping) remains active.
- **ADR-009** (fully) — OIDC state parameter CSRF protection no longer applicable. Web CSRF handled by SvelteKit built-in + SameSite cookies.
- **ADR-010** (fully) — OIDC callback token storage no longer applicable. The httpOnly cookie security principle is adopted directly in this ADR's token model.
