# e06s03: Latest-Only Live Contextual Recall

## 1. Metadata

- **Story ID:** e06s03
- **Epic:** e06 Search And Contextual Recall
- **Type:** feat
- **Context:** recall-domain
- **Status:** todo
- **BCPs:** 8
- **Risk:** P0
- **Decision:** ADR-0001 latest-only contextual recall

## 2. User Story

As the workspace owner, I want relevant saved records to appear from my current unsaved input without interrupting typing, so I can recall connected work across domains while preserving my draft and acting only by choice.

## 3. Problem

Live recall combines mutable unsaved context, delayed work, lexical and semantic retrieval, browser lifecycle changes, and fallible inference. Without one latest-only coordinator, old responses can replace newer context, hidden pages can accumulate work, and semantic failure can masquerade as no relevance.

## 4. Outcome

Every project, task, note, and resource creation and editing surface submits immutable snapshots to one latest-only client session. One stateless authenticated server operation returns an atomic cross-domain batch; only the current session, context revision, and dispatch generation can publish it, with honest degraded coverage and no draft mutation.

## 5. In Scope

- Domain-specific immutable snapshots containing current text and relevant surrounding context from both create and edit surfaces for projects, tasks, notes, and resources.
- Quiet-period automatic refresh plus separately available manual refresh.
- Session ID, monotonically increasing context revision, and monotonically increasing dispatch generation.
- Best-effort cancellation, server-owned semantic deadline, and correctness through generation matching.
- Combined lexical, identifier, relationship, and compatible semantic retrieval with stable-identity deduplication.
- Real persisted lexical/identifier possible-duplicate resource candidates supplied by e04s03; no fake, fixture-only, generated, or e06-owned duplicate-query path in live behavior.
- Pause/resume, disposal, retry, automatic-refresh and automatic-presentation controls, breadth, and atomic publication.

## 6. Out Of Scope

- Conversational answers, autonomous agents, generated summaries, or automatic mutations.
- Progressive streaming or partial batch publication.
- Saving unsaved recall input or requiring save before recall.
- Provider registry, hidden cloud fallback, reranking, or custom inference protocol.
- Guaranteeing every query returns every domain or only accepted suggestions.

## 7. Domain Terms

- **Recall session:** Stateful client coordinator for one visible editor lifecycle.
- **Context revision:** Monotonic number assigned whenever recall-relevant input changes.
- **Dispatch generation:** Monotonic number assigned whenever a request is dispatched or prior work is invalidated.
- **Context key:** Deterministic hash of normalized recall inputs defined by `tech-stack.md`.
- **Atomic batch:** One immutable result set published all at once.
- **Coverage:** Complete or degraded status for lexical, relationship, and semantic channels.

## 8. Preconditions

- Each create and edit surface for projects, tasks, notes, and resources can provide a typed immutable domain snapshot without persisting it.
- e06s01 global lexical search and e04s03 real persisted resource-duplicate candidates are available; e06s02 compatible vectors enable semantic coverage when available.
- The user has an authenticated application session.
- Automatic refresh, automatic presentation, breadth, and result amount have explicit defaults.

## 9. Dependencies

- e06s01 global lexical retrieval.
- e06s02 durable provenance-compatible embeddings.
- e05 relationships as retrieval anchors and existing-link state.
- e04s03 real persisted lexical/identifier duplicate-candidate contract.
- e01s03 shared `DraftSaveCoordinator` plus domain editor adapters from e02 through e04; recall reads immutable drafts but does not own save/conflict state.
- `DESIGN_PLAN_LATEST.md` and ADR-0001 invariants are authoritative.

## 10. Data

- Each request contains session ID, context revision, dispatch generation, context key, domain kind, typed snapshot, breadth, and bounded result limit.
- Normalized context includes domain kind, draft or record identity, recall text fields, active note paragraph, sorted project associations, and relationship anchors.
- Unsaved input remains transient and is never written by recall.
- Each candidate contains stable identity, record kind, authoritative label/detail snapshot, source revision, existing-link state, evidence, archived state when explicitly included, and coverage.
- Candidate batches are deduplicated by stable identity and exclude the active and archived records by default.
- Possible duplicate resources are the actual saved resource records from e04s03, carrying authoritative identities, revisions, details, and lexical/identifier evidence; production recall never manufactures or re-queries a parallel candidate source.

## 11. API

- One stateless authenticated recall operation accepts one validated immutable request and returns one immutable batch.
- Server retrieval combines available lexical, identifier, relationship, and provenance-compatible semantic channels according to breadth and limit.
- Semantic timeout/failure returns available non-semantic results with explicit degraded semantic coverage.
- Transport or database failure returns unavailable; it never blocks or mutates editor state.
- Response uses `Cache-Control: no-store`, validates all boundary data, and exposes no raw ranks, distances, provider payloads, or mutation operation.

