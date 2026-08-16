# DR-002: PostgreSQL is the structured store

**Status:** Accepted
**Date:** 2026-08-15
**Supersedes:** nothing
**Related:** Altair Architecture Foundations, Altair System Architecture, Altair Component Model, Altair v0 Scope, DR-001, DR-007

---

## Context

The architecture foundations set sixteen weighted requirements across the data model and queries, and the rule for using them: a product meeting most of what matters beats two products each meeting half, the bar for a second product is that the gap is load-bearing and no single product closes it, and any accepted gap is named in advance and carries its workaround. Both gates that had to precede this are closed.

**One requirement decides the choice.** Literal text matching, similarity search over stored vectors, scoping, and the audience predicate must resolve against one candidate set in one query. It is load-bearing, because post-filtering leaks and gets limits wrong. That eliminates every architecture assembled from a database plus a search engine plus a vector store, whatever their individual merits, and it cannot be taken as an accepted gap.

**A second constraint narrows the field further.** Read-your-own-writes requires the three paths to share one store, so this is a single product holding entities, relations, bodies, versions, the change sequence, conflict state, derived text, embeddings, both search indexes, and configuration.

---

## Decision

**PostgreSQL, self-hosted, with pgvector for similarity search and its built-in full text search for literal matching.**

Concretely:

1. **One store, as the foundations require.** Everything the structured store is responsible for lives in one database. The object store holds file bodies and nothing else, per DR-003.
2. **Relations are rows in one table**, referencing entities by identity, with direction as a property of the single record and no reverse row. Backlinks are derived by querying the far endpoint, never maintained. A relation record surviving the deletion of an endpoint is a predicate on that endpoint's lifecycle state, so deletion and restore write no relation rows, and erasure removes them.
3. **Both indexes live beside the audience predicate**, which is what makes one candidate query possible rather than merely convenient.
4. **The derivation queue is a table**, claimed with `SELECT ... FOR UPDATE SKIP LOCKED`, with a notification from the write path as the signal that something changed. It is not a second product. This follows the component model rather than adding to it: outstanding work is computed from provenance in the store, and the queue is an optimisation over that computation which may be lost without loss.
5. **Self-hosted, not a hosted Postgres product.** The same software either way, so this is a deployment choice and reversible in both directions.
6. **Shape is validated on write.** This closes the posture the foundations deliberately left open. Partial data being always valid is a statement about completeness, not about shape: nearly every column is nullable and an entity with a title and nothing else is ordinary, but what a record is composed of is checked where it enters. The read path is the part expected to churn continuously, and it must not become the component that interprets shapes.
7. **PostgreSQL 18 is the baseline.** Nothing here requires 19.

---

## Alternatives considered

### SurrealDB

A single engine covering relational, document, graph, vector, and full-text, with hybrid retrieval fused in-engine.

**Rejected, and the closest of the alternatives by a wide margin.** On fit to the requirements as written it is arguably ahead. Reciprocal rank fusion over a BM25 index and an HNSW index is an engine function rather than SQL to be written and maintained. Relations as records belonging to neither endpoint is its native shape rather than a table to model, and traversal is a prefix scan rather than a join. Externally assigned identifiers are the default. The durability objection that would have decided this a year ago no longer holds: synced writes became the default in 3.0.

It is rejected on two grounds, neither technical. The core is licensed under the Business Source License 1.1, which is not an open source licence and converts to Apache 2.0 four years after each release. Nothing about Altair's use is restricted, since the prohibition is on offering the database as a managed service. The objection is that the vision's operating model exists so that nothing a household depends on can be repriced or withdrawn by another party, and a licence controlled going forward by a venture-funded company with a separate enterprise edition relocates that exposure from the hosting layer to the source layer rather than removing it. Second, three major versions in roughly two years, each rebuilding significant internals. The write path is small, correctness-critical, and rarely touched by design, and sitting it on a foundation that is rebuilt this often is a trade against exactly that property.

### SQLite with FTS5 and sqlite-vec

**Rejected, and the honest runner-up.** It closes the decisive requirement with fewer moving parts, and at household scale its concurrency limits are not binding. Rejected because its vector extension is considerably younger than pgvector, and because it forecloses moving any component onto separate hardware, which the foundations explicitly anticipate for inference.

### A relational database with a separate search engine or vector store

**Rejected on the decisive requirement.** Separate systems can only post-filter, which the foundations name as leaking and getting limits wrong. This is the accepted-gap rule doing its job: the gap is load-bearing, so a second product would need to close it, and splitting is what opens it.

