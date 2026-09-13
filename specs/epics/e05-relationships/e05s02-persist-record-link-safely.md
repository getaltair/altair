# e05s02: Convert Notes And Persist New Record Links Safely

## 1. Metadata

- **Story ID:** e05s02
- **Epic:** e05 Cross-Domain Relationships
- **Type:** feat
- **Context:** relationships-domain
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want to convert a note into a task or resource while retaining and linking the source, and I want any unsaved new record and requested link saved together, so explicit creation never loses source material or reports a partly persisted connection.

## 3. Problem

A note-to-task or note-to-resource conversion requires a new domain record plus a link while retaining the source note. More generally, a link action from any unsaved new-record draft requires multiple writes. Independent writes can delete or reinterpret the source, orphan a new record, omit the requested link, duplicate work after a lost acknowledgement, or overwrite concurrent changes.

## 4. Outcome

One authenticated create-and-link command atomically creates the new record and its requested relationship, replays a prior acknowledgement for the same operation, and reports conflicts or failures without discarding the draft. Note-to-task and note-to-resource conversions use this command, retain the unchanged source note, prefill the new record from it, and link both records exactly once.

## 5. In Scope

- Explicitly convert an existing note into a new task or resource by prefilling a new draft, retaining the source note, and linking source and result.
- Create one project, task, note, or resource from any validated unsaved new-record draft and link it to one existing active record.
- Commit record creation and canonical relationship creation in one transaction.
- Require a stable operation ID and provide durable idempotent replay.
- Revalidate the target identity, revision, archive state, and relationship state.
- Preserve the client draft and, for conversion, the source note for retry, conflict resolution, or copying after failure.
- Route editor save state, stale-write conflicts, retry, and draft disposition through the shared e01s03 `DraftSaveCoordinator` rather than a relationship-specific save coordinator.

## 6. Out Of Scope

- Batch creation or linking to multiple targets.
- Automatic suggestion acceptance or autonomous linking.
- Merging duplicate records or changing resource quantity.
- Last-write-wins conflict resolution.
- Persisting unsaved recall input before an explicit action.

## 7. Domain Terms

- **Create-and-link command:** Explicit mutation that creates one record and one requested link.
- **Note conversion:** Explicit creation of a task or resource prefilled from a note while the source note remains authoritative and linked to the result.
- **Operation ID:** Stable client-generated identifier for durable replay of one logical mutation.
- **Expected target revision:** Revision shown when the user selected the target.
- **Lost acknowledgement:** A committed command whose response did not reach the client.
- **Atomicity:** Both record and link commit, or neither commits.

## 8. Preconditions

- The user explicitly requests creation and linking.
- A note conversion begins from a saved active source note; conversion never changes or removes that source.
- Draft data satisfies the selected domain's minimum-input rules.
- The target snapshot supplies stable identity and source revision.
- An authenticated session and valid CSRF token are present.

## 9. Dependencies

- e05s01 canonical related-link invariants and storage.
- Domain create validation from e02, e03, and e04.
- e01 transaction, session, typed error, and operation-replay facilities.
- e01s03 shared `DraftSaveCoordinator` for editor save acknowledgement, conflict state, retry, and recoverable draft ownership.

## 10. Data

- Persist the new record, canonical relationship, operation ID, request fingerprint, and acknowledgement in one transaction; the existing source note is retained unchanged.
- The fingerprint binds operation ID to actor, record kind, normalized command payload, target identity, and expected target revision.
- Reusing an operation ID with the same fingerprint replays the stored acknowledgement.
- Reusing an operation ID with a different fingerprint returns a typed idempotency conflict.
- The authoritative record and relationship remain independent of recall snapshots.

## 11. API

- A typed create-and-link command accepts record kind, domain payload, target identity, expected target revision, relationship kind, and operation ID for both general unsaved new-record linking and note conversion.
- The response returns committed record identity/revision and canonical relationship identity.
- Validation, stale revision, archived target, idempotency conflict, and transient persistence failure are distinct typed outcomes.
- A response never reports link success before transaction commit.

## 12. UI

- The explicit Link action enters a pending state while retaining all entered content.
- A note exposes explicit Create task from this and Create resource from this actions that prefill the target draft, identify the retained source, and require user submission.
- Success transitions only from the server acknowledgement and exposes the committed record and link.
- Failure leaves the draft and selected target visible with retry and copy options.
- Conflict freezes the attempted payload and target snapshot for explicit review; it does not silently retarget.
- Save acknowledgement, save failure, stale-write conflict, retry, and draft disposition use the shared e01s03 `DraftSaveCoordinator`; this story adds create-and-link command state, not another editor save state machine.

## 13. Security

- Require session authentication and CSRF protection.
- Authorize both creation and target access without leaking unauthorized target existence.
- Validate all boundary payloads and reject operation-ID payload substitution.
- Keep personal draft content out of logs and error details.

## 14. Failure Modes