## 12. UI

- Project, task, note, and resource create and edit surfaces show `idle`, `waiting`, `loading`, `ready`, `unavailable`, or `paused` status without stealing focus.
- Suggestions update only after the quiet period or explicit manual refresh.
- Breadth changes retrieval; automatic presentation and amount shown remain separate controls.
- Automatic presentation may be hidden while manual recall stays available.
- Desktop uses a related-content panel; compact layouts provide a touch-usable entry point without leaving the active workflow.
- Recall status remains separate from e01s03 `DraftSaveCoordinator` saving, saved, error, and conflict states; recall failures cannot transition editor save state.

## 13. Security

- Require application-session authentication and normal CSRF protection for recall requests.
- Keep provider credentials on the server and route only to the explicitly configured endpoint.
- Never log raw recall text, active paragraphs, labels, excerpts, vectors, or provider payloads.
- Prevent shared HTTP/server-response caching and return `Cache-Control: no-store`.
- Complete inference privacy verification before sending real content, including transport and non-retention evidence.

## 14. Failure Modes

- Semantic timeout or failure: publish available lexical/relationship results with degraded coverage.
- Whole-operation transport/database failure: enter `unavailable`, retain the draft, and offer retry.
- Empty context: invalidate prior generation, cancel work, clear candidates, and enter `idle`.
- Malformed boundary data: reject with typed error and do not publish a partial batch.
- Archived/deleted candidate or stale revision after retrieval: exclude before publication; mutations later revalidate independently.
- Hidden page or disposed editor: invalidate, cancel, and prevent late publication.

## 15. Concurrency

The coordinator must implement and test this transition table:

| Current state | Event | Required transition and effect |
|---|---|---|
| any non-disposed | non-empty input, auto refresh on | increment context revision and generation, cancel work/timer, enter `waiting`, start new quiet timer |
| any non-disposed | non-empty input, auto refresh off | increment context revision and generation, cancel work/timer, enter `idle`, hide prior batch when context key differs |
| any non-disposed | empty input | increment context revision and generation, cancel work/timer, clear candidates, enter `idle` |
| `waiting` | current quiet timer expires | increment dispatch generation, dispatch snapshot, enter `loading` |
| visible non-disposed | manual refresh with non-empty context | invalidate timer/work, increment generation, dispatch immediately, enter `loading` |
| `loading` | matching successful response | atomically publish batch and enter `ready` |
| `loading` | matching semantic-only failure | atomically publish non-semantic batch with degraded coverage and enter `ready` |
| `loading` | matching transport failure | retain draft, enter `unavailable` |
| any non-disposed | response mismatches session, revision, or latest generation | discard with no publication or state regression |
| `unavailable` | retry | increment generation, dispatch latest snapshot, enter `loading` |
| any visible state | page hidden | increment generation, cancel timer/work, retain current published batch, enter `paused` |
| `paused` | manual refresh | dispatch nothing and remain `paused` |
| `paused` | resume | read latest snapshot; enter `waiting` only for non-empty context with auto refresh on, otherwise `idle`; never revive timer |
| any non-disposed | dispose | increment generation, cancel timer/work, enter terminal `disposed` and reject all later events/publication |

Cancellation is an optimization. Session, context revision, and latest dispatch generation matching are the only publication correctness rule.

## 16. Accessibility

- Status and degraded coverage are announced politely without announcing every keystroke or stealing focus.
- Suggestions and controls are fully keyboard operable and touch sized on Android.
- Automatic display can be reduced or hidden independently of recall breadth.
- Motion or refresh never changes the focused control or a target under active interaction; e06s04 owns the complete interaction freeze contract.
- Error, paused, empty, and degraded states are distinguishable in text.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Latest-only cross-domain live recall

One coordinator retrieves real saved records from unsaved context on both create and edit surfaces for every domain, publishes only the latest complete batch, and degrades without blocking editing or saving.

```gherkin
Scenario Outline: Happy path recalls across domains from unsaved create and edit input
  Given the <surface> has non-empty current recall context and automatic refresh is enabled
  When input remains quiet for the configured period
  Then one request is dispatched with the current session, context revision, generation, and immutable <domain> snapshot
  And one matching atomic batch can publish projects, tasks, notes, and resources once by stable identity
  And the draft remains unsaved and unchanged
  Examples:
    | surface | domain |
    | project create surface | project |
    | project edit surface | project |
    | task create surface | task |
    | task edit surface | task |
    | note create surface | note |
    | note edit surface | note |
    | resource create surface | resource |
    | resource edit surface | resource |

Scenario: Happy path returns a real lexical duplicate resource candidate
  Given a saved resource has a name or identifier similar to the current resource create or edit input
  When live recall receives lexical or identifier candidates from e04s03
  Then the possible duplicate is that saved authoritative resource with its stable identity, revision, detail, and lexical evidence
  And no fake or generated resource candidate is substituted
```

