# PR Review: feat/web-client → main

**Date:** 2026-04-15
**Feature:** Context/Features/008-WebClient/
**Branch:** feat/web-client
**PR:** #9 — feat(web): Feature 008 — SvelteKit web client
**Reviewers:** code-reviewer, pr-test-analyzer, silent-failure-hunter (automated)
**Status:** ✅ Resolved

## Summary

21 findings across 4 categories. 11 fix-now issues (including 2 critical: repository
error swallowing across 9 files, and `uploadData()` missing error boundary), 6 missing
tasks (primarily test coverage gaps against FA-002, FA-003, FA-006, FA-009), 1
architectural concern (missing logging infrastructure), and 1 convention gap ($effect
cleanup pattern inconsistently applied). Strengths: admin guard, PowerSync browser
guard, auth hook tests, and quest-transitions layered test coverage are all solid.

## Findings

### Fix-Now

#### [FIX] P9-001: Input component broken label-input association
- **File:** `apps/web/src/lib/components/primitives/Input.svelte:28`
- **Severity:** Critical
- **Detail:** `<input>` uses raw `{id}` prop (defaults to `undefined`) but `<label>`
  uses `for={inputId}` (the derived fallback ID). When no `id` prop is passed, the
  label's `for` points to a generated ID like `input-a3f2g9`, but the `<input>` has
  no `id` at all. Label clicks don't focus the input; `aria-describedby` references
  are also broken. The `$derived` on line 20 exists for this purpose but isn't wired
  up. Fix: change line 28 from `{id}` to `id={inputId}`.
- **Status:** ✅ Fixed
- **Resolution:** Changed `{id}` to `id={inputId}` on the `<input>` element in Input.svelte:28

#### [FIX] P9-002: `$derived()` used where `$derived.by()` is required
- **File:** `apps/web/src/routes/+page.svelte:12`
- **Severity:** Critical
- **Detail:** `const greeting = $derived(() => { ... })` stores a function reference,
  not a string. Svelte 5's derivation tracking and caching are bypassed — the body
  re-executes on every render rather than when dependencies change. The call site on
  line 77 (`{greeting()}`) masks the bug. Fix: use `$derived.by(() => { ... })` and
  change line 77 to `{greeting}`.
- **Status:** ✅ Fixed
- **Resolution:** Changed `$derived(() =>` to `$derived.by(() =>` and updated call site from `{greeting()}` to `{greeting}`

#### [FIX] P9-003: All 9 repositories silently swallow database errors
- **File:** `apps/web/src/lib/repositories/` — ~15 sites across all 9 `.svelte.ts` files
- **Severity:** Critical
- **Detail:** Every reactive subscription uses an async IIFE inside `$effect()` with
  no `.catch()`. If `db.watch()` throws at any point (schema mismatch after migration,
  SQL error, PowerSync internal error, quota exceeded), the rejection disappears
  entirely. Users see empty or stale data with no indication of failure. Fix: add
  `.catch((err) => console.error('[RepoName] watch failed:', err))` to every async
  IIFE. See also P9-021 for the logging infrastructure gap.
- **Status:** ✅ Fixed
- **Resolution:** Added `.catch((err) => console.error('[repo] watch failed:', err))` to all async IIFEs across all 9 repository files (~15 sites total)

#### [FIX] P9-004: `uploadData()` missing try/catch — `batch.complete()` never called on failure
- **File:** `apps/web/src/lib/sync/connector.ts:28-67`
- **Severity:** Critical
- **Detail:** If any CRUD entry throws (non-2xx response, network error), execution
  exits at the throw site and `batch.complete()` on line 66 is never reached. PowerSync
  retries the entire batch, re-uploading entries that already succeeded. If the server
  is not strictly idempotent, this causes duplicate writes. A permanently invalid record
  creates an infinite retry loop that blocks all future sync for the user. Zero logging
  on failure. Fix: wrap the loop in try/catch, log with context, re-throw so PowerSync
  knows the upload did not complete.
- **Status:** ✅ Fixed
- **Resolution:** Wrapped the CRUD loop and `batch.complete()` in try/catch; logs `[connector] uploadData failed:` and re-throws

