# e08s01: Touch-Usable Responsive Workflows

## 1. Metadata

- **Story ID:** e08s01
- **Epic:** e08 Cross-Device Release Readiness
- **Type:** feat
- **Context:** responsive-ui
- **Status:** todo
- **BCPs:** 3
- **Risk:** P1

## 2. User Story

As the workspace owner using Android or a desktop browser, I want every core
workflow adapted to my viewport and input method so that I can create, edit,
recover, and connect records without fighting a shrunken desktop interface.

## 3. Problem

A layout that merely scales down can hide actions, create horizontal scrolling,
make drag-only behavior inaccessible, and render compare or recall surfaces too
narrow to use. Release claims require measurable viewport and target criteria,
not subjective statements that the interface is responsive.

## 4. Outcome

Core project/task, note, resource, relationship, recall, attachment, archive,
and recovery workflows work at supported mobile and desktop dimensions. Mobile
uses deliberate single-column or sequential surfaces, reachable actions, visible
state, and touch targets large enough for reliable use.

## 5. In Scope

- Responsive navigation and domain lists/details/editors.
- Touch alternatives for board status changes and all pointer interactions.
- Compact recall, sequential note comparison, and reliable return paths.
- Draft, save, conflict, upload, archive, and degraded-state actions on mobile.
- Automated measurements at mobile and desktop viewport fixtures.

## 6. Out Of Scope

- Native Android/desktop applications, PWA installation, and full offline operation.
- Device-specific gestures as the only way to invoke an action.
- Pixel-identical layouts across browser engines.
- Tablet-only workflows that differ from both mobile and desktop behavior.

## 7. Domain Terms

- **Core workflow:** An accepted create, edit, status, preview, recall, link, recovery, or export interaction.
- **Mobile reference viewport:** `360x800` CSS pixels with touch input.
- **Minimum supported viewport:** `320x568` CSS pixels at 100% zoom.
- **Primary touch target:** A control that performs or advances a core workflow.
- **Reliable return path:** A persistent or browser-compatible route back to the unchanged originating draft/context.

## 8. Preconditions

- Domain workflows and semantic HTML exist before final responsive acceptance.
- Browser automation can emulate viewport, touch, reduced motion, and keyboard input.
- Tests use representative long labels, untitled records, validation errors, and loading states.
- Mobile checks run without browser zoom or desktop-site mode.

## 9. Dependencies

- e02 project lists, task lists/boards, status commands, and project details.
- e03 CodeMirror note editing, quick capture, preview, compare, and draft recovery.
- e04 resource list/cards/detail and quantity actions.
- e05 relationships and e06 recall preview/actions.
- e07 attachment, archive, restore, purge, and export surfaces.

## 10. Data

Responsive behavior does not create a second mobile data model. Routes, stable
record IDs, drafts, acknowledged server state, conflicts, and recall snapshots
remain shared contracts. Client viewport state is presentation-only and must not
be persisted as authoritative domain data. Navigation preserves the active draft
and return context through record IDs and client draft state.

## 11. API

- Mobile and desktop use the same authenticated Axum contracts and typed errors.
- No workflow requires a pointer-specific endpoint or device user-agent branch.
- Requests are not duplicated solely because a responsive surface changes layout.
- Status, link, save, recovery, and recall actions report server acknowledgement consistently across viewports.

## 12. UI

- At `360x800`, core content fits one viewport width with no page-level horizontal overflow; CodeMirror may scroll code lines internally.
- At `320x568`, navigation and every core action remain reachable without overlap or clipping.
- Primary touch targets are at least `44x44` CSS pixels; adjacent targets have at least `8` CSS pixels of separation unless their target boxes do not overlap and each remains `44x44`.
- Desktop at `1440x900` may use panels/side-by-side compare; mobile uses sequential preview/compare with a persistent Back to draft action.
- Task status has a tap/menu alternative to drag-and-drop, and recall is available without leaving the active editor.

## 13. Security

- Responsive variants expose no content before session validation and do not place secrets in routes or client logs.
- Hidden/off-canvas personal content is removed from accessibility and focus order, not merely translated off screen.
- Destructive confirmations remain explicit and cannot be bypassed by gesture overlap.
- Managed-device local-draft recovery controls remain discoverable at mobile width.

