# e06s02: Durable Embedding Indexing And Provenance

## 1. Metadata

- **Story ID:** e06s02
- **Epic:** e06 Search And Contextual Recall
- **Type:** feat
- **Context:** recall-infrastructure
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want saved material indexed automatically and recoverably, so semantic recall survives inference outages and never mixes incompatible or stale vectors.

## 3. Problem

Embedding generation is an external, fallible operation. Running it inline with saves would block authoritative data, while ephemeral jobs, missing provenance, or stale vectors could silently lose indexing work or return invalid suggestions.

## 4. Outcome

Every recall-relevant committed content revision creates durable indexing work without waiting for inference. A retryable worker stores vectors only with validated provenance, discards obsolete work safely, and supports complete rebuild from authoritative content.

## 5. In Scope

- Persist indexing work atomically with recall-relevant authoritative record changes.
- Generate embeddings through the configured OpenAI-compatible `/v1/embeddings` endpoint.
- Persist content revision, model identity/revision, dimensions, and relevant embedding configuration.
- Retry bounded transient failures with backoff and survive process restart.
- Rebuild derived vectors and remove or deactivate stale, archived, and purged candidates.

## 6. Out Of Scope

- Blocking saves on embedding generation.
- A broker, separate indexing service, provider registry, or custom inference protocol.
- Automatic fallback to another provider or model.
- Reranking, custom model management, training, OCR, or attachment-content embeddings.
- Treating vectors as authoritative or backup-required user content.

## 7. Domain Terms

- **Indexing work item:** Durable request to embed one authoritative content revision.
- **Embedding provenance:** Model identity/revision, dimensions, configuration, and source content revision.
- **Obsolete work:** Work for a source revision superseded before vector publication.
- **Compatible vector:** Query and stored vector produced under the same accepted provenance contract.
- **Derived index:** Rebuildable retrieval data, not authoritative content.

## 8. Preconditions

- PostgreSQL 18 and pgvector are available.
- The deployment declares one inference endpoint, model identifier, transport, and server-side credentials as applicable.
- Recall-relevant content extraction is deterministic for each domain record revision.

## 9. Dependencies

- e01 authoritative persistence, configuration, migrations, and worker execution foundation.
- e01s03 authoritative revision commits and shared `DraftSaveCoordinator` acknowledgement contract; indexing observes committed revisions and never owns editor save/conflict state.
- Saved domain records from e02 through e04.
- e06s01 candidate identity and archive filtering contracts.
- Privacy verification required by `DESIGN_PLAN_LATEST.md` before real content is sent.

## 10. Data

- Work item records source kind/ID/revision, status, attempt count, next attempt, and non-sensitive failure class.
- Vector records source identity/revision plus model identity/revision, dimensions, embedding configuration fingerprint, and vector.
- Unique keys prevent duplicate active work for the same source revision and duplicate vectors for the same complete provenance.
- Worker publication rechecks current source revision and lifecycle state atomically.
- Archived or purged records cannot retain actionable vectors; rebuild enumerates authoritative active records.

## 11. API

- Keep indexing internal to the server; ordinary save responses do not wait for it.
- A focused inference adapter sends validated standard embedding requests and validates response count, dimensions, numeric values, and model contract.
- Operational commands can inspect backlog health and trigger a rebuild without exposing personal content or credentials.
- Semantic query rejects incompatible provenance rather than mixing vectors.
- No browser endpoint exposes provider credentials or raw vectors.

## 12. UI

- Saving remains successful when indexing is pending or inference is unavailable.
- Recall can state semantic coverage is degraded without presenting indexing internals as user cleanup work.
- No mandatory indexing dashboard or manual maintenance routine is introduced.
- Operational status may expose counts and age, never content payloads.

## 13. Security

- Send content only to the explicitly configured endpoint over the accepted private or authenticated TLS transport.
- Keep credentials server-side and redact payloads, vectors, excerpts, and credentials from logs.
- Verify application, proxy, and inference non-retention before sending real content.
- Permit no hidden cloud fallback.
- Bound request size, response dimensions, timeout, and parsing.