#### [FIX] P9-005: Root layout `fetch('/api/auth/me')` unprotected — crashes entire app
- **File:** `apps/web/src/routes/+layout.server.ts:9-10`
- **Severity:** High
- **Detail:** No try/catch around `fetch()` or `response.json()`. A network error or
  malformed response body crashes the root layout load function, taking down every
  page. The settings form actions in the same codebase correctly use try/catch — this
  is an oversight. Fix: wrap in try/catch with graceful fallback (`isAdmin: false,
  user: null`).
- **Status:** ✅ Fixed
- **Resolution:** Wrapped entire profile fetch + JSON parse in try/catch; returns `{ isAdmin: false, user: null }` on any failure

#### [FIX] P9-006: Admin layout unprotected fetch and JSON parse
- **File:** `apps/web/src/routes/admin/+layout.server.ts:6,8`
- **Severity:** High
- **Detail:** `fetch()` on line 6 has no network-error protection; `res.json()` on
  line 8 has no `.catch()`. A malformed API response (proxy error page, partial body)
  crashes all admin routes instead of redirecting to `/`. Fix: wrap both calls in
  try/catch and throw `redirect(303, '/')` on any failure.
- **Relates to:** FA-002 (admin access control), S014 (is_admin risk gate)
- **Status:** ✅ Fixed
- **Resolution:** Wrapped fetch + JSON parse in try/catch; uses `isRedirect()` to re-throw SvelteKit redirects, falls through to `redirect(303, '/')` on network/parse error

#### [FIX] P9-007: Empty catch block in logout action
- **File:** `apps/web/src/routes/settings/+page.server.ts:78`
- **Severity:** High
- **Detail:** `catch { }` with no logging. Server-side sessions remain active if the
  logout API consistently fails, with zero visibility. Fix: `catch (err) {
  console.error('[settings] Server-side logout failed:', err); }`.
- **Status:** ✅ Fixed
- **Resolution:** Added `console.error('[settings] Server-side logout failed:', err)` to catch block

#### [FIX] P9-008: Empty catch block in admin health check
- **File:** `apps/web/src/routes/admin/health/+page.server.ts:8`
- **Severity:** High
- **Detail:** The health page is the diagnostic tool for operators. Its catch block
  silently swallows all errors and returns fallback data (`'unreachable'`), making it
  impossible to distinguish "API is down" from "the health check code is broken". Also
  `res.json()` can throw on a proxy error page and is swallowed by the same catch. Fix:
  log before returning fallback.
- **Status:** ✅ Fixed
- **Resolution:** Added `console.error('[admin/health] Health check failed:', err)` before returning fallback data

#### [FIX] P9-009: Settings `load()` fetch unprotected
- **File:** `apps/web/src/routes/settings/+page.server.ts:6`
- **Severity:** High
- **Detail:** `fetch('/api/auth/me')` in the `load` function has no try/catch. A
  network error crashes the settings page load, even though the form actions in the
  same file correctly use try/catch. Fix: wrap consistently with the actions below it.
- **Status:** ✅ Fixed
- **Resolution:** Wrapped `fetch('/api/auth/me')` and `res.json()` in try/catch; returns `{ displayName: '', email: '' }` on failure

#### [FIX] P9-010: Today page `$effect` missing `AbortController` cleanup
- **File:** `apps/web/src/routes/+page.svelte:26-36`
- **Severity:** Medium
- **Detail:** `for await` loop over `client.watch()` with no abort mechanism. If the
  effect re-triggers (HMR, future refactoring), the old iterator keeps running
  alongside the new one. Fix: add `AbortController` and return cleanup function,
  matching the pattern already established in `note.svelte.ts`.
- **Status:** ✅ Fixed
- **Resolution:** Added `AbortController`, passed `{ signal: controller.signal }` to `client.watch()`, returns `() => controller.abort()`, added `.catch()` for error visibility

#### [FIX] P9-011: Fragile `document.querySelector('.editor')` in note editor
- **File:** `apps/web/src/routes/knowledge/[id]/+page.svelte:266`
- **Severity:** Medium
- **Detail:** Class-based global DOM query to find the textarea. Non-idiomatic in
  Svelte; breaks silently if `.editor` is reused elsewhere. Fix: `bind:this` on the
  textarea and reference directly.
- **Status:** ✅ Fixed
- **Resolution:** Added `let editorTextarea: HTMLTextAreaElement | null = $state(null)`, added `bind:this={editorTextarea}` on textarea, replaced `document.querySelector('.editor')` with `editorTextarea`

