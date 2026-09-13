# e05s01: Link And Unlink Records Bidirectionally

## 1. Metadata

- **Story ID:** e05s01
- **Epic:** e05 Cross-Domain Relationships
- **Type:** feat
- **Context:** relationships-domain
- **Status:** todo
- **BCPs:** 3
- **Risk:** P1

## 2. User Story

As the workspace owner, I want to link any two records once and see that relationship from either record, so connected work, knowledge, and resources remain discoverable without copies.

## 3. Problem

Records need one stable, symmetric `related` relationship. Directional or duplicated storage would allow reversed duplicates, inconsistent views, and accidental endpoint deletion during unlinking.

## 4. Outcome

An authenticated user can create or remove one `related` link for two distinct active records. Both endpoint views reflect the same relationship, and retries are deterministic.

## 5. In Scope

- Link projects, tasks, notes, and resources in any cross-domain or same-domain pair.
- Canonically order endpoints by domain kind and stable ID.
- Enforce uniqueness by relationship kind and canonical endpoint pair.
- Return an existing link for an idempotent repeated create request.
- Remove only the relationship and expose the result from both endpoints.

## 6. Out Of Scope

- User-defined relationship kinds, directions, labels, or explanations.
- Project association and project-board membership.
- Automatic links from recall suggestions.
- Record archive, restore, or purge behavior.
- Graph visualization.

## 7. Domain Terms

- **Record identity:** Stable domain kind plus stable record ID.
- **Related link:** Symmetric explicit connection between two records.
- **Canonical endpoint pair:** Endpoints sorted by domain kind, then stable ID.
- **Self-link:** A relationship whose two identities are equal; it is invalid.
- **Project association:** A separate relationship that controls board membership.

## 8. Preconditions

- The user has an authenticated application session.
- Both endpoints exist and are active.
- Stable IDs are assigned by the authoritative store.

## 9. Dependencies

- e01 secure sessions, typed errors, database access, and migrations.
- Domain record identity contracts from e02, e03, and e04.
- PostgreSQL transactions and SQLx explicit migrations.

## 10. Data

- Store `relationship_id`, `kind = related`, canonical `left_kind`, `left_id`, `right_kind`, `right_id`, and timestamps.
- Add a database unique constraint over kind and both canonical endpoints.
- Add a database check that the two endpoint identities differ.
- Validate endpoint existence and active state in the authoritative transaction.
- Never duplicate endpoint records or derive identity from mutable labels.

## 11. API

- A domain-oriented create-link command accepts two typed record identities and `related` kind.
- A delete-link command identifies the canonical relationship and is idempotent when already absent.
- A record relationship query returns the opposite endpoint regardless of stored endpoint position.
- Responses use typed validation, not-found, conflict, and authorization errors without exposing internals.
- Success is reported only after server acknowledgement.

## 12. UI

- Record details show related records with kind, identifying label, and navigation action.
- The link picker excludes the active record and marks already linked records.
- Unlink requires an explicit action and removes the item from both endpoint views after acknowledgement.
- Project associations remain visually distinct from general related links.

## 13. Security

- Require the authenticated application session and normal CSRF protection for mutations.
- Resolve both endpoint records inside the user's personal workspace boundary.
- Do not expose record existence through distinguishable unauthorized responses.
- Do not log personal labels or content in mutation diagnostics.

## 14. Failure Modes

- Missing or archived endpoint: reject without creating a link.
- Self-link: return a typed validation error.
- Reversed or exact duplicate: return the existing canonical link without another row.
- Database failure: roll back and leave both endpoint views unchanged.
- Unlink retry after success: acknowledge the absent relationship without deleting either record.

## 15. Concurrency

- Concurrent exact and reversed creates converge on one row through canonicalization and database uniqueness.
- A create racing endpoint archival must lock or validate consistently so no link is acknowledged to an ineligible endpoint.
- Concurrent unlink requests are idempotent.
- Reads from either endpoint use the same committed relationship.

## 16. Accessibility

- Link and unlink controls have accessible names that include the target label.
- Every pointer action has a keyboard and touch-usable equivalent.
- Success and error state changes are announced without stealing focus.
- Do not rely on color or endpoint position to communicate relationship meaning.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Canonical bidirectional related links

The system stores one symmetric related link for two distinct active records and exposes it from both endpoints.

```gherkin
Scenario: Happy path links and unlinks two records bidirectionally
  Given an active note and an active resource are not related
  When the user links the note to the resource
  Then one canonical related link is committed
  And each endpoint lists the other record
  When the user unlinks them
  Then neither endpoint lists the relationship
  And both records still exist unchanged
```

### Edge Case

```gherkin
Scenario Outline: Edge cases preserve canonical uniqueness and endpoint safety
  Given two relationship endpoint requests described by <case>
  When the create-link command is handled
  Then the result is <result>
  And the relationship row count is <count>
  Examples:
    | case                                      | result                         | count |
    | the same record at both endpoints         | a typed validation error       | 0     |
    | an exact duplicate of an existing link    | the existing link acknowledgement | 1  |
    | a reversed duplicate of an existing link | the existing link acknowledgement | 1  |
    | one archived endpoint                     | a typed ineligible error       | 0     |
```

## 18. Test Strategy

- Unit-test endpoint canonicalization across every domain-kind ordering and equal identities.
- Integration-test database check and uniqueness constraints, bidirectional queries, idempotent delete, and transaction rollback.
- Race exact and reversed create requests and create against endpoint archival.
- Component-test link picker exclusions, existing-link state, acknowledgement timing, and accessible controls.
- Browser-test link then unlink from opposite endpoint on desktop and touch-sized layouts.

## 19. Implementation Notes

- Purpose: the relationships domain owns symmetric explicit-link invariants and commands.
- Callers: record detail APIs and UIs, recall link actions, imports, exports, and restore workflows.
- Contracts: stable typed identities, canonical ordering, uniqueness, no self-links, server-acknowledged mutation, and unlink-without-delete.
- Keep domain logic independent of Axum extractors and SQLx rows; map at boundaries.
- Use one ordinary relationships module and one explicit migration. Reason for Depth: shared canonicalization must be identical for every caller and protected by the database.
- No new external packages are proposed; slopcheck is not applicable.

## 20. Definition Of Done

- All acceptance scenarios have automated coverage.
- Migration constraints and concurrent duplicate behavior are verified against PostgreSQL.
- Both endpoint APIs and responsive UI expose the same link.
- Authentication, CSRF, typed errors, and accessibility tests pass.
- `just test`, `just lint`, `just build`, and `just preflight` pass.
- Preflight reports no new security findings in affected relationship paths.
- No task is marked passing until its verify command exits successfully.