### Document stores

**Rejected.** A record belonging to neither of the things it joins has no natural home in a model built around self-contained documents, which the foundations flag as the requirement that most constrains the shape of a store. The decisive query requirement is also unmet.

### Graph databases

**Rejected.** They win a requirement that is only strongly preferred, since backlinks are one hop and nothing in this design traverses deep, and lose the one that is load-bearing.

### A hosted Postgres product

**Rejected for the instance, not as software.** It is the same engine, so nothing about the schema or the queries differs. What it adds is convenience the operator already has covered, and what it costs is infrastructure that a third party can price or withdraw, which the vision's operating model exists to rule out. The exit was always self-hosting the same software, so this decision takes the exit at the start rather than holding it in reserve.

---

## Accepted gaps

Named in advance, each with its mitigation, per the foundations.

| Gap | Weight | Mitigation |
|---|---|---|
| Approximate vector indexes filter after scanning, so a restrictive predicate can collapse recall | Load-bearing where it bites | Three tiers chosen by the size of the filtered set: pre-filter and compute exact distances when the set is small, use the index when the predicate passes nearly everything, and iterative index scans in between. The filtered size is a cheap count on ordinary indexed columns. Single-user v0 does not reach the regime where this bites |
| Literal matching is weaker than a dedicated engine on typos and prefixes | Strongly preferred | Trigram matching alongside full text search |
| Fusion across the two retrieval arms is written and maintained here rather than provided by the engine | Convenience | It is a bounded amount of SQL, and holding it means it can be tuned, which is the arm of the pipeline expected to churn |
| Random identifiers scatter inserts across an index rather than clustering them at its end | Convenience | None taken. The remedy is an internal surrogate key, rejected below with the condition that reopens it |

**The internal surrogate key is rejected rather than scheduled.** The remedy for scattered inserts is a monotonic internal key that every index clusters on, with the identifier carried alongside as an ordinary unique column. Its price is a second key on every table keyed by identity, a lookup wherever a reference arrives from outside, and a second thing about every row that can be wrong. Household write volume is nowhere near where scattered inserts cost anything, and taking the key now would be paying that price against a projection rather than against a measurement. It is reopened when insert cost on `entity` starts to track table size rather than staying flat, which is a thing to observe. The identifier scheme in DR-007 is unaffected either way, because a surrogate key is internal to this store and never crosses the interface.

---

## Consequences

**Gained**

- The decisive query requirement is met in one product, so no candidate set is assembled across systems
- Relations, backlinks, and cross-domain retrieval are ordinary queries
- The derivation queue costs no additional product
- Conditional writes against a per-record counter, and externally assigned identifiers, are ordinary
- Deletion, restoration, and the horizon stay computable by predicate, so time continues to produce no writes

**Given up**

- Hybrid fusion is maintained rather than supplied
- Relations are modelled rather than native, which is cheap here but is a real advantage given up

**Obligations this creates**

- **The embedding dimension is fixed in the schema by the model chosen.** Changing models means re-embedding the corpus. Provenance already records what produced each derived value, so this is recoverable rather than fatal, but it is not a choice to drift into.
- **Canonical ordering for symmetric and untyped relations is the write path's responsibility.** Symmetry is a property of the type declaration, which a constraint cannot see, so without it the same connection can be recorded twice in opposite directions and surface as duplicate backlinks.
- **Do not make choices that foreclose export**, carried forward unchanged from DR-001.

---

## Deliberately not decided here

- The runtime and language, which is the next choice and constrains the client, the outbox, and the read path
- The object store product, which is DR-003
- The embedding model, which fixes the dimension above
- How blocks, bodies, and type-specific content are represented in the schema
- Whether to adopt PostgreSQL 19 when it ships. SQL/PGQ is a read-only view layer over the same tables and changes nothing here. `ON CONFLICT DO SELECT` would simplify idempotent outbox replay from two statements to one, which is worth taking when it is available and is not worth waiting for

---

## Notes

The SurrealDB assessment was made against its state in August 2026 rather than against a prior formed earlier, and two things in it moved the answer: durability defaults were corrected in 3.0, and in-engine rank fusion is a real advantage over writing the same thing in SQL. The decision turned on licence and release culture rather than on capability. If either changes, this is worth reopening, and the reasoning above is recorded so that reopening it costs a comparison rather than a rediscovery.