#### [FIX] P9-019: `subscribeToStreams()` has no logging on connection failure
- **File:** `apps/web/src/lib/sync/index.ts:34-36`
- **Severity:** Medium
- **Detail:** `await client.connect()` errors propagate with no log entry. PowerSync
  connection failures are invisible in production. Fix: wrap in try/catch, log with
  context, re-throw.
- **Status:** ✅ Fixed
- **Resolution:** Wrapped `client.connect()` in try/catch; logs `[sync] PowerSync connection failed:` and re-throws

#### [FIX] P9-020: `fetchCredentials()` fetch has no network-error protection
- **File:** `apps/web/src/lib/sync/connector.ts:9-26`
- **Severity:** Medium
- **Detail:** `fetch('/api/auth/powersync-token')` has no try/catch. Network errors
  propagate with no log. Auth token refresh failures are invisible regardless of
  whether the PowerSync SDK retries. Fix: catch, log, re-throw.
- **Status:** ✅ Fixed
- **Resolution:** Wrapped `fetch()` call in try/catch; logs `[connector] fetchCredentials network error:` and re-throws

### Missing Tasks

#### [TASK] P9-012: NoteEditor.spec.ts never renders the component — FA-006 unverified
- **File:** `apps/web/src/routes/knowledge/NoteEditor.spec.ts:98-213`
- **Severity:** Critical
- **Detail:** The entity_relations INSERT tests and backlinks test call `db.execute()`
  directly with hardcoded SQL — the `NoteEditor` component is never rendered or
  invoked. Verifies only the SQL the test author wrote, not the SQL the component emits.
  If `selectLinkedNote()` is refactored, tests stay green while the bug ships. The
  snapshot invariant test (lines 225-247) asserts on a literal object the test
  constructs. FA-006 coverage depends entirely on the E2E test in
  `sync-integration.e2e.ts`, which is conditional on `window.__altairSync`.
  Fix: render the actual NoteEditor page component and simulate `[[` input, or delete
  these tests in favor of the E2E.
- **Relates to:** FA-006 (wiki-link autocomplete creates entity_relations row)
- **Status:** ✅ Task created
- **Resolution:** Added as S026-T in Steps.md Phase 7

#### [TASK] P9-013: ItemDetail.spec.ts validates copied validation logic — FA-009 unverified
- **File:** `apps/web/src/routes/tracking/items/[id]/ItemDetail.spec.ts:17-28`
- **Severity:** Critical
- **Detail:** `validateConsumption` is defined inside the test file, not imported from
  the component. If the component's boundary condition changes (e.g., `< 0` → `<= 0`),
  tests pass green while the bug ships. FA-009 (negative-quantity block before outbox
  write) is unverified. Fix: extract validation to `$lib/utils/validate-consumption.ts`
  and import in both component and test, OR render the component and drive the input.
- **Relates to:** FA-009 (consumption block before outbox entry)
- **Status:** ✅ Task created
- **Resolution:** Added as S027-T in Steps.md Phase 7

#### [TASK] P9-014: ShoppingList CSS assertions are no-ops in happy-dom — FA-010 uncovered
- **File:** `apps/web/src/routes/tracking/shopping-lists/ShoppingList.spec.ts:32-126`
- **Severity:** High
- **Detail:** Both visual CSS tests fall back to `expect(container).toBeTruthy()` when
  scoped Svelte styles aren't found in happy-dom. Confirms only that the component
  mounted without throwing. Fix: add Playwright E2E test using `getComputedStyle` on
  the checkbox element; delete the no-op unit tests.
- **Relates to:** FA-010 (shopping list pill checkbox visual)
- **Status:** ✅ Task created
- **Resolution:** Added as S028-T in Steps.md Phase 7

#### [TASK] P9-015: FA-003 sync stream coverage partial and conditional
- **File:** `apps/web/src/lib/sync/sync-integration.e2e.ts:84-103`
- **Severity:** High
- **Detail:** Spec requires all 5 baseline streams; test queries only 3 (`quests`,
  `notes`, `tracking_items`). Domain table assertion is wrapped in
  `if (syncClientPresent)` — if `window.__altairSync` is not exposed, the only
  passing assertion is IndexedDB creation. Aggregate `questCount + noteCount +
  itemCount > 0` permits two of three tables to be empty. Fix: expose `__altairSync`
  unconditionally; assert all 5 streams individually; use per-table
  `expect(count).toBeGreaterThan(0)`.
