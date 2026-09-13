# e06s04: Preview Link And Dismiss Suggestions

## 1. Metadata

- **Story ID:** e06s04
- **Epic:** e06 Search And Contextual Recall
- **Type:** feat
- **Context:** recall-interactions
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want to preview, explicitly link, or dismiss a stable suggestion without the target moving during my action, so recall remains trustworthy on pointer, keyboard, and touch devices.

## 3. Problem

Live result refresh can replace or reorder a suggestion between action intent and activation. That can preview, link, or dismiss the wrong record. Dismissal also needs deterministic context scope so unchanged input does not immediately revive a rejected candidate.

## 4. Outcome

This story owns the displayed-target freeze contract. The displayed target is frozen from the first active pointer, keyboard, or touch transition through completion or cancellation. At most the newest completed eligible batch waits and publishes afterward. Preview preserves the draft and return path, link uses a separate revalidated command, and dismissal is deterministic for the unchanged context key.

## 5. In Scope

- Preview authoritative candidate details without mutating the record or draft.
- Open a candidate with a reliable return path and preserved draft state.
- Explicitly link a saved active record, or use e05s02 for an unsaved source record.
- Dismiss a candidate for the deterministic current context key.
- Freeze displayed target and defer batch publication for all active pointer, keyboard, and touch transitions.

## 6. Out Of Scope

- Automatic linking, editing, quantity changes, task-status changes, or note merging.
- Permanent global blocklists or a dismissal review queue.
- Trusting recall snapshots as mutation authorization or current data.
- Two-pane comparison on layouts too narrow to preserve usable editors.
- Suggestions for archived records by default.

## 7. Domain Terms

- **Displayed target:** Stable identity and source revision bound to the rendered action control.
- **Interaction hold:** Interval during which result publication cannot replace/reorder the displayed target.
- **Pending batch:** Newest completed eligible batch retained during an interaction hold.
- **Context-scoped dismissal:** Candidate suppression keyed by candidate identity plus deterministic context key.
- **Return path:** Navigation state that restores the originating draft, cursor/scroll context where applicable, and recall surface.

## 8. Preconditions

- e06s03 has published an immutable current candidate batch.
- Each rendered action captures stable identity, source revision, context key, session, context revision, and generation.
- e05 relationship commands are available for explicit link actions.
- Draft recovery and navigation boundaries are available in each editor.

## 9. Dependencies

- e06s03 latest-only coordinator and immutable batches.
- e05s01 link/unlink for saved active records.
- e05s02 atomic create-and-link for unsaved source records.
- Domain preview/detail and draft preservation from e02 through e04.
- e01s03 shared `DraftSaveCoordinator` for editor save acknowledgements, conflicts, retry, and draft ownership across preview and return navigation.
- Deterministic context-key definition in `tech-stack.md`.

## 10. Data

- Bind each action control to candidate stable identity and displayed source revision, not list index.
- Interaction hold records input modality, target identity, start generation, and active transition state.
- Keep only the newest completed batch whose session/revision/generation remains current while held.
- Dismissal key combines candidate stable identity with the SHA-256 context key; unchanged normalized context remains suppressed.
- Dismissal is client/session workspace state for v0.1 unless an existing accepted persistence boundary requires otherwise; it must never mutate the candidate record.

## 11. API

- Preview reads current authoritative detail by stable identity and labels archive/revision changes.
- Link is a separate authenticated CSRF-protected command that revalidates source/target revision, archive state, and existing relationship state.
- Dismissal does not call a record mutation endpoint.
- Recall snapshot labels and revisions are hints for presentation, never authorization for mutation.
- Link success appears only after server acknowledgement.
- Link/create-and-link responses enter editor save or conflict state only through the e01s03 `DraftSaveCoordinator`; recall interaction state does not duplicate those transitions.

## 12. UI

