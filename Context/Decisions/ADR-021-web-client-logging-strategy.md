# ADR-021: Web Client Logging Strategy — console.error Stopgap

## Status

Accepted

## Date

2026-04-16

## Context

The web client (`apps/web/`) shipped Feature 008 with no structured logging infrastructure. The
only logging call in the entire application was a single `console.error` in `hooks.server.ts`.

The PR #9 review (findings P9-003 through P9-020) identified ~15 error-handling gaps:
repository watch failures silently discarded, `uploadData()` missing error boundary,
`fetchCredentials()` unprotected, multiple server-side load functions missing try/catch, and
`subscribeToStreams()` swallowing connection failures. Every gap was reachable in production and
several could cause silent data loss or infinite retry loops.

All fixes were applied during review resolution, but each fix uses `console.error` directly —
there is no `logError()` wrapper, no structured context shape, and no path to a production
observability tool (Sentry, Datadog, etc.) without touching every call site.

## Decision

Accept `console.error` as the logging mechanism for the current release (Feature 008).

Before any future feature work modifies the web client error-handling paths, establish a minimal
`logError(message: string, context?: Record<string, unknown>): void` utility at
`apps/web/src/lib/utils/logger.ts`. The utility must:

1. Accept a structured `context` object so call sites do not need to change when a provider is
   adopted.
2. In development (`import.meta.env.DEV`): delegate to `console.error`.
3. In production: delegate to `console.error` initially; swap to Sentry or equivalent by
   replacing this single file.
4. Never throw — logging must be a best-effort side effect.

Once the utility exists, all `console.error(...)` calls introduced by the P9 review fixes should
be migrated to `logError(...)` in a single pass.

## Consequences

**Benefits:**
- Observability gaps are fixed for the current release without blocking the PR merge.
- The structured `context` parameter enforces a consistent call shape from the start; future
  migration to a real provider requires only a one-file change.
- No call sites need updating until a provider is chosen.

**Trade-offs:**
- Production errors are visible in browser consoles only (not aggregated or alerted on) until
  the utility is adopted and wired to a provider.
- Two passes of work are required: create the utility, then migrate call sites.

**Risks:**
- If the migration pass is deferred indefinitely, `console.error` becomes a permanent production
  pattern — add the utility creation to the backlog immediately.

## Compatibility

No overlapping accepted ADRs found. ADR-011 (AppError Variant Taxonomy) governs the server's
error taxonomy (`apps/server/`) and has no bearing on the web client's logging approach.

## Related

- **Feature:** Context/Features/008-WebClient/
- **Review findings:** P9-003, P9-004, P9-005, P9-007, P9-008, P9-019, P9-020 (PR #9)
- **Files affected:** `apps/web/src/lib/utils/logger.ts` (to be created), all repository and sync files modified during P9 review resolution
- **Backlog:** Create `logError` utility and migrate P9 call sites