- **Relates to:** FA-003 (PowerSync sync round-trip, all streams)
- **Status:** ✅ Task created
- **Resolution:** Added as S029-T in Steps.md Phase 7

#### [TASK] P9-016: FA-002 has no E2E coverage for non-admin redirect from `/admin`
- **File:** `apps/web/src/routes/auth/` (gap — no test file)
- **Severity:** Medium
- **Detail:** `admin.spec.ts` unit test covers server-side guard logic correctly, but
  no E2E test validates the redirect fires in a running SvelteKit app with a real
  session. Fix: add short Playwright test that logs in as non-admin user, navigates to
  `/admin`, and asserts `page.url()` redirects to `/`.
- **Relates to:** FA-002 (non-admin redirect), S014 (is_admin risk gate)
- **Status:** ✅ Task created
- **Resolution:** Added as S030-T in Steps.md Phase 7

#### [TASK] P9-017: `uploadData` partial-failure path untested
- **File:** `apps/web/src/lib/sync/schema.spec.ts` (gap)
- **Severity:** High
- **Detail:** No test covers `uploadData` throwing mid-batch. The fix for P9-004 must
  be accompanied by a test where `getCrudBatch` returns a multi-entry batch and the
  fetch mock rejects on the second entry; asserts `batch.complete()` is not called.
- **Relates to:** P9-004 fix (uploadData error handling)
- **Status:** ✅ Task created
- **Resolution:** Added as S031-T in Steps.md Phase 7

### Architectural Concerns

#### [ADR] P9-021: No logging infrastructure exists in `apps/web/`
- **File:** Entire `apps/web/src/`
- **Severity:** High
- **Detail:** The entire web client has no `logError`, `logForDebugging`, or Sentry
  integration. The only logging call in the application is a single `console.error` in
  `hooks.server.ts:26`. This is the root cause enabling most error-handling gaps above
  — every catch block has nowhere meaningful to log. All P9-003 through P9-020 fixes
  will use `console.error` as a stopgap, which is insufficient for production
  observability. A minimal `logError(message, context)` wrapper (Sentry in prod,
  `console.error` in dev) should be established before those fixes land, or immediately
  after as a follow-on task.
- **Relates to:** P9-003, P9-004, P9-005, P9-007, P9-008, P9-019, P9-020
- **Status:** ✅ ADR created
- **Resolution:** ADR-021 (Context/Decisions/ADR-021-web-client-logging-strategy.md)

### Convention Gaps

#### [RULE] P9-018: `$effect` cleanup pattern inconsistently applied across repositories
- **Files:** `quest.svelte.ts:30`, `epic.svelte.ts:26`, `initiative.svelte.ts:26`,
  `routine.svelte.ts:26` (missing cleanup); `note.svelte.ts`, `tag.svelte.ts`,
  `item.svelte.ts`, `shopping-list.svelte.ts` (correct pattern with AbortController
  or active flag)
- **Severity:** Medium
- **Detail:** 4 of 9 repository files use `for await (const result of client.watch())`
  inside a module-level `$effect()` with no `AbortController` and no cleanup return
  function. If the effect re-triggers, old iterators accumulate. The correct pattern
  is already used in the other 4 repositories. The rule file `svelte.md` references
  `$effect()` but does not specify the cleanup requirement for async iterators.
  Suggested addition to `.claude/rules/svelte.md`:
  > "Any `$effect()` containing an async iterator (`for await`) must include an
  > `AbortController` and return a cleanup function that calls `controller.abort()`."
- **Status:** ✅ Rule updated
- **Resolution:** Added `$effect` async iterator cleanup requirement and `.catch()` requirement to `.claude/rules/svelte.md` Reactive Patterns section

## Resolution Checklist
- [x] All [FIX] findings resolved (P9-001 through P9-011, P9-019, P9-020)
- [x] All [TASK] findings added to Steps.md or resolved (P9-012 through P9-017)
- [x] All [ADR] findings have ADRs created or dismissed (P9-021)
- [x] All [RULE] findings applied or dismissed (P9-018)
- [x] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-16
**Session:** review-resolve — all 21 findings executed inline

| Category | Total | Resolved |
|---|---|---|
| [FIX] | 13 | 13 |
| [TASK] | 6 | 6 |
| [ADR] | 1 | 1 |
| [RULE] | 1 | 1 |
| **Total** | **21** | **21** |
