# ADR-011: AppError Variant Taxonomy

## Status

Accepted

## Date

2026-04-12

## Context

`apps/server/server/src/error.rs` defines `AppError` with two variants: `NotFound` (→ 404) and
`Internal` (→ 500). Every future route handler returns `Result<impl IntoResponse, AppError>`.

With only two variants, every error that is not a 404 maps to 500. Auth failures, validation errors,
conflicts, and bad requests all become `Internal` — indistinguishable from genuine server errors.
Clients cannot distinguish retryable (5xx) from non-retryable (4xx) errors, and cannot surface
actionable feedback to users.

This decision must be made before feature handlers are written, because adding variants later
requires updating every `match` on `AppError` across the codebase.

## Decision

Expand `AppError` to the following variant taxonomy before any domain handlers are written:

| Variant | HTTP Status | Use case |
|---|---|---|
| `NotFound` | 404 | Resource does not exist |
| `Unauthorized` | 401 | No valid credentials presented |
| `Forbidden` | 403 | Valid credentials, insufficient permission |
| `BadRequest(String)` | 400 | Malformed input, validation failure |
| `Conflict(String)` | 409 | State conflict (duplicate, concurrent write) |
| `Internal` | 500 | Unexpected errors; message NOT leaked to client |

Response body format (JSON):

```json
{ "error": "<variant-specific message or generic fallback>" }
```

`Internal` responses return `{ "error": "Internal server error" }` regardless of the underlying
cause. The actual cause is logged at `tracing::error!` level with full context before the response
is returned.

All variants are added to `error.rs` now. `IntoResponse` is updated accordingly. Existing callers
(`NotFound`, `Internal`) are unaffected.

## Consequences

### Positive

- Clients can distinguish retryable (5xx) from non-retryable (4xx) errors
- Auth middleware can return 401/403 without abusing `Internal`
- Validation errors return 400 with a descriptive message
- Exhaustive `match` on `AppError` is enforced at compile time — new variants force handler updates

### Negative

- More variants to test (each should have a route-level test for status + body)
- `BadRequest(String)` and `Conflict(String)` carry a user-visible string — must be sanitized
  at call sites; no internal details should be interpolated into these messages

### Neutral

- This taxonomy covers the needs of all planned domain features (Guidance, Knowledge, Tracking)
- Further variants (e.g., `TooManyRequests` for rate limiting) can be added when the feature arrives

## Related

- P1-013: Review finding that identified the two-variant limitation
- P1-014: `AppError::IntoResponse` test task (S018-T) covers this taxonomy
- `apps/server/server/src/error.rs`
