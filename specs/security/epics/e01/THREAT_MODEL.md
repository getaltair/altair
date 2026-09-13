---
type: threat-model
context: altair-v0.1-e01
risk: HIGH
---

# e01 Secure Application Foundation Threat Model

## Surface Area

- Public liveness endpoint and authenticated application routes.
- Authentik OIDC authorization start and callback.
- Server-owned session creation, rotation, expiry, revocation, and cookies.
- CSRF validation for state-changing requests.
- Initial PostgreSQL record identity, revision, archive, restore, and draft-save boundaries.
- Frontend rendering through Vue, Naive UI, and Ionicons.

## Threats

| Category | Risk | Exploit Path | Required Mitigation |
|---|---|---|---|
| Authentication bypass / CWE-287 | High | Forged or replayed callback bypasses OIDC state, nonce, PKCE, issuer, or audience validation | Validate the complete authorization response server-side and reject replay |
| Session fixation / CWE-384 | High | An attacker preserves a session identifier across login | Rotate opaque session identity after authentication and privilege-state changes |
| CSRF / CWE-352 | High | A third-party origin submits an authenticated mutation | SameSite session cookies plus server-validated CSRF tokens and origin checks |
| IDOR / CWE-639 | High | A caller supplies another record identity | Scope every record operation to the authenticated workspace, including archive and restore |
| SQL injection / CWE-89 | High | Record fields or identifiers alter a query | Use developer-authored SQL with SQLx bound parameters; never interpolate user input |
| Sensitive data exposure / CWE-200 | High | Tokens, cookies, personal fields, or draft text enter logs/errors | Redact boundary logs and return typed errors without secrets or personal content |
| XSS / CWE-79 | Medium | Authored values reach unsafe browser HTML | Use Vue escaping; prohibit unsanitized `v-html`; sanitize later Markdown rendering explicitly |
| Revision race / CWE-362 | Medium | Concurrent saves overwrite acknowledged content | Enforce atomic expected-revision checks and idempotent operation replay |
| Health information disclosure / CWE-200 | Medium | Public health output reveals dependencies or configuration | Return process liveness only, with no personal, dependency, version, or secret detail |

## Security Invariants

1. Remote personal data is never accessible without a valid application session.
2. Provider tokens and credentials never reach browser storage or responses.
3. Every mutation requires both authenticated workspace scope and CSRF validation.
4. Save acknowledgement requires an atomic expected-revision match or idempotent replay.
5. Logs and typed errors omit cookies, tokens, authorization codes, record content, and drafts.
6. The public health endpoint exposes liveness only.

## Verification Guidance

- Test invalid state, nonce, PKCE verifier, issuer, audience, replay, expiry, and logout.
- Test missing, mismatched, and cross-origin CSRF requests against every mutation family.
- Test record access with valid but non-owned identifiers even though v0.1 is single-user.
- Test concurrent and replayed save/archive/restore operations against server revisions.
- Scan responses and captured logs for credentials and fixture personal content.
- Run dependency advisory checks through Bun and Cargo before commit.

## False-Positive Exclusions

- Developer-authored static SQL with bound parameters is not SQL injection.
- Client-side visibility checks are not authorization controls; only server enforcement is evaluated.
- UUID opacity is not a substitute for ownership checks, but UUID use alone is not a finding.
- Documentation-only examples and test fixtures without a production input path are excluded.
