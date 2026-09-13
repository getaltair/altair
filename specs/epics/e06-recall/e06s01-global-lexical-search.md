# e06s01: Global Cross-Domain Lexical Search

## 1. Metadata

- **Story ID:** e06s01
- **Epic:** e06 Search And Contextual Recall
- **Type:** feat
- **Context:** recall-domain
- **Status:** todo
- **BCPs:** 3
- **Risk:** P1

## 2. User Story

As the workspace owner, I want one search across projects, tasks, notes, and resources, so I can retrieve saved material without remembering its domain or classification.

## 3. Problem

Domain-only browsing cannot reliably retrieve untitled fragments, inconsistent terminology, or identifiers. Search must remain useful when semantic inference is unavailable.

## 4. Outcome

Authenticated global search returns real active records from every domain using text and identifier matching, supports optional domain narrowing, and honestly reports an empty result without implying semantic coverage.

## 5. In Scope

- Search saved project, task, note, and resource text fields.
- Search domain identifiers stored on authoritative records.
- Combine PostgreSQL full-text search and `pg_trgm` matching.
- Expand lexical matches through one hop of active explicit relationships so directly connected active records can be found even when their own fields do not match the query.
- Deduplicate by stable record identity and rank deterministic ties.
- Filter by one or more domain kinds and exclude archived records by default.

## 6. Out Of Scope

- Semantic/vector retrieval, live unsaved-input recall, and suggestions.
- OCR, website ingestion, or arbitrary attachment-content extraction.
- Search folders, tags, saved queries, or advanced query language.
- Searching archived records unless a later explicit archived-search surface requires it.
- Attachment-name search. e07s01 integrates authoritative attachment names into this search contract after attachment metadata exists.
- Implementing possible-duplicate resource retrieval; e04s03 already supplies real persisted lexical/identifier candidates for e06s03 to consume.
- Semantic relationship inference, automatic linking, and transitive relationship traversal beyond one explicit hop.

## 7. Domain Terms

- **Global search:** Explicit query across all supported record domains.
- **Lexical match:** Full-text, trigram, or domain-identifier match.
- **Stable identity:** Domain kind plus record ID used for deduplication.
- **Authoritative detail:** Label and excerpt derived from saved record data.
- **Coverage:** Retrieval channels actually used for a result set.
- **Relationship-derived match:** An active record reached through one explicit relationship from a lexical match, with the matched record identified as its retrieval basis.

## 8. Preconditions

- The user has an authenticated application session.
- Searchable records have been saved to PostgreSQL.
- Required PostgreSQL extensions and explicit indexes are available.

## 9. Dependencies

- Domain read models from e02, e03, and e04.
- e01 authenticated API and PostgreSQL migration foundation.
- e01s03 shared `DraftSaveCoordinator` for preserving any originating editor draft/save/conflict state across result navigation and return.
- e04 domain read models include the searchable resource fields; e04s03 separately owns persisted lexical/identifier duplicate candidates.
- e07s01 is a later integration point for attachment-name search, not an e06s01 dependency.

## 10. Data

- Build searchable documents from canonical authored fields without replacing source data.
- Keep per-kind field mapping explicit so different domain schemas remain distinct.
- Return stable identity, record kind, authoritative label, excerpt/detail, source revision, and lexical evidence.
- Expand each lexical seed through active one-hop relationships, return the seed identity as relationship evidence, and never treat that evidence as a lexical match on the related record.
- Prefer a record's direct lexical evidence when it is both a lexical and relationship-derived match, and return it once by stable identity.
- Exclude archived records at query time and after ranking.
- Index updates follow authoritative record commits; stale derived documents are rebuildable.

## 11. API

- `GET` or equivalent read operation accepts validated query text, optional domain filters, cursor, and bounded limit.
- Empty normalized input returns an empty complete lexical result without database-wide matching.
- Response reports `coverage.lexical = complete` and does not claim semantic execution.
- Response reports whether relationship expansion completed and identifies relationship-derived results without changing ownership of the lexical query contract.
- Results never expose PostgreSQL ranks or internal query syntax.
- Responses containing personal data use `Cache-Control: no-store`.
- The same lexical query contract feeds general e06s03 recall; e06s03 consumes possible-duplicate resource candidates from e04s03 rather than reimplementing them here.

## 12. UI

- A global search entry is available without first selecting a domain.
- Results show record kind, identifying label, useful excerpt/detail, and lexical evidence when understandable.
- Relationship-derived results identify the directly connected lexical match as their understandable retrieval basis.
- Domain filters are optional and reversible.
- Keyboard and touch users can open a result and return to the same query and position.
- When search starts from an editor, navigation and return preserve the e01s03 `DraftSaveCoordinator` draft/save/conflict state; search defines no parallel editor state machine.
- Empty and error states are distinct.

## 13. Security