### Edge Case

```gherkin
Scenario Outline: Edge generations and cancellations never publish obsolete work
  Given <sequence>
  When all timers, cancellation callbacks, and responses settle
  Then <outcome>
  Examples:
    | sequence | outcome |
    | revision 1 dispatches, revision 2 dispatches, then response 1 arrives last | only response 2 can publish |
    | two manual refreshes share one context revision and the first returns last | only the newest dispatch generation can publish |
    | cancellation fails and the cancelled request succeeds | its response is discarded by generation matching |
    | input becomes empty while loading | work is cancelled, candidates clear, state is idle, and the response cannot publish |
    | the page hides while waiting | the timer is cancelled, state is paused, and no request dispatches |
    | the page hides while loading and the response returns | the batch is retained only if it was already published; the late response is discarded |
    | the page resumes with changed non-empty context and auto refresh on | a new quiet period starts from the latest snapshot and no old timer revives |
    | the page resumes with auto refresh off | state is idle and no request dispatches |
    | the editor disposes while waiting or loading | state remains disposed and no timer, retry, callback, or response publishes |

Scenario Outline: Edge degraded and control states preserve ordinary work
  Given <condition>
  When recall is requested
  Then <outcome>
  Examples:
    | semantic inference times out but lexical retrieval succeeds | lexical results publish atomically with semantic coverage degraded |
    | semantic response has incompatible provenance | it is excluded and coverage is degraded without mixing vectors |
    | the database or transport fails | state becomes unavailable, draft and save controls remain usable, and retry uses the latest snapshot |
    | automatic refresh is disabled | input invalidates old results but dispatches nothing until manual refresh |
    | automatic presentation is hidden | manual recall remains available without automatic display |
    | a complete latest request has zero candidates | ready shows an honestly empty complete batch rather than unavailable |
    | a candidate equals the active record or is archived | that candidate is excluded by default |
```

## 18. Test Strategy

- Use fake timers and deterministic fake gateways to exhaustively test every state/event pair and invalid transition.
- Generate permutations of input, manual refresh, timer expiry, response, retry, hide, resume, cancellation result, and disposal across successive revisions/generations.
- Assert no obsolete response can alter state, candidates, coverage, or accessibility announcements.
- Server integration-test channel combination, stable-identity deduplication, active/archive filtering, e04s03 real lexical/identifier duplicate-resource candidates, provenance compatibility, deadlines, and atomic batch errors.
- Contract/security-test auth, CSRF, no-store, configured-provider-only routing, no fallback, and payload-safe logging.
- Component/browser-test create and edit surfaces for every domain, e01s03 `DraftSaveCoordinator` state isolation, draft preservation, focus stability, Android lifecycle, controls, and semantic/total outage degradation.

## 19. Implementation Notes

- Purpose: the recall domain owns latest-only interactive retrieval behavior and its immutable public candidate contract.
- Callers: project, task, note, and resource create and edit surfaces plus manual recall UI.
- Contracts: all eight create/edit integrations, unsaved-input transience, e04s03 real duplicate candidates, session/revision/generation publication guard, atomic batches, explicit coverage, read-only behavior, draft preservation, e01s03 save/conflict ownership, and separate breadth/presentation controls. e06s04 exclusively owns displayed-target freeze during suggestion actions.
- Implement one framework-light TypeScript coordinator behind a composable; keep Vue components and CodeMirror adapters thin. Keep server orchestration independent of Axum, SQLx, and provider payload types.
- Reason for Depth: central state ownership is required because four editor types share race-sensitive timers, browser lifecycle, cancellation, and stale-response rejection.
- Use the existing stack and standard cancellation primitives; no new external package is proposed.

## 20. Definition Of Done

- The full transition table and all Gherkin examples are automated with deterministic timing.
- Every project, task, note, and resource create and edit surface participates using current unsaved context without recall persistence or draft mutation.
- Real lexical/identifier duplicate-resource candidates are integrated from e04s03; e06 adds no duplicate retrieval or fake production candidates.
- Semantic timeout, semantic incompatibility, transport failure, and recovery remain honest and non-blocking.
- Recall remains isolated from e01s03 `DraftSaveCoordinator` save, error, retry, and conflict transitions.
- Privacy, auth, CSRF, no-store, payload-safe logging, focus, and responsive UI tests pass.
- `just test`, `just lint`, `just build`, and `just preflight` pass.
- Preflight reports no new security findings in affected recall, editor-integration, and inference paths.
- No task is marked passing until its verify command exits successfully.
