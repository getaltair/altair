# e04s03: Duplicate suggestion choices preserve draft

## 1. Metadata

- **ID:** e04s03
- **Type:** feat
- **Context:** domain
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0
- **Epic:** e04 Resource Records

## 2. User Story

As the workspace owner, I want possible duplicate resources to offer explicit
choices while preserving my new draft so that I can reuse, update, adjust, or
keep records separate without automatic merging or losing entered information.

## 3. Problem

Similarity cannot prove identity or interchangeability. Opening an existing
record too early can discard an identifier, location, detail, or description;
automatic quantity changes can also corrupt unknown or deliberately separate stock.

## 4. Outcome

A possible duplicate is a non-blocking suggestion retrieved from real persisted
resources through lexical and identifier matching. The user may inspect it,
continue the draft, create separately, use or update the existing record, adjust
quantity explicitly, or return to the intact draft. No choice loses draft content.

## 5. In Scope

- Retrieve possible duplicate candidates from persisted active resources using
  real lexical and identifier matching during creation without blocking save.
- Preview/open an existing resource with a reliable return to the unchanged draft.
- Explicitly choose separate creation, existing-record use, detail update, quantity set,
  or quantity increment where e04s02 permits it.
- Preserve all draft text, identifiers, details, and local UI context through the
  e01s03 `DraftSaveCoordinator` until explicit acknowledged draft disposition.
- Revalidate candidate identity, revision, lifecycle, and action prerequisites on command.

## 6. Out Of Scope

- Automatic merge, automatic linking, automatic count changes, and identity inference.
- Assuming similar variants, locations, or serial numbers are interchangeable.
- Permanent duplicate suppression, cleanup queues, and bulk deduplication.
- Semantic duplicate-candidate enhancement; e06 may later consume or enhance this
  story's contract without becoming an e04s03 dependency.
- Cross-domain recall refresh and interaction coordination.
- Deterministic fake or fabricated candidates outside isolated tests.

## 7. Domain Terms

- **Possible duplicate:** A saved resource candidate whose similarity is advisory only.
- **Draft:** Unsaved resource input and selected references owned by the creation surface.
- **Use existing:** Continue with the saved record without mutating it automatically.
- **Keep separate:** Save the draft as a distinct stable resource.
- **Draft disposition:** Explicit decision to retain or discard the intact draft
  after an acknowledged alternative action.

## 8. Preconditions

- e04s01 supplies resource drafts, create/edit commands, stable IDs, and revisions.
- e04s02 supplies explicit quantity set/increment semantics.
- Persisted active resources with searchable authored text and identifiers exist.
- e01s03 supplies `DraftSaveCoordinator` and generic save/conflict decisions.
- Draft recovery is available under the configured personal/managed-device policy.

## 9. Dependencies

- Depends on e04s01, e04s02, and e01s03; it has no dependency on e06.
- Exposes resource-owned persisted lexical/identifier candidate retrieval that e06
  may later consume or enhance without replacing its baseline behavior.
- No new external package is required.

## 10. Data

- Draft state remains separate from saved resource state and duplicate-query caches.
- Candidate snapshot includes stable resource ID, source revision, archived state,
  identifying fields, retrieval source/evidence, and details sufficient for comparison.
- A selected action carries candidate ID and candidate revision so refreshes cannot
  retarget the command; e04s03 does not own an e06 interaction coordinator.
- Draft recovery includes text fields, identifiers, details, cursor/scroll or
  navigation context where applicable, and its own draft revision.
- Alternative mutation commands use unique operation IDs and expected saved-record revisions.

## 11. API

- Provide read-only candidate retrieval over real persisted active resources using
  lexical and identifier queries; it cannot encode mutation side effects.
- Existing-resource detail, update, quantity set, and quantity increment use their normal APIs.
- Every action revalidates stable ID, active lifecycle, current revision, and
  quantity preconditions rather than trusting the candidate snapshot.
- Separate creation uses the ordinary resource create API and is never rejected solely for similarity.
- Responses distinguish acknowledged action, conflict, archived candidate, invalid quantity,
  and unavailable candidate without changing draft disposition.
- e06 may later consume the candidate contract or add semantic evidence, deduped by
  stable resource ID, without being required for this story.

## 12. UI

- Label candidates as possible duplicates and show distinguishing identifiers, location,
  quantity state, and useful details without claiming equivalence.
- Provide Preview, Use existing, Update details, Set quantity, Add quantity, and Keep separate
  only when each action is applicable; none is preselected as the correct resolution.
- Bind each selected action to the visible candidate ID and revision; a refreshed
  list cannot change the target already submitted for server revalidation.
- Returning from preview or any failed/conflicting action restores the same draft and position.
- After an acknowledged alternative action, ask explicitly whether to discard or retain the draft;
  keeping it permits copying remaining details or creating a distinct record.
- Show only candidates backed by persisted records; label lexical, identifier, and
  later semantic evidence without implying identity.

## 13. Security

- Require authenticated sessions for candidate details and every mutation.
- Apply CSRF protection to update, quantity, and create commands.
- Revalidate resource ownership server-side; never trust candidate payload fields.
- Keep unsaved draft text and candidate excerpts out of logs and shared caches.
- Duplicate-query responses and draft-bearing navigation state use `Cache-Control: no-store`.

