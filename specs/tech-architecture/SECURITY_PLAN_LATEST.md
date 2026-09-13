---
type: security-plan
context: altair-v0.1
---

# Security Plan

Authentik uses OIDC authorization code flow with PKCE. Axum owns exchange and
server sessions through Secure, HttpOnly cookies; state-changing requests use
CSRF protection. The browser never receives provider credentials or direct
Garage, database, or inference access.

Personal and unsaved content is excluded from logs, telemetry, and shared
caches. Inference uses only the declared endpoint, has no hidden fallback, and
is bounded by server-owned timeouts. Attachment names, Markdown rendering,
search text, import files, and exports are treated as untrusted input.

P0/P1 story ledgers include boundary tests and a final `just preflight` security
gate. Before real inference content, record non-retention and routing evidence
in `specs/verifications/INFERENCE_PRIVACY_LATEST.md` without secrets or personal
payloads.