## 14. Failure Modes

- Inference timeout/unavailable: retain durable work and schedule bounded retry; saves remain unaffected.
- Validation, authentication, or permanent provider error: retain visible failed work without automatic retry loops.
- Process restart: pending or leased-expired work becomes eligible again.
- Source changes while inference runs: discard the obsolete result and process current durable work.
- Dimension/model mismatch: reject publication and mark the work incompatibly configured.
- Archive/purge races: prevent the vector from becoming actionable.

## 15. Concurrency

- Claim work with a durable lease or PostgreSQL locking so workers do not publish duplicate active results.
- Worker completion compares source revision, lifecycle, and provenance in one transaction.
- New saves enqueue newer revisions without waiting for or cancelling authoritative persistence.
- Rebuild and incremental indexing converge through the same uniqueness and obsolescence rules.
- Retry is safe because embedding work and publication are idempotent for complete provenance.

## 16. Accessibility

- User-facing degraded coverage is announced in text and does not rely on color.
- Saving status remains separate from indexing/recall status.
- Any operational status surface uses accessible tables or lists with explicit labels.
- No transient inference state steals focus from editing.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Durable provenance-safe embedding index

Authoritative saves enqueue durable asynchronous embedding work, and only current vectors with compatible complete provenance become queryable.

```gherkin
Scenario: Happy path indexes a saved revision asynchronously
  Given an active note revision commits successfully
  When durable indexing work is processed by the configured embeddings endpoint
  Then the save acknowledgement does not wait for inference
  And one vector is published for that exact content revision
  And its model identity, model revision, dimensions, and configuration fingerprint are stored
  And compatible semantic queries can use it
```

### Edge Case

```gherkin
Scenario Outline: Edge failures preserve authority and provenance
  Given <condition>
  When indexing runs
  Then <outcome>
  Examples:
    | condition | outcome |
    | inference is unavailable across an application restart | the save remains committed and durable work retries later |
    | source revision changes before an old response returns | the old vector is not published as current and newer work remains eligible |
    | response dimensions differ from configuration | the vector is rejected and the failure is recorded without retrying as transient |
    | a record is archived while work is in flight | no actionable vector is published for the archived record |
    | the index is deleted and rebuilt | equivalent current work and provenance are derived from authoritative active records |
```

## 18. Test Strategy

- Unit-test content extraction, provenance compatibility, retry classification, backoff bounds, and response validation.
- Integration-test transactional enqueue, lease recovery, restart, unique publication, archive/purge races, and rebuild convergence.
- Contract-test the exact `/v1/embeddings` subset against a deterministic fake endpoint.
- Fault-test timeouts, malformed responses, wrong counts/dimensions, auth failure, and obsolete responses.
- Verify privacy canary evidence, configured-provider-only routing, no fallback, and payload-safe logs before real content.

## 19. Implementation Notes

- Purpose: recall infrastructure turns authoritative revisions into rebuildable, provenance-safe vectors.
- Callers: authoritative domain save transactions enqueue work after e01s03 revision rules succeed; e06s03 semantic retrieval reads compatible current vectors. The shared `DraftSaveCoordinator` remains the editor save/conflict owner.
- Contracts: non-blocking saves, durable work, bounded retry, complete provenance, current-revision publication, no fallback, and rebuildability.
- Use a focused inference client and PostgreSQL-backed worker inside the one Rust application.
- Reason for Depth: the durable worker boundary is required to survive external failure and restart without coupling authoritative saves to GPU availability.
- Existing OpenAI-compatible HTTP and selected database capabilities suffice; no new external package is proposed.

## 20. Definition Of Done

- Save latency and success do not depend on inference availability.
- Restart, lease expiry, retry classification, stale revision, archive/purge, and rebuild scenarios pass.
- Provenance compatibility prevents mixed model/configuration vectors.
- Privacy verification exists before real content reaches inference, with no secrets or personal payloads recorded.
- `just test`, `just lint`, `just build`, and `just preflight` pass before task statuses change.
- Preflight reports no new security findings in affected indexing, inference, and operational paths.