- Require the authenticated session for query and result access.
- Validate query length, filters, pagination, and limits before constructing SQL.
- Bind SQL parameters and never interpolate query text.
- Do not log raw query text or result excerpts.
- Use `Cache-Control: no-store` for responses and no shared client cache.

## 14. Failure Modes

- Empty or whitespace query: return no results without an error.
- Invalid domain filter or excessive limit: return a typed validation error.
- Database unavailable: show search unavailable without affecting editing, saving, or direct relationships.
- Stale derived search data: source revision permits detection and rebuild.
- No match: state that lexical search found no matches, not that no relevant content exists.
- Archived or removed relationship endpoint: omit that endpoint without suppressing valid lexical matches.

## 15. Concurrency

- Each result carries the authoritative source revision read for its snapshot.
- Pagination uses deterministic ordering so equal scores do not reorder unpredictably.
- Direct lexical matches rank before relationship-derived matches at an equal deterministic tie position; expansion remains one hop and cycle-safe.
- A record archived during search is filtered before publication or rejected when opened.
- Late responses in the global search UI cannot replace results for a newer query generation.

## 16. Accessibility

- Search input has a persistent accessible label and announces result count changes politely.
- Domain filters expose selected state programmatically.
- Result kind and archive/error status are conveyed in text, not color alone.
- Keyboard focus remains in the search workflow after results refresh.

## 17. Acceptance Criteria

### Happy Path

#### ADDED: Cross-domain lexical search

Global search retrieves active saved records across all domains from authored fields and domain identifiers with optional domain narrowing.

```gherkin
Scenario: Happy path retrieves saved records across domains
  Given active project, note, task, and resource records contain matching text or identifiers
  And an active non-matching note is explicitly related to a matching project
  When the user searches globally for that input
  Then matching records from every applicable domain are returned once by stable identity
  And the related note is returned once with the matching project identified as relationship evidence
  And each result shows its kind, authoritative label, detail, revision, and understandable lexical evidence
  And applying a resource filter returns only resource matches
  And live recall can consume those authoritative global-search candidates
```

### Edge Case

```gherkin
Scenario Outline: Edge queries remain safe and honest
  Given <condition>
  When global lexical search runs
  Then <outcome>
  Examples:
    | condition | outcome |
    | whitespace-only input | an empty complete lexical result is returned without scanning all records |
    | an archived matching record | the archived record is excluded |
    | the same record matches title, body, and identifier | one result is returned for its stable identity |
    | a record is both a direct lexical match and related to another match | one result is returned with its direct lexical evidence |
    | a matching record links to an archived record | the archived related record is excluded while the matching record remains |
    | related records form a cycle or a chain longer than one hop | each direct active endpoint is returned once and traversal stops after one hop |
    | the database is unavailable | a typed unavailable result is shown and ordinary editing remains usable |
    | no lexical fields match | the UI says no lexical matches and makes no semantic-coverage claim |
```

## 18. Test Strategy

- Unit-test per-domain searchable-field extraction, normalization, relationship evidence, identity deduplication, and deterministic tie ordering.
- Integration-test full-text, trigram, domain identifiers, one-hop active relationship expansion, archive filtering, filters, limits, and parameter binding.
- API-test authentication, validation, pagination, `no-store`, and payload-safe logging.
- Component-test filter state, empty/error distinction, late-response rejection, and return-path preservation around e01s03 `DraftSaveCoordinator` state.
- Browser-test global search and domain narrowing at desktop and Android widths.

## 19. Implementation Notes

- Purpose: the recall domain provides one lexical retrieval contract over distinct domain records.
- Callers: global search UI and e06s03 server recall operation.
- Contracts: active saved lexical and one-hop relationship-derived candidates, stable-identity deduplication, authoritative details, bounded deterministic results, explicit coverage, and no raw ranking leakage.
- Keep SQL ranking in the PostgreSQL adapter and map results into domain-neutral recall candidates.
- Keep relationship expansion subordinate to the lexical seed query; this story does not own semantic relationship inference or mutate links.
- Reason for Depth: one shared lexical operation prevents search and live recall from drifting in field coverage or archive safety.
- Use selected PostgreSQL capabilities only; no new external package is proposed.

## 20. Definition Of Done

- Every domain and lexical field class has automated positive and negative coverage.
- One-hop active relationship expansion, relationship evidence, cycle safety, archive exclusion, and direct-match deduplication are verified.
- Archive exclusion, stable ordering, authentication, no-store, and safe logging are verified.
- Search stays usable independently of inference.
- e06s01 does not duplicate e04s03 possible-duplicate retrieval; e06s03 consumes those real persisted candidates directly.
- Attachment-name search is deferred explicitly to e07s01 integration.
- Responsive keyboard and touch workflows pass accessibility tests.
- Editor-originated search navigation preserves the shared e01s03 `DraftSaveCoordinator` state.
- `just test`, `just lint`, `just build`, and `just preflight` pass before task statuses change.
- Preflight reports no new security findings in affected search paths.
