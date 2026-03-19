# F3: Spec Validation Evidence

**Task:** Final Verification Task 3 from p3-008-api-docs
**Date:** 2026-03-14
**Status:** COMPLETED WITH ISSUES

## Server Startup

```bash
cd apps/server && source .env && export DATABASE_URL && cargo run
```

**Result:** Server started successfully on port 3000
**Health Check:** `{"database":"connected","status":"healthy","timestamp":"2026-03-14T20:55:06.515053263+00:00"}`

## OpenAPI Spec Fetch

```bash
curl -s http://localhost:3000/docs/openapi.json > /tmp/altair-spec.json
```

**Result:** Spec fetched successfully
**OpenAPI Version:** 3.1.0
**File saved:** `.sisyphus/evidence/openapi-spec.json`

## Endpoint Count

| Metric | Value | Expected |
|--------|-------|----------|
| Unique Paths | 20 | - |
| Total Operations | 31 | 25 |
| Auth Schemes | 1 | 1 |

### Unique Paths (20)

1. `/attachments`
2. `/auth/me`
3. `/core/households`
4. `/core/households/{id}`
5. `/core/households/{id}/memberships`
6. `/core/initiatives`
7. `/core/initiatives/{id}`
8. `/core/relations`
9. `/core/relations/{id}`
10. `/core/relations/{id}/status`
11. `/core/tags`
12. `/core/tags/{id}`
13. `/guidance/quests`
14. `/guidance/routines`
15. `/health`
16. `/knowledge/notes`
17. `/search`
18. `/sync/checkpoint`
19. `/tracking/items`
20. `/users/me`

## Auth Scheme Verification

```json
{
  "better_auth_session": {
    "type": "apiKey",
    "in": "cookie",
    "name": "better-auth.session_token"
  }
}
```

**Result:** PASS - Auth scheme defined correctly

## Redocly Validation

```bash
npx @redocly/cli lint /tmp/altair-spec.json
```

**Result:** FAILED - 18 errors, 20 warnings

### Errors (18)

1. **no-empty-servers** (1): Servers must be present
2. **operation-operationId-unique** (10): Duplicate operationIds:
   - `list` (used in households, initiatives, relations, tags)
   - `create` (used in households, initiatives, relations, tags)
   - `update` (used in initiatives, tags)
   - `me` (used in auth, users)
3. **security-defined** (8): Placeholder endpoints missing security:
   - `/attachments`
   - `/guidance/quests`
   - `/guidance/routines`
   - `/health`
   - `/knowledge/notes`
   - `/search`
   - `/sync/checkpoint`
   - `/tracking/items`

### Warnings (20)

- **info-license-strict** (1): License missing url/identifier
- **operation-2xx-response** (8): Placeholder endpoints missing 2XX response
- **operation-4xx-response** (11): Operations missing 4XX response
- **no-unused-components** (2): Unused schemas (HouseholdMembership, ListRelationsQuery)

## Verdict

### VALIDATION STATUS: **INVALID**

The OpenAPI spec has structural issues that prevent it from passing Redocly validation:

| Check | Status | Notes |
|-------|--------|-------|
| Spec Fetchable | PASS | Server serves spec at `/docs/openapi.json` |
| OpenAPI 3.1 Compliance | PARTIAL | Has errors preventing full validation |
| Endpoint Count | PASS | 31 operations (exceeds 25 expected) |
| Auth Scheme | PASS | `better_auth_session` defined correctly |
| OperationId Uniqueness | FAIL | 10 duplicate operationIds |
| Servers Field | FAIL | Missing servers definition |
| Security Coverage | FAIL | 8 endpoints lack security definition |

### Required Fixes

1. Add unique operationIds (e.g., `listHouseholds`, `createHousehold`, etc.)
2. Add `servers` field to spec
3. Add security definitions to placeholder endpoints (or mark as public)
4. Add proper 2XX/4XX responses to placeholder endpoints

## Files

- `.sisyphus/evidence/openapi-spec.json` - Full OpenAPI spec
- `.sisyphus/evidence/redocly-validation.txt` - Redocly output
- `.sisyphus/evidence/endpoint-count.txt` - Endpoint count
- `.sisyphus/evidence/auth-scheme.txt` - Auth scheme definition
