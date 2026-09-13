# e01s03 Shared Record Lifecycle And Draft Coordination

## 1. Metadata

- **ID:** e01s03
- **Type:** feat
- **Context:** domain
- **Status:** todo
- **BCP:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want every record kind to share revision-checked saving and reversible lifecycle commands so that edits cannot overwrite newer work and ordinary removal does not destroy data.

## 3. Problem

Domain work needs one durable identity, save-coordination, and reversible lifecycle contract before separate record implementations can safely persist data.

## 4. Outcome

Every domain record has immutable identity and monotonic revision metadata. A domain-independent `DraftSaveCoordinator` owns deterministic revision-checked save state, while generic active/archive/restore lifecycle commands support notes, resources, tasks, and projects without owning any domain-specific editor or cascade policy.

## 5. In Scope

- Shared identity, revision, operation-ID, and lifecycle value types for project, task, note, and resource records.
- The singular domain-independent `DraftSaveCoordinator` state machine for one in-flight revision-checked save, coalesced pending work, idempotent retry, stale-response rejection, and explicit conflict commands.
- PostgreSQL migrations and atomic expected-revision/idempotency semantics.
- Generic single-record `active -> archived -> active` archive/restore commands usable by notes, resources, tasks, and projects, with active-by-default filtering and preserved identity/relationships.
- Consistent application/API metadata, acknowledgement, and typed conflict contracts.

## 6. Out Of Scope

- Full domain fields, relationships, domain-specific editors, archive/restore UI, project cascade policy, attachments, and recall indexes.
- Final purge, retention, attachment-byte cleanup, and post-purge recovery; e07 remains the sole owner of final purge.

## 7. Domain Terms

- **Stable ID:** Immutable globally unique identity independent of names and edits.
- **Server revision:** Monotonic version of acknowledged authoritative record state.
- **Expected revision:** Revision supplied by a mutation to prevent stale overwrite.
- **DraftSaveCoordinator:** Domain-independent state machine that coordinates draft revisions, save generations, operation IDs, acknowledgements, errors, pending work, and conflicts.
- **Archive:** Revision-checked reversible transition from `active` to `archived`.
- **Restore:** Revision-checked reversible transition from `archived` to `active`.

## 8. Preconditions

- e01s01 supplies the server and database migration/test interface.
- e01s02 supplies the authenticated owner context for exposed record operations.

## 9. Dependencies

- PostgreSQL 18 and SQLx are accepted `[OK]` dependencies.

## 10. Data

The base persistence contract includes stable ID, record kind, owner identity, server revision, created timestamp, updated timestamp, and `active` or `archived` lifecycle state. Revision starts at one and increases only on an acknowledged content or lifecycle mutation. Idempotency records bind an operation ID to its command fingerprint and original acknowledgement.

The coordinator keeps draft content state (`clean`, `modified`, or `conflict`) separate from save progress (`idle`, `saving`, or `error`). It tracks draft revision, save generation, operation ID, expected/acknowledged server revision, one in-flight operation, one coalesced pending save, and both local and server conflict snapshots without depending on any record's content shape.

## 11. API

Record representations expose stable ID, kind, lifecycle, revision, and timestamps. Revision-checked save, archive, and restore commands carry record kind, stable ID, operation ID, and expected revision. Replaying the same operation durably returns its original acknowledgement; reusing an operation ID for a different command is rejected. Revision mismatch returns a typed conflict with current metadata and no partial write.

## 12. UI

No generic record editor or archive screen is introduced. Domain-specific clients integrate the shared coordinator and commands while retaining their own content, labels, controls, and accessibility behavior.

## 13. Security

All record access is scoped to the authenticated workspace owner and mutation boundaries require CSRF protection. Errors and idempotency lookups do not reveal whether another owner's record exists or expose record content in logs.

## 14. Failure Modes

- Invalid kinds, revisions, or IDs fail boundary validation.
- Migration failure leaves the prior schema intact.
- A stale expected revision changes no authoritative fields or lifecycle state.
- Save failure retains local draft state; a conflict retains both local and server snapshots until an explicit reload, replace, or manual-merge command.
- Archive of an archived record and restore of an active record return typed no-op outcomes; final purge is unavailable here.
- Reusing an operation ID with a different command fingerprint is rejected without replaying or applying either new intent.

## 15. Concurrency

Expected-revision comparison, idempotency recording, and mutation occur atomically. Two saves or lifecycle commands from the same revision yield one acknowledged winner and one conflict; retries of the winner return the same acknowledgement. Save acknowledgements never move backward, only the current generation changes save progress, and an acknowledgement marks a draft clean only when its captured draft revision is still current. Edits during an in-flight save coalesce into one pending save against the newly acknowledged revision.

Archive and restore race safely with saves and with each other. The committed revision determines the winner, no rejected command partially mutates content or lifecycle, and parameterized race tests cover note, resource, task, and project records. Generic project archive changes only that project; any project-plus-task policy composes these commands in its domain story.

The coordinator serializes events and applies the following normative precedence table. A "current" response matches the active generation and operation ID; every stale response is ignored before any other rule. When events are queued together, a current conflict is reduced first, then one explicit conflict-resolution command, then edits, then timer-driven autosave or retry. The first accepted resolution command increments the generation synchronously and disables the other conflict commands, so competing or late commands are no-ops.

