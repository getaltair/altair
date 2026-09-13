# e03s02 Quick Capture And Markdown Import

## 1. Metadata

- **ID:** e03s02
- **Type:** feat
- **Context:** notes-domain
- **Status:** todo
- **BCP:** 3
- **Risk:** P1

## 2. User Story

As the workspace owner, I want to capture a fragment or import a Markdown file without classifying it so that useful knowledge enters the workspace with minimal friction.

## 3. Problem

Mandatory titles, folders, tags, projects, or metadata mapping would block fast capture and basic seeding.

## 4. Outcome

Global quick capture creates an ordinary note from text or a URL, and Markdown import creates a note whose canonical content preserves the selected file text.

## 5. In Scope

- Global text and URL quick capture with no required title or classification.
- Device-local recovery of unsaved quick-capture drafts after accidental navigation, using the e01s03 shared `DraftSaveCoordinator`, with disable-and-clear controls for managed/shared devices.
- Single-file Markdown import without mandatory metadata mapping.
- A deterministic display label derived from title, first nonblank line, filename, or Untitled fallback without changing canonical content.

## 6. Out Of Scope

- Inbox workflow, folders/tags, bulk migration, front-matter interpretation, deduplication, OCR, and web ingestion.
- Attachment-based quick capture and attachment upload integration, which are deferred to e07s01 and are not completed by this story.
- Note-to-task or note-to-resource conversion, which is an explicit create-and-link workflow owned by e05s02 after the e02 task and e04 resource creation contracts exist; silent conversion remains prohibited.

## 7. Domain Terms

- **Quick capture:** Creation path for an ordinary note without classification.
- **Quick-capture draft recovery:** Device-local snapshot of unsaved capture text, kept separate from acknowledged note state and coordinated through e01s03's shared `DraftSaveCoordinator`.
- **Display label:** UI fallback, not authored content or an AI-generated title.
- **Markdown import:** Basic creation of a note from selected Markdown file text.

## 8. Preconditions

- e01s03 provides the shared `DraftSaveCoordinator` used for quick-capture draft and save transitions.
- e03s01 provides authenticated note creation, canonical Markdown, and save acknowledgement.
- e05s02 depends on this story's ordinary-note identity when it later implements explicit note-to-task/resource create-and-link conversion.

## 9. Dependencies

- Browser file selection and existing note APIs are sufficient; no additional external package is planned.

## 10. Data

Capture stores entered text exactly as canonical Markdown. Device-local quick-capture recovery stores unsaved text separately from acknowledged note state and server-response caches, and clearing recovery removes that local snapshot. Import decodes accepted UTF-8 Markdown text, preserves line endings/content according to a documented deterministic normalization policy, and may retain filename as provenance/display metadata without injecting it into Markdown.

## 11. API

Both paths use ordinary note creation. Requests require nonblank text after input validation; acknowledgement returns stable note identity and revision. Unsupported or invalid file encoding returns a typed validation error without creating a partial note.

## 12. UI

Quick capture is globally reachable and requires only one text field plus Save. It restores an unsaved device-local draft after accidental navigation when recovery is enabled and offers explicit disable-and-clear controls for managed/shared devices. Import offers a file chooser, reviewable content, Save, Cancel, and clear success/error state.

## 13. Security

Creation requires an authenticated session and CSRF proof. Persistent device-local quick-capture recovery can be disabled and cleared on managed/shared devices and explains that unsent content is not available cross-device. File handling is local until explicit save, enforces an implementation-defined safe size bound, treats Markdown as untrusted text, and never executes imported HTML or scripts.

## 14. Failure Modes

- Empty input remains editable and is not saved as a meaningless record.
- Invalid/oversized import remains unsaved with a clear error.
- Network failure retains capture/import text for retry or copy-out and never reports success.
- Accidental navigation retains and restores the unsaved quick-capture draft when device-local recovery is enabled.
- Disabling and clearing recovery removes the stored quick-capture draft and prevents later restoration on that managed/shared device.

