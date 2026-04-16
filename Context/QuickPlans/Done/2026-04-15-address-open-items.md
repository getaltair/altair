# Quick Plan: Address Feature 008 Open Items

**Date:** 2026-04-15
**Task:** Address the two open items remaining after Feature 008 validation pass.

---

## Task
Fix the two non-blocking issues identified by S025 validator:

1. **`window.__altairSync` not exposed** — `apps/web/src/lib/sync/index.ts` does not assign the PowerSync client to `window.__altairSync`, so the deep SQL verification path in `sync-integration.e2e.ts` (FA-003) is always skipped. The sync *works*, but the test cannot introspect row counts.

2. **5 `bun run check` warnings** — all non-blocking but worth cleaning:
   - `src/routes/guidance/epics/[id]/+page.svelte:157` — missing `line-clamp` standard property
   - `src/routes/guidance/initiatives/+page.svelte:147` — missing `line-clamp` standard property
   - `src/routes/settings/+page.svelte:21,22` — `state_referenced_locally` (reactive `data` captured outside `$derived`)

---

## Goal
- `bun run check` exits 0 with **0 warnings** (or ≤ 2 if the line-clamp is from a vendor-prefix that can't be avoided — accept with a comment)
- FA-003 sync integration test has a working `window.__altairSync` path so that the IndexedDB row-count assertion can execute on the happy path

---

## Approach

### Item 1 — Expose PowerSync client on window (browser only)
**File:** `apps/web/src/lib/sync/index.ts`

In the `getSyncClient()` function, after the singleton is initialized, add:
```ts
if (browser && typeof window !== 'undefined') {
  (window as any).__altairSync = client;
}
```
This is guarded by the existing `if (browser)` check pattern already in the file. Exposes the client for test introspection without affecting production behaviour.

### Item 2a — line-clamp warnings
**Files:** `apps/web/src/routes/guidance/epics/[id]/+page.svelte:157`, `apps/web/src/routes/guidance/initiatives/+page.svelte:147`

Add the standard `line-clamp` property alongside the vendor-prefixed one:
```css
-webkit-line-clamp: 2;
line-clamp: 2;  /* add this */
```

### Item 2b — state_referenced_locally in settings
**File:** `apps/web/src/routes/settings/+page.svelte:21,22`

Wrap the `data` references in `$derived`:
```ts
// Before
let displayName = $state(data.displayName);
let email = $state(data.email);

// After
const displayName = $derived(data.displayName);
const email = $derived(data.email);
// Or: initialise from $derived snapshot if editing is needed:
let displayName = $state($derived.by(() => data.displayName));
```
Read the file to confirm the exact pattern before editing.

---

## Verification
```bash
cd apps/web
bun run check     # 0 errors, 0 warnings (or ≤ 2 with justified comment)
bun run test --run  # 97/97 pass
```

Also confirm `window.__altairSync` is defined in browser context: can be verified by reading the export from `sync/index.ts`.

---

## Risks
- `state_referenced_locally` fix: changing `$state` to `$derived` may break two-way binding if the settings form mutates these values. Read the component first — if they're used as form inputs that the user edits, keep them as `$state` but initialise with `data.X` directly (the warning may be acceptable in that case and can be suppressed with `// @ts-ignore` or a `svelte-ignore`).
- Exposing `window.__altairSync` is test-only instrumentation. It should never be used in production logic.
