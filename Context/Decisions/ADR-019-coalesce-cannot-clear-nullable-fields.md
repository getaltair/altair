# ADR-019: COALESCE Pattern Cannot Clear Nullable Fields

**Status:** Accepted  
**Date:** 2026-04-15  
**Deciders:** Engineering  
**Tags:** rust, sql, api-design

---

## Context

The tracking domain uses a COALESCE-based partial update pattern in service functions:

```sql
UPDATE tracking_items SET
    name        = COALESCE($1, name),
    description = COALESCE($2, description),
    location_id = COALESCE($3, location_id),
    ...
WHERE id = $4
```

This pattern treats `NULL` bindings as "no update intended" — the existing column value is preserved. This is convenient for optional update fields but introduces a semantic gap: a client can never intentionally clear a nullable field (e.g., set `description` back to `NULL`, or unlink a `location_id`).

The same pattern applies to `update_location`, `update_category`, and `update_item`.

---

## Decision

Accept the limitation for the current tracking domain implementation. The COALESCE pattern is retained as-is.

Nullable fields that clients may legitimately need to clear (e.g., `description`, `location_id`, `category_id`, `barcode`, `expires_at` on items) cannot be cleared via PATCH until this is revisited.

---

## Rationale

- All tracking domain nullable fields are optional metadata; clearing them is not required by any current user story or invariant.
- Changing the pattern requires either: (a) an explicit `null_fields: [String]` parameter to signal intentional clearing, or (b) a JSON Merge Patch (`application/merge-patch+json`) approach where `null` in the body means "clear".
- Both alternatives add complexity that is not justified by current requirements.
- This is a known limitation, not a bug. The gap is documented here so it can be addressed when a clearing use case arises.

---

## Consequences

**Positive:**
- Simple, readable service code.
- No risk of accidentally clearing a field due to a missing optional parameter.

**Negative:**
- Clients cannot clear nullable fields (description, location_id, category_id, barcode, expires_at) once set.
- If a clearing use case arises, a more sophisticated partial-update approach (JSON Merge Patch or an explicit `clear_fields` list) will be needed.

---

## Alternatives Considered

### JSON Merge Patch (`RFC 7396`)
Use `Content-Type: application/merge-patch+json` where a `null` value in the body signals "clear this field". Rejected for now — requires custom deserialization (serde's default `Option<T>` cannot distinguish `absent` from `null`; needs a three-state wrapper like `Option<Option<T>>`).

### Explicit `clear_fields` array
Accept a `"clear_fields": ["description", "location_id"]` parameter alongside the update body. Rejected for now — adds surface area without a current use case.

---

## Related

- P6-019 review finding (PR feat/tracking-domain, 2026-04-15)
- ADR-015: DB/API Type Separation (established row vs. response type pattern)