- Pointer hold begins on `pointerdown` and ends after click/activation or `pointercancel`/capture loss.
- Keyboard hold begins on action-key `keydown` for Enter or Space and ends after activation or corresponding `keyup`, blur, or cancellation.
- Touch uses pointer events where supported; fallback hold begins on `touchstart` and ends after activation, `touchend`, `touchcancel`, or gesture cancellation.
- While held, keep the rendered target, focus, ordering, and action labels stable; retain only the newest eligible completed batch.
- After release, publish that batch only if still current; otherwise discard it. Nested/modal preview extends the hold until the initiating activation is safely bound to the captured identity.

## 13. Security

- Preview and link require the authenticated session; link also requires CSRF protection.
- Revalidate both endpoints and revisions server-side to prevent stale-target or archived-target mutation.
- Never derive authorization from a candidate's displayed kind, label, or list position.
- Keep preview payloads out of shared caches and personal content out of logs.
- Dismissal keys contain hashes and stable IDs, not raw context text.

## 14. Failure Modes

- Candidate changes/archives after display: preview reports current state; link returns a typed conflict through the e01s03 `DraftSaveCoordinator` and preserves the draft.
- New batch completes during interaction: defer it without changing target, focus, or ordering.
- Several batches complete while held: retain only the newest still-eligible batch.
- Session/revision/generation changes before release: discard pending batch and do not revive the old context.
- Pointer cancel, keyboard blur, touch cancel, lost capture, page hide, or disposal: end the hold deterministically and apply normal latest-only eligibility rules.
- Preview/navigation failure: retain originating draft and return path.

## 15. Concurrency

The freeze contract must cover every active transition:

| Modality/state | Hold starts | Hold completes or cancels | Publication rule |
|---|---|---|---|
| mouse/pen | target `pointerdown` | bound activation, `pointercancel`, lost capture, blur, page hide, or disposal | target stays captured; newest eligible batch waits |
| touch pointer events | target `pointerdown` | bound activation, `pointerup`, `pointercancel`, gesture cancellation, page hide, or disposal | target stays captured; newest eligible batch waits |
| touch fallback | `touchstart` | bound activation, `touchend`, `touchcancel`, gesture cancellation, page hide, or disposal | target stays captured; newest eligible batch waits |
| keyboard Enter | action-key `keydown` | bound activation, `keyup`, blur, page hide, or disposal | focused target and ordering stay fixed |
| keyboard Space | action-key `keydown` | activation after `keyup`, blur, page hide, or disposal | focused target and ordering stay fixed through activation |
| preview open/close | initiating activation is captured | preview binds identity; return restores origin | refresh cannot retarget the preview or return path |

Nested modality signals for one physical action form one hold. Release publishes at most one newest eligible batch after checking session, context revision, dispatch generation, context key, visibility, and disposal state.

## 16. Accessibility

- Preview, open, link, and dismiss are named controls reachable in logical keyboard order.
- Focus remains on the same stable target during refresh and returns predictably after preview or dismissal.
- Dismissal and link outcomes use polite status announcements.
- Touch targets meet the application's mobile sizing standard and do not depend on hover.
- Reduced automatic presentation does not remove manual recall or action access.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Stable explicit suggestion actions and deterministic dismissal

Every suggestion action uses the identity displayed when interaction begins; no refresh can retarget it, and dismissal suppresses that candidate only for the unchanged deterministic context.

```gherkin
Scenario: Happy path previews, links, and dismisses stable suggestions
  Given candidate A is displayed for the current unsaved note context
  When the user begins preview interaction with candidate A and a newer batch places candidate B in that position
  Then candidate A remains the bound preview target through activation
  And the newer eligible batch publishes only after the interaction completes
  When the user explicitly links candidate A
  Then the server revalidates and acknowledges the relationship before linked state appears
  When the user dismisses candidate A
  Then candidate A remains hidden while the deterministic context key is unchanged
```

### Edge Case