## 14. Failure Modes

- Candidate disappears or becomes archived: explain unavailability and retain the full draft.
- Candidate revision changes: show current saved record and preserve intended updates for resolution.
- Quantity increment is selected for unknown quantity: reject it and offer explicit Set total.
- Network, duplicate-query, or mutation failure never blocks separate creation or destroys draft content.
- Lexical/identifier retrieval failure is reported as unavailable or degraded and
  never replaced with fabricated candidates.
- A refreshed candidate list cannot retarget an action already bound to an ID and revision.
- Explicit discard failure leaves the recoverable draft available rather than assuming deletion.

## 15. Concurrency

- Submitted actions carry candidate ID and source revision and are revalidated server-side.
- Duplicate-query refresh and draft saving remain independent; query failure cannot
  alter `DraftSaveCoordinator` state or draft disposition.
- e04s03 defines no cross-domain refresh, cancellation, or publication state machine.
- Existing-record mutations use expected revisions and idempotent operation IDs.
- Separate create and existing-record mutation cannot share one success flag or draft lifecycle transition.

## 16. Accessibility

- Candidate labels explicitly say “possible duplicate” and do not rely on visual similarity cues.
- Preview and every choice are keyboard and touch operable with stable focus during refreshes.
- Refreshes preserve focus, and submitted actions remain bound to their named candidate.
- Return restores focus to the originating draft control; discard confirmation names what is retained or removed.
- Compact Android presentation provides equivalent preview and choices without hover dependence.

## 17. Acceptance Criteria

#### ADDED: Duplicate choices preserve drafts and require explicit action

### Happy Path

```gherkin
Scenario: Inspect an existing resource and keep a distinct device
  Given my unsaved draft includes a serial number, description, location, and details
  And lexical or identifier retrieval finds a similar persisted resource
  When I preview the saved resource and return
  Then every draft field and editing position is unchanged
  When I choose "Keep separate"
  Then a distinct resource is acknowledged with its own stable identity
  And the suggested resource remains unchanged
```

```gherkin
Scenario: Explicitly add stock to a known existing quantity
  Given my draft suggests an existing resource with known quantity 2
  When I choose "Add quantity" and explicitly enter 3
  Then the normal revision-checked increment command acknowledges quantity 5
  And no other draft detail is copied or discarded automatically
  And I choose explicitly whether to retain or discard the remaining draft
```

### Edge Case

```gherkin
Scenario: Preserve the selected target while candidates refresh
  Given I selected candidate A at its displayed revision
  When the duplicate query refreshes and displays candidate B
  Then my submitted action still names candidate A and its displayed revision
  And the server revalidates candidate A before any mutation
```

```gherkin
Scenario: Candidate changes before an update command
  Given I prepared an update for a suggested existing resource
  When another device edits or archives that resource before I submit
  Then my command returns a conflict or unavailable result without mutation
  And my full new-resource draft and intended update remain available
  And no quantity, details, links, or draft disposition change automatically
```

## 18. Test Strategy

- Domain tests prove each choice maps to an explicit ordinary command and suggestions never mutate.
- SQLx integration tests retrieve actual persisted candidates by authored text and
  identifier, exclude archived records, dedupe stable IDs, and return no fabricated fallback.
- Domain and component tests prove refreshed candidate lists cannot retarget an
  action and duplicate queries cannot alter shared draft/save state.
- API tests cover candidate retrieval/revalidation, auth, CSRF, idempotency, archive races,
  quantity prerequisites, separate create, and no draft disposition side effect.
- Component tests cover full text/identifier/detail drafts, navigation return, conflict,
  failed actions, explicit discard, stable focus, and accessible labels.
- Playwright covers desktop and Android possible-duplicate workflows with refresh during interaction.

## 19. Implementation Notes

- Purpose: duplicate resolution owns real persisted lexical/identifier retrieval and
  draft-safe decisions while coordinating explicit resource commands.
- Callers: resource creation editor, resource preview, resource and quantity commands,
  and later e06 consumers.
- Contracts: advisory candidates, ID/revision-bound actions, server-side revalidation, separate draft/saved/cache
  state, explicit mutation, explicit draft disposition, stable return path, and no automatic merge.
- Keep the coordinator thin: reuse e04s01 create/update and e04s02 quantity commands rather than
  adding a broad “resolve duplicate” mutation endpoint.
- Keep lexical/identifier retrieval available independently; e06 may later enrich
  candidates semantically through the same read-only contract.
- Reuse e01s03 `DraftSaveCoordinator`; do not depend on or recreate the e06
  interaction coordinator.

## 20. Definition Of Done

- Every duplicate choice preserves the full draft until explicit acknowledged disposition.
- Similarity alone never blocks separate creation or changes an existing resource.
- Candidate retrieval is proven against persisted resources; production behavior
  never fabricates deterministic duplicate candidates.
- Refresh tests prove an action remains bound to its selected candidate ID/revision
  without an e06 interaction coordinator.
- Mutation revalidation, failures, conflicts, archived candidates, and unknown quantity are automated.
- Desktop and Android workflows meet accessibility and text/identifier/detail draft-recovery requirements.
- `just test`, `just lint`, and `just build` pass.
- Final `just preflight` reports no new security findings in affected e04s03 paths.