## 15. Concurrency

The quick-capture adapter uses e01s03's shared `DraftSaveCoordinator` for draft generations, one in-flight save, coalesced edits, acknowledgement, retry, and stale-response rejection. Each explicit save uses a unique operation ID so retry cannot create duplicate notes after a lost acknowledgement. Device-local recovery remains separate from coordinator state and acknowledged content.

## 16. Accessibility

Capture and import are keyboard and touch usable, file controls have labels, errors identify the affected field, and save status is announced without stealing focus.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Classification-free capture and Markdown import

Both paths create ordinary notes only after server acknowledgement and preserve authored text.

```gherkin
Scenario: Save an unclassified fragment
  Given the owner opens quick capture
  When they enter "check ESP32 clock power notes" without a title tag or project
  And the server acknowledges Save
  Then one ordinary note is created with that canonical Markdown
  And no classification is required

Scenario: Save a URL through quick capture
  Given the owner opens quick capture
  When they paste a URL and the server acknowledges Save
  Then one ordinary note is created with the URL as canonical Markdown
  And no website ingestion or attachment is claimed

Scenario: Recover a quick-capture draft after accidental navigation
  Given device-local quick-capture recovery is enabled
  And the owner has entered an unsaved fragment
  When accidental navigation closes quick capture and the owner returns
  Then the unsaved fragment is restored on that device
  And the e01s03 shared DraftSaveCoordinator retains the current draft generation

Scenario: Import a Markdown file
  Given the owner selects a valid UTF-8 Markdown file
  When they review its content and the server acknowledges Save
  Then one ordinary note is created with the normalized canonical Markdown
  And no metadata mapping is required
```

### Edge Case

```gherkin
Scenario: Import invalid Markdown file input
  Given the owner selects an unsupported or oversized file
  When import validation runs
  Then no note is created
  And the owner sees a clear error and can choose another file

Scenario: Save acknowledgement is lost
  Given a capture operation was durably applied
  When the same operation ID is retried
  Then the original note acknowledgement is returned
  And no duplicate note is created

Scenario: Disable and clear recovery on a managed or shared device
  Given quick capture has an unsaved device-local recovery snapshot
  When the owner disables and clears persistent recovery
  Then the stored snapshot is removed from that device
  And returning to quick capture does not restore the cleared text
  And the UI explains that unsent content will not be recoverable there
```

## 18. Test Strategy

Use domain tests for label fallback and text preservation, adapter tests against e01s03's shared `DraftSaveCoordinator`, API tests for idempotent creation and validation, component tests for no-required-metadata and disable-and-clear behavior, and browser tests for capture/import success, network failure, accidental navigation recovery, and managed/shared-device recovery disabled.

## 19. Implementation Notes

Reuse ordinary note creation and integrate its quick-capture adapter with e01s03's shared `DraftSaveCoordinator`; do not fork coordinator states or create an inbox record type. Keep device-local recovery metadata separate from coordinator state, acknowledged content, and server-response caches. Define accepted extension, MIME/encoding checks, size bound, and newline normalization in implementation tests before parsing files. Do not add attachment capture here: e07s01 owns attachment upload integration. Do not implement note-to-task/resource conversion here: e05s02 owns the explicit atomic create-and-link action while retaining the source note.

## 20. Definition Of Done

- Text/URL capture and one-file Markdown import preserve canonical authored content.
- Accidental navigation restores an unsaved device-local quick-capture draft when enabled through the e01s03 shared coordinator integration.
- Managed/shared-device controls disable persistent recovery and clear stored quick-capture drafts.
- Neither path requires classification or a title.
- Invalid input and network failure create no false success or duplicate.
- `just test`, then `just lint`, then `just build` pass in that order.
- After those gates, `just preflight` reports no new security findings in the affected capture, import, and local recovery paths.
