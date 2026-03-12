# ADR-002: Primary Database Selection

| Field | Value |
|---|---|
| **ADR** | ADR-002 |
| **Title** | Select PostgreSQL as the Primary Database |
| **Status** | Accepted |
| **Date** | 2026-03-11 |
| **Deciders** | Altair architecture/design |
| **Related Docs** | `altair-architecture-spec.md`, `altair-core-prd.md`, `altair-guidance-prd.md`, `altair-knowledge-prd.md`, `altair-tracking-prd.md` |

---

# Context

Altair requires a primary persistence layer for:

- Guidance domain data
- Knowledge domain data
- Tracking domain data
- cross-domain relationships
- multi-user sharing over time
- search/indexing pipelines
- attachment metadata
- AI-derived relationship metadata
- synchronization with offline clients

The initial database candidates considered were:

- **PostgreSQL**
- **SurrealDB**

SurrealDB was attractive because Altair has many explicit and inferred relationships across entities, and a major product goal is surfacing, storing, and reusing those relationships over time.

However, Altair also has several non-negotiable architectural constraints:

- **offline-first clients**
- **SQLite on clients**
- **web + Android as first-class clients**
- **desktop clients for Linux and Windows**
- **self-hosting**
- **high ecosystem stability**
- **future multi-user shared state** such as household inventory and shared chores

These constraints shift the decision from "which database best matches the conceptual graph shape of the domain" to "which database best supports reliable product delivery."

---

# Decision

Altair will use **PostgreSQL** as the **primary system-of-record database**.

Related technologies may include:

- **pgvector** for embedding/vector storage
- PostgreSQL full-text search features where appropriate
- additional indexing/search subsystems as needed
- SQLite as the local client database
- object storage for binary attachments

PostgreSQL will store:

- canonical domain entities
- explicit cross-entity relationships
- inferred/suggested relationships
- sync-related metadata needed by the backend and sync subsystem
- attachment metadata
- audit/event history where needed

---

# Rationale

## 1. Maturity and operational stability

PostgreSQL is the more mature, better-supported, and lower-risk choice.

Advantages include:

- broad ecosystem support
- strong tooling
- well-understood operations
- reliable migrations
- strong backup/restore story
- excellent hosting and self-hosting support

For Altair, this reduces infrastructure and delivery risk.

## 2. Better fit with offline-first sync strategy

PostgreSQL aligns cleanly with the selected sync strategy:

- **Postgres on server**
- **SQLite on clients**
- **PowerSync for synchronization**

This pairing is significantly more mature and direct than a SurrealDB-based sync architecture.

## 3. Relationship modeling does not require a graph-native primary DB

Although Altair has graph-like data, PostgreSQL can still model relationships as **first-class domain records**.

Altair does not merely need links. It needs relationships that can be:

- created explicitly by users
- inferred by AI or rule systems
- reviewed later
- ranked or filtered
- accepted or rejected
- queried across domains

That behavior can be modeled with explicit relational structures.

### Example relationship record shape

```text
entity_relations
----------------
id
from_entity_type
from_entity_id
to_entity_type
to_entity_id
relation_type
source_type
confidence
status
evidence_json
created_at
updated_at
last_confirmed_at
```

This preserves the product’s graph-oriented behavior while keeping the primary persistence layer stable and conventional.

## 4. Extension ecosystem is strong

PostgreSQL can be extended as Altair evolves.

Examples:

- **pgvector** for embeddings
- full-text search
- JSONB for flexible metadata payloads
- recursive queries where helpful
- materialized views or projections for search/index pipelines

This provides a strong balance between flexibility and stability.

## 5. SurrealDB remains attractive, but not enough to outweigh delivery risk

SurrealDB’s graph/document/vector ergonomics are appealing, especially for inferred and explicit relationship storage.

However, adopting it as the primary database would introduce additional risk in areas where Altair already has complexity:

- multi-client sync
- Android/local database strategy
- ecosystem/tooling maturity
- operational familiarity
- delivery speed for v1

That tradeoff is not justified at this stage.

---

# Consequences

## Positive

- lower infrastructure risk
- easier adoption of PowerSync
- easier operational support
- easier schema evolution
- broad compatibility with self-hosted deployment targets
- straightforward fit for analytics, indexing, and admin workflows

## Negative

- graph-style traversal is less elegant than in a graph-native database
- relationship-heavy queries may require more deliberate schema and indexing work
- semantic and graph-oriented projections may eventually require dedicated subsystems

## Neutral / Follow-on implications

- relationship modeling must be treated as a **first-class domain design problem**
- PostgreSQL should not be used as an excuse to flatten or under-model relationships
- specialized projections for search, semantic linking, or graph exploration may still be added later

---

# Alternatives Considered

## SurrealDB as primary database

### Pros

- graph-native relationship ergonomics
- multi-model design
- attractive conceptual fit for cross-domain relationship storage
- possible embedded usage in some environments

### Cons

- weaker fit with the selected sync direction
- more delivery and ecosystem risk
- less mature operational posture for this use case
- increased risk of spending v1 effort on infrastructure rather than product behavior

### Why rejected

Rejected for now because Altair’s higher-risk problem is **reliable offline sync across clients**, not the inability to model graph-like relationships in PostgreSQL.

---

# Decision Details

PostgreSQL is the **authoritative persistence layer** for Altair v1 and likely v2.

This decision does **not** prohibit:

- search projections
- vector indexes
- graph-like relationship tables
- later addition of specialized stores for semantic or graph-heavy workloads

It only defines the **primary system of record**.

---

# Follow-up Work

1. Define core schema for:
   - Guidance
   - Knowledge
   - Tracking
   - shared/core entities
   - relationship records

2. Define indexing strategy for:
   - relationship lookups
   - cross-domain search
   - semantic enrichment metadata

3. Define sync-facing schema boundaries for PowerSync replication.

4. Revisit the decision only if:
   - relationship queries become unreasonably awkward
   - sync or indexing requirements materially change
   - PostgreSQL becomes the main blocker rather than an enabler

---

# Final Note

This decision optimizes for **delivery realism**, **ecosystem maturity**, and **sync compatibility**.

Altair’s product value depends more on:

- good relationship modeling
- good sync behavior
- good UX for shared/offline state

than on using the most aesthetically pleasing database for graph-shaped data.