## 14. Failure Modes

- Network/save errors retain input and keep Retry and Copy reachable at minimum width.
- Conflict layouts preserve both local and server content without horizontal page overflow.
- Semantic outage keeps manual recall and degraded messaging reachable while ordinary editing continues.
- Long labels wrap or truncate with an accessible full name and never cover adjacent actions.
- On-screen keyboard resize does not hide the focused field or primary save/recovery status.

## 15. Concurrency

Responsive remounts and breakpoint changes must not reset draft revision, save
generation, pending operation IDs, or recall session correctness. Rotating or
resizing during save, upload, or recall retains the latest authoritative state
and does not dispatch duplicate mutations. A recall batch held during touch
interaction remains frozen until that interaction completes, as required by the
latest-only recall contract.

## 16. Accessibility

- All workflows pass keyboard navigation at desktop and mobile viewport widths.
- Controls expose accessible names, visible focus, semantic roles, and non-color state text.
- Layout supports 200% browser zoom/reflow without loss of content or function.
- Status messages use appropriate live regions without repeated disruptive announcements.
- Motion respects `prefers-reduced-motion`, and focus follows dialogs, previews, and return paths predictably.

## 17. Acceptance Criteria

#### ADDED: Core workflows remain usable across desktop and mobile

### Happy Path

```gherkin
Scenario: Complete a task workflow by touch
  Given a touch browser viewport of 360 by 800 CSS pixels
  When I create a task, associate it with a project, and change its status by menu
  Then no drag gesture is required
  And every primary action target measures at least 44 by 44 CSS pixels
  And the page has no horizontal overflow

Scenario: Use recall while writing on mobile
  Given I am editing a note at the mobile reference viewport
  When recall surfaces an earlier note and I open its preview
  Then I can inspect it in a sequential mobile surface
  And one touch action returns me to the unchanged draft and editor context
```

### Edge Case

```gherkin
Scenario: Recover a failed save at minimum width
  Given a viewport of 320 by 568 CSS pixels and an unsent edited draft
  When the save request fails
  Then the error, Retry, and Copy actions are visible and operable
  And no input or page action is clipped by horizontal overflow

Scenario: Resize during active work
  Given an upload, save, or recall request is in flight
  When the viewport crosses between mobile and desktop layouts
  Then the operation is not duplicated or falsely acknowledged
  And the current draft and latest valid response remain intact

Scenario: Navigate with keyboard at mobile width
  Given a 360 by 800 viewport without touch or pointer input
  When I traverse navigation, editor, recall, and recovery controls
  Then every action is reachable with visible focus in logical order
  And hidden responsive content receives no focus
```

## 18. Test Strategy

- Playwright projects measure `320x568`, `360x800` touch, and `1440x900` desktop layouts.
- Runtime assertions check page overflow, target bounding boxes, target separation, clipping, and focused-element visibility.
- Scenario tests cover creation, task status, editing, compare/return, recall/link, quantities, attachments, and recovery.
- Component tests cover breakpoint remounts during save/upload/recall and hidden-content focus exclusion.
- Accessibility checks combine automated rules with keyboard, 200% zoom/reflow, and screen-reader status review.

## 19. Implementation Notes

Use the existing Vue, Naive UI, XIcons, and CodeMirror stack. Responsive layout's
purpose is to present existing domain commands; callers are all workspace routes;
contracts are shared state, action parity, draft continuity, and measurable
reachability. Prefer CSS and composition over duplicate mobile components when
the interaction remains the same, but use sequential mobile surfaces where a
desktop split view is unusable. No external package is proposed.

## 20. Definition Of Done

- Core workflows pass at all three reference viewports and with touch and keyboard input.
- Primary touch targets and overflow meet the numeric acceptance thresholds.
- Mobile status, recall, compare, conflict, and recovery actions preserve drafts and return paths.
- Automated responsive regressions run through the stable test command.
- Accessibility, lint, test, and production build gates pass.
- A final `just preflight` passes after all story implementation and verification tasks.
