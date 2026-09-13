# e03s01 Draft-Safe Markdown Note Editing

## 1. Metadata

- **ID:** e03s01
- **Type:** feat
- **Context:** notes-domain
- **Status:** todo
- **BCP:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want to write autosaved Markdown without losing unsent or conflicting work so that interrupted and cross-device editing remains recoverable.

## 3. Problem

An online-first editor can falsely report saves, overwrite another device, or discard local writing unless draft, save, revision, and recovery transitions are explicit.

## 4. Outcome

CodeMirror edits canonical Markdown through a note adapter to the shared `DraftSaveCoordinator`, recovers local drafts when enabled, and presents the coordinator's deterministic save and explicit conflict outcomes.

## 5. In Scope

- Note navigation, create/open, CodeMirror 6 editing of canonical Markdown, application-integrated inline Markdown presentation, Vim integration with a discoverable disable path, and optional title.
- Integrate e01s03's domain-independent `DraftSaveCoordinator` through a note-specific content/save adapter.
- Map shared coordinator states and commands to visible Saved, Saving, Error, and explicit note conflict-resolution controls.
- Note autosave timing and note API integration for one in-flight save, coalesced pending edits, idempotent retry, and explicit conflict resolution.
- Device-local draft recovery that persists by default on personal devices and can be disabled and cleared on managed/shared devices.

## 6. Out Of Scope

- Owning, redefining, or forking the `DraftSaveCoordinator`, generic revision/idempotency contract, or generic archive/restore commands supplied by e01s03.
- Real-time co-editing, automatic merge, publication workflow, note attachments, semantic recall, and editor-state serialization as canonical content.

## 7. Domain Terms

- **Canonical Markdown:** Authored text stored independently of CodeMirror state.
- **Note save adapter:** Translation between note title/Markdown and e01s03's domain-independent coordinator commands and outcomes.
- **Local recovery:** Device-local note draft snapshot stored separately from acknowledged state and server-response caches.
- **Conflict controls:** Note UI actions that dispatch the coordinator's Reload server, Replace server, or Use manual merge commands.

## 8. Preconditions

- e01s02 provides authenticated sessions.
- e01s03 provides stable note identity, atomic server revisions, durable idempotency, the complete `DraftSaveCoordinator` contract, and generic note archive/restore commands.

## 9. Dependencies

- CodeMirror 6 and its Vim integration are accepted `[OK]` dependencies; Pinia is `[OK]` only if state must span components.

## 10. Data

Authoritative note data is stable ID, optional title, canonical Markdown, lifecycle, server revision, and timestamps. The note adapter supplies title/Markdown snapshots to the shared coordinator and consumes its revision, generation, operation, pending, and conflict state. Device-local recovery metadata remains note-specific and separate from coordinator state, acknowledged content, and server-response caches.

## 11. API

- Create accepts optional title and Markdown and acknowledges stable ID/revision.
- The note adapter submits e01s03 revision-checked save commands containing operation ID, expected server revision, title, and Markdown.
- The API returns e01s03's durable replay acknowledgement or typed revision conflict, with current note title/Markdown added as the note-specific conflict snapshot.
- Note archive and restore use e01s03's generic lifecycle commands; this story does not redefine them or add final purge.

## 12. UI

The knowledge workspace provides accessible note navigation and a responsive CodeMirror 6 editor. Inline Markdown presentation remains an application behavior over canonical authored Markdown, and Vim integration has a discoverable enable/disable control. The UI maps e01s03 coordinator output exactly: `clean + idle` to Saved, any `saving` to Saving, and any `error` to Error. Conflict shows retained local and server note versions with Reload server, Replace server, and Use manual merge actions. Recovery settings explain and control persistent device-local storage for managed/shared devices.

## 13. Security

Notes require the authenticated owner and CSRF protection for mutations. Device-local persistent recovery defaults to personal-device behavior, can be disabled and cleared, and explains that unsent drafts are not available cross-device.

## 14. Failure Modes

- Network failure is passed to the shared coordinator without changing note draft content; retry resolves its failed operation before sending coalesced newer content.
- Invalid or stale responses are rejected by the shared coordinator and cannot advance acknowledged note state.
- Navigation/crash recovery retains unsent text when enabled.
- Conflict never triggers automatic replacement, merge, or retry.

## 15. Concurrency

e01s03 exclusively defines edit/save ordering, generation handling, coalescing, retry, and conflict-command precedence. This story verifies the note adapter preserves those transitions exactly: each CodeMirror edit supplies one newer draft snapshot, autosave timers dispatch only coordinator-approved effects, and late responses cannot mutate the note outside coordinator output. Note integration tests cover typing during ordinary and Replace server saves, retained local/server Markdown, and one coalesced follow-up save.

## 16. Accessibility

Editor controls and conflict actions have programmatic names, keyboard operation, visible focus, and announced status changes that do not steal focus. Vim mode has a discoverable disable path.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Notes integrate shared draft-safe coordination

The note adapter follows e01s03's shared state transitions and never silently discards either note version.

```gherkin
Scenario: Edit Markdown with inline presentation and Vim integration
  Given the owner opens a note in the CodeMirror 6 editor
  When they edit canonical Markdown using inline presentation and enable Vim mode
  Then the saved content remains authored Markdown independent of editor state
  And Vim mode has a discoverable disable control

Scenario: Edit arrives while a save is in flight
  Given note revision 4 is saving draft revision 7
  When the owner edits the note to draft revision 8
  And revision 7 is acknowledged as server revision 5
  Then the editor remains modified
  And one coalesced save sends draft revision 8 against server revision 5
```

### Edge Case

```gherkin
Scenario: Another device has changed the note
  Given local Markdown expects server revision 4
  When the server reports current revision 5
  Then the editor enters conflict plus idle
  And both local and server Markdown remain available
  And autosave is suspended until an explicit conflict command

Scenario: A late save response follows manual merge
  Given the owner selected Use manual merge and edited the merged text
  When a response from an older save generation arrives
  Then that response does not change content save status or acknowledged revision

Scenario: Persistent recovery is disabled on a managed device
  Given the owner disables and clears persistent local recovery
  When they continue editing a note
  Then the editor does not persist the draft across a browser restart
  And the UI explains that unsent content is not recoverable on that device
```

## 18. Test Strategy

Reuse e01s03's exhaustive coordinator contract tests rather than duplicating them. Add note-adapter contract tests for title/Markdown snapshots and conflict content, API integration tests proving shared revision/idempotency behavior, fake-timer tests for autosave effects, component tests for status/conflict controls, and browser tests for reload, navigation, local recovery enabled/disabled, and keyboard operation.

## 19. Implementation Notes

Keep authored Markdown independent of CodeMirror view state. Implement a thin note adapter around e01s03's `DraftSaveCoordinator`; do not create note-owned save states, generations, retry precedence, or conflict transitions. Note components render coordinator outputs and dispatch coordinator commands. Generic note archive/restore comes from e01s03, while final purge remains in e07.

## 20. Definition Of Done

- Note editing passes the complete shared coordinator contract without redefining it.
- Failures, navigation, and conflicts retain recoverable writing.
- Successful status follows server acknowledgement only.
- `just test`, then `just lint`, then `just build` pass in that order.
- After those gates, `just preflight` reports no new security findings in the affected note editor, adapter, and recovery paths.