- Record validation fails: write nothing and retain the draft.
- Note conversion fails: retain the unchanged source note and the complete prefilled task or resource draft; report no conversion link.
- Target is missing, archived, revised, or already linked incompatibly: write nothing and return a typed outcome.
- Relationship insert fails after record insert: roll back the record.
- Connection drops after commit: same-operation retry replays the original acknowledgement.
- Connection drops before commit: retry performs one transaction and creates one record/link pair.

## 15. Concurrency

- Lock and revalidate the target and relationship state inside the transaction.
- Concurrent same-operation requests converge on one stored acknowledgement.
- Different operation IDs creating equivalent records remain separate user-requested records; no automatic merge occurs.
- Concurrent target archive or relationship creation yields a serializable committed result or a typed retry/conflict, never partial success.
- The e01s03 `DraftSaveCoordinator` prevents an old mutation response from replacing newer editor draft/save/conflict state.

## 16. Accessibility

- Pending state is programmatically exposed and does not remove keyboard focus.
- Error and conflict messages identify the recoverable action in text.
- Retry, copy, cancel, and inspect-target controls are keyboard and touch usable.
- No timeout silently discards entered content or forces rapid action.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Atomic idempotent create-and-link

An explicit create-and-link action durably commits both writes once or commits neither, while preserving recoverable draft state for every non-success outcome. This owns both general unsaved new-record-and-link behavior and note-to-task or note-to-resource conversion that retains and links the source note.

```gherkin
Scenario Outline: Happy path atomically creates a record and requested link
  Given <source>
  When the user submits create-and-link with a new operation ID
  Then one <record> and one canonical related link commit in one transaction
  And the server acknowledges both stable identities
  And any source note remains unchanged and linked to the new record
  And the UI reports success only after that acknowledgement
  Examples:
    | source | record |
    | a valid general unsaved project draft and an active related note at revision 4 | project |
    | Create task from this on an active source note with a valid prefilled task draft | task |
    | Create resource from this on an active source note with a valid prefilled resource draft | resource |
```

### Edge Case

```gherkin
Scenario Outline: Edge failures never produce partial or duplicate success
  Given a create-and-link command with <condition>
  When the command is submitted or retried
  Then <outcome>
  And the draft remains recoverable
  Examples:
    | condition                                      | outcome |
    | a relationship insert failure                  | neither record nor link is committed |
    | a target archived before transaction validation | neither record nor link is committed and a typed conflict is returned |
    | a lost acknowledgement after commit            | the same acknowledgement is replayed and exactly one record and link exist |
    | the same operation ID with a changed payload   | an idempotency conflict is returned and prior data is unchanged |
    | concurrent identical same-operation requests   | exactly one record, one link, and one acknowledgement exist |
    | note-to-task conversion is retried after a lost acknowledgement | the source note remains unchanged and exactly one task and one link exist |
    | note-to-resource relationship insertion fails | the source note and draft remain intact and no resource or link is committed |
```

## 18. Test Strategy

- Unit-test request fingerprinting, replay decisions, and typed conflict mapping.
- Integration-test rollback at every write boundary, source-note retention, and durable replay after simulated lost acknowledgement.
- Race same-operation submissions, target archive, and equivalent relationship creation under PostgreSQL.
- Component-test create-and-link pending state around the e01s03 `DraftSaveCoordinator` save, success, retry, conflict, cancellation, and draft retention transitions.
- Browser-test general unsaved create-and-link plus explicit note-to-task and note-to-resource conversion with injected network failures and server acknowledgement timing.

## 19. Implementation Notes

- Purpose: the create-and-link application command owns general unsaved new-record linking and explicit note-to-task or note-to-resource conversion as one durable user action.
- Callers: note conversion actions, recall suggestion actions, and project, task, note, and resource creation surfaces.
- Contracts: explicit intent, retained source note, one transaction, durable idempotency, target revalidation, typed conflict, server-acknowledged success, draft preservation, and e01s03 `DraftSaveCoordinator` ownership of editor save/conflict state.
- Keep domain validation outside Axum/SQLx types; perform orchestration in one application command and transaction adapter.
- Reason for Depth: durable operation replay is required to distinguish a lost acknowledgement from a second user action without duplicating personal records.
- No new service, broker, framework, or external package is proposed; slopcheck is not applicable.

## 20. Definition Of Done

- Every transaction boundary and acceptance scenario has automated coverage.
- Fault injection proves no partial record/link state and durable same-operation replay.
- Conflict and network-failure UI retains the full draft, selected target, and unchanged source note for conversions.
- Authentication, CSRF, payload-safe logging, and accessibility tests pass.
- Editor save, retry, and conflict behavior is integrated with the shared e01s03 `DraftSaveCoordinator` without a duplicate state machine.
- `just test`, `just lint`, `just build`, and `just preflight` pass.
- Preflight reports no new security findings in affected relationship and editor-integration paths.
- No task is marked passing until its verify command exits successfully.