```gherkin
Scenario Outline: Edge interaction transitions never retarget an action
  Given an interaction with candidate A starts by <start>
  And one or more newer batches complete during the hold
  When the interaction ends by <end>
  Then <outcome>
  Examples:
    | start | end | outcome |
    | mouse pointerdown | click activation | candidate A receives the action and only the newest eligible batch may publish afterward |
    | pen pointerdown | pointercancel | no action fires and the newest eligible batch may publish afterward |
    | touch pointerdown | gesture cancellation | no action fires and no target is substituted mid-gesture |
    | touchstart fallback | touchend activation | candidate A receives the action once and no synthetic click retargets it |
    | Enter keydown | keyup after activation | focused candidate A receives the action and ordering stays fixed |
    | Space keydown | keyup activation | focused candidate A stays fixed through keyup and receives the action |
    | keyboard keydown | blur before activation | no action fires and the hold ends safely |
    | any active hold | page hide or disposal | no pending batch publishes into the paused or disposed session |

Scenario Outline: Edge dismissal is deterministic and scoped
  Given candidate A is dismissed for a context key
  When <change> occurs
  Then <dismissal result>
  Examples:
    | change | dismissal result |
    | prose changes only by case or repeated whitespace | candidate A remains suppressed |
    | identifier punctuation or case changes | a different key permits candidate A to be reconsidered |
    | project-association IDs are supplied in another order | candidate A remains suppressed because IDs are sorted canonically |
    | recall-relevant content changes materially | the new key permits candidate A to be reconsidered |
    | another draft has equivalent visible prose but a different draft identity | dismissal does not become a permanent cross-draft ban |
```

## 18. Test Strategy

- Unit-test canonical context serialization: fixed field order, explicit nulls, sorted IDs, Unicode NFC, prose case/whitespace normalization, identifier preservation, and SHA-256 stability.
- Use fake timers/events to test every row in the interaction transition table and nested pointer/touch synthetic-click behavior.
- Permute multiple batch completions, context changes, hide/resume, disposal, cancellation, preview opening, and interaction release; assert at most one eligible publication.
- Component-test stable keys/identities rather than list indexes, focus retention, announcements, e01s03 `DraftSaveCoordinator` conflict UI, and return-path restoration.
- Browser-test real pointer, keyboard Enter/Space, touch emulation, pointer cancellation, blur, page visibility, responsive preview, and draft preservation.
- Integration-test link revalidation for changed revision, archived target, existing link, idempotent replay, and server acknowledgement.

## 19. Implementation Notes

- Purpose: recall interactions own displayed-target freeze and translate immutable suggestions into safe user-controlled preview, link, and local dismissal actions.
- Callers: related-content panels and compact recall surfaces in every domain editor.
- Contracts: stable-identity binding, full-modality interaction hold, newest-only deferred publication, server mutation revalidation, deterministic context dismissal, focus/draft/return preservation, and e01s03 ownership of editor save/conflict state.
- Place the hold in the shared recall coordinator, not individual buttons; render controls keyed by stable identity.
- Reason for Depth: centralized interaction holding is required to prevent independent desktop/mobile components from creating different retargeting races.
- Use browser Pointer Events and existing Vue facilities; no new external package is proposed.

## 20. Definition Of Done

- Every pointer, keyboard, touch, cancellation, visibility, preview, and disposal transition has deterministic automated coverage.
- No generated or refreshed batch can change the target of an active action.
- Dismissal key fixtures are stable and prove both unchanged suppression and changed-context reconsideration.
- Preview and link preserve drafts, revalidate current authority, and report success only after acknowledgement.
- Accessibility and responsive browser tests pass.
- Preview/link integrations preserve the e01s03 `DraftSaveCoordinator` save/conflict contract without a duplicate state machine.
- `just test`, `just lint`, `just build`, and `just preflight` pass before task statuses change.
- Preflight reports no new security findings in affected recall-interaction and relationship-integration paths.
