# e03s03 Preview Compare And Return Path

## 1. Metadata

- **ID:** e03s03
- **Type:** feat
- **Context:** notes-domain
- **Status:** todo
- **BCP:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want to preview or compare another note and reliably return to my draft so that I can reuse knowledge without losing current work.

## 3. Problem

Opening related knowledge can destroy editor context or make comparison unusable on narrow screens; automatic merging would also exceed user-controlled scope.

## 4. Outcome

The owner can read a safe Markdown preview, compare two intact notes side by side where space permits or sequentially on mobile, copy text, and return to the exact working draft context.

## 5. In Scope

- Markdown reading preview for a note.
- Desktop/tablet comparison where usable and sequential comparison on narrow screens.
- Explicit Open for editing, Copy, and Back to draft actions with preserved draft, cursor, scroll, and originating route context.

## 6. Out Of Scope

- Automatic merge/consolidation, diff engine, collaborative editing, recall ranking, automatic links, and mutation of either note from preview.

## 7. Domain Terms

- **Origin draft:** The editor state active before preview/compare navigation.
- **Return path:** Explicit navigation restoring the origin draft and useful editor context.
- **Compare:** Read-only inspection of two distinct note contents; not merging.

## 8. Preconditions

- e03s01 provides note editing, draft retention, canonical Markdown, and local recovery behavior.

## 9. Dependencies

- Existing Vue routing and note APIs are sufficient. Use the `[OK]` `markdown-it` renderer with `[OK]` DOMPurify sanitization; add exact compatible versions through Bun during implementation.

## 10. Data

Preview uses acknowledged canonical Markdown. Origin context stores note/draft identity, draft revision, route, cursor selection, and scroll anchor separately from server-response caches. Copy changes only the clipboard.

## 11. API

Read operations fetch owner-scoped note identity, canonical Markdown, revision, and display metadata. Preview/compare commands make no mutation request.

## 12. UI

Desktop uses a readable split only above a tested width; otherwise comparison is sequential. The origin draft remains visually identifiable. Back restores the origin, while Open explicitly changes the editing target after preserving the origin draft.

## 13. Security

Render Markdown with raw HTML disabled or sanitized under an explicit allowlist. Reject script/event-handler execution and unsafe URL schemes. Preview APIs enforce owner scope and do not leak inaccessible note existence.

## 14. Failure Modes

- Preview load failure leaves the origin draft intact and offers Back/retry.
- A missing or changed target is reported without replacing origin state.
- Clipboard denial selects/exposes copyable text and reports the failure.
- Restoration failure falls back to retained local draft content rather than server cache.

## 15. Concurrency

Preview snapshots carry server revision and do not overwrite newer cache or draft data. If the target changes while open, the current preview stays stable until explicit refresh; returning always restores the origin draft revision captured locally, including edits made before navigation.

## 16. Accessibility

Preview headings preserve semantic order, controls have accessible names, split panes have logical keyboard order, focus returns to the originating control/editor position, and narrow-screen actions meet touch target requirements.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Non-destructive preview, compare, and return

Inspecting another note does not mutate either note or discard the active draft and context.

```gherkin
Scenario: Compare another note on a wide screen
  Given the owner has an unsaved modified origin draft
  When they open another note for comparison
  Then both notes are readable without changing either record
  And Back restores the origin draft cursor and scroll context
```

### Edge Case

```gherkin
Scenario: Compare on a narrow Android viewport
  Given the owner has an unsaved origin draft
  When they preview another note sequentially
  And they activate Back to draft
  Then the complete origin draft is restored
  And no side-by-side layout makes either note unusably narrow

Scenario: Preview rendering receives unsafe Markdown HTML
  Given a note contains a script or unsafe URL scheme
  When the note is previewed
  Then executable content does not run
  And the canonical Markdown remains unchanged
```

## 18. Test Strategy

Use renderer security tests, component tests for read-only commands and restoration, and Playwright workflows at desktop and Android viewports covering modified drafts, focus/cursor/scroll return, target failure, and clipboard denial.

## 19. Implementation Notes

Preserve origin draft state before navigation and restore it from draft state, not a server response cache. Prefer one responsive compare component over separate desktop/mobile feature behavior; no merge abstraction is justified.

## 20. Definition Of Done

- Preview rendering is safe and does not mutate canonical Markdown.
- Wide and narrow compare paths preserve both notes and restore origin context.
- Keyboard, focus, and touch behavior is tested.
- `just test`, then `just lint`, then `just build` pass in that order.
- After those gates, `just preflight` reports no new security findings in the affected preview and navigation paths.