| Rule | Starting state or condition | Event or competing event | Required precedence and next state | Pending-save treatment |
|---|---|---|---|---|
| C01 | Any state | Response for a stale generation or operation ID | Ignore it; it cannot change content, acknowledged revision, save progress, or conflict state. | Preserve the current pending save unchanged. |
| C02 | `modified + saving` | Edit | Accept the edit before any queued autosave timer, increment draft revision, remain `modified + saving`. | Replace any older pending save with one snapshot of the newest local draft. |
| C03 | `modified + saving` | Autosave timer | Do not start a competing save while one is in flight. | Keep exactly one coalesced newest pending snapshot. |
| C04 | `modified + error` | Edit competing with Retry | Accept the edit first and retain the failed operation for idempotent retry; Retry resends that failed operation before newer content. | Coalesce the edit as the one newest pending save. |
| C05 | `modified + error` | Retry | Reuse the failed operation ID and fingerprint; on acknowledgement, dispatch the newest pending snapshot against the acknowledged revision. | Never replace the failed payload with pending content; clear pending only when its own save starts. |
| C06 | Any non-conflict state | Current-generation revision conflict | Conflict takes precedence over queued autosave/retry, enters `conflict + idle`, and preserves current local plus returned server snapshots. | Fold the newest pending snapshot into the retained local conflict snapshot; dispatch nothing. |
| C07 | `conflict + idle` | Edit | Update the retained local snapshot and draft revision, remain `conflict + idle`, and do not mutate the server snapshot. | Treat the newest local snapshot as pending intent, but do not dispatch it. |
| C08 | `conflict + idle` | Autosave timer or Retry | Suppress the event; conflict cannot be resolved implicitly. | Preserve the newest local snapshot without dispatch. |
| C09 | `conflict + idle` | Reload server | If it is the first accepted resolution command, invalidate older generations, adopt the conflict's server snapshot as the clean acknowledged draft, and enter `clean + idle`. | Explicitly discard local pending intent only as the stated effect of Reload; retain no pending save. |
| C10 | `conflict + idle` | Replace server | If it is the first accepted resolution command, invalidate older generations and send the newest retained local snapshot against the conflict's current server revision with a new operation ID; enter `modified + saving`. | Move the retained local snapshot into the replacement in-flight save; later edits form one new pending save. |
| C11 | `conflict + idle` | Use manual merge | If it is the first accepted resolution command, invalidate older generations, install only the user's explicit merged snapshot as a new draft revision, and enter `modified + idle`; no save starts until the normal autosave rule or explicit save event. | Clear the old pending slot; the merged draft is the current modified content for the next save. |
| C12 | `conflict + idle` or resolution in progress | Reload, Replace, and Use manual merge compete | Process only the first command in serialized coordinator order; disable controls synchronously and ignore every losing command or late event from the invalidated generation. | Apply only the winning row's pending-save rule. |

Exhaustive coordinator tests and all adapter contract tests must cite the applicable C01-C12 row of this table. They must cover every row, each pair of competing conflict commands, edits during Replace, current and stale acknowledgements after each resolution command, and pending-save preservation or disposal exactly as stated.

## 16. Accessibility

This foundation has no direct interaction. Typed save and lifecycle outcomes must support later programmatic labels, announced status, explicit confirmation, and non-color-only conflict/archive states.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Shared revision-checked draft and lifecycle contract

Persisted domain records retain identity across edits, archive, and restore. All record kinds use the same coordinator and revision/idempotency rules.

```gherkin
Scenario: An edit arrives while a save is in flight
  Given a record at server revision 4 is saving draft revision 7
  When its adapter supplies draft revision 8 before revision 7 is acknowledged
  Then the DraftSaveCoordinator keeps the draft modified
  And one coalesced save sends draft revision 8 against acknowledged server revision 5

Scenario Outline: Archive and restore each domain record kind
  Given an active <kind> record has stable ID A and server revision 1
  When archive and restore are acknowledged with current expected revisions
  Then the <kind> record returns to active with stable ID A
  And no relationship or domain content is removed

  Examples:
    | kind     |
    | note     |
    | resource |
    | task     |
    | project  |
```

### Edge Case

```gherkin
Scenario: Concurrent commands use the same revision
  Given save and archive commands both expect server revision 1
  When both target the same record concurrently
  Then exactly one command is durably acknowledged
  And the other returns a typed revision conflict
  And no fields from the rejected command are persisted

Scenario: A lost acknowledgement is retried
  Given an archive operation was durably applied but its acknowledgement was lost
  When the same operation ID and command fingerprint are retried
  Then the original acknowledgement is returned
  And the record revision advances only once
```

## 18. Test Strategy

Use exhaustive table-driven tests keyed to every row in Section 15 for every `DraftSaveCoordinator` transition and command precedence, including edit-during-save, failure, retry, stale generation, reload, replace, manual merge, pending saves, and all pairs of competing conflict commands. Adapter contract tests must reference the same rows rather than redefine precedence. Use SQLx integration tests for atomic expected revisions and durable idempotency, parameterized archive/restore tests for all four record kinds, API shape tests, and real-database races among save, archive, restore, and replay.

## 19. Implementation Notes

`e01s03` is the singular owner of the domain-independent `DraftSaveCoordinator` contract and generic active/archive/restore lifecycle commands. Later domain stories supply adapters and domain payloads rather than redefining this state machine. Keep domain types independent of Vue, CodeMirror, SQLx rows, and Axum payloads. Generic lifecycle commands transition one record; domain-specific multi-record policy may compose them transactionally. Preserve archived records and relationships. Final purge remains exclusively in e07.

## 20. Definition Of Done

- Migration applies and rolls back safely in test infrastructure.
- The shared coordinator's complete Section 15 transition and precedence table, including every competing-command pair, idempotent replay, pending-save rule, and stale-generation behavior, is tested independently of record kind and UI framework.
- Note, resource, task, and project archive/restore preserve stable IDs and relationships; save/lifecycle races have one atomic winner.
- Final purge is absent from this story and remains owned by e07.
- `just test`, then `just lint`, then `just build` pass in that order.
- After those gates, `just preflight` reports no new security findings in the affected shared record, coordinator, and lifecycle paths.
