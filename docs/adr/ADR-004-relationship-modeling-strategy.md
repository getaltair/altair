# ADR-004: Relationship Modeling Strategy

| Field | Value |
|---|---|
| **ADR** | ADR-004 |
| **Title** | Model Cross-Entity Relationships as First-Class Domain Records |
| **Status** | Accepted |
| **Date** | 2026-03-11 |
| **Deciders** | Altair architecture/design |
| **Related Docs** | `altair-architecture-spec.md`, `ADR-002-primary-database-selection.md`, `ADR-003-sync-layer-selection.md` |

---

# Context

A major part of Altair’s product value is not only storing entities, but surfacing, preserving, and reusing the **relationships between them**.

Examples include:

- a note references an item
- a note supports an initiative
- a quest requires materials
- an inventory item is linked to maintenance notes
- a task is related to a routine or household context
- two notes are semantically related
- an AI workflow suggests that one record is relevant to another

These relationships are important because they may be:

- **explicitly created by users**
- **imported from external systems**
- **suggested by AI**
- **derived by rules**
- **confirmed, rejected, or ignored over time**
- **reused later for search, guidance, planning, and automation**

This means relationships are not merely implementation details. They are part of the product’s persistent knowledge model.

The database decision chose PostgreSQL rather than a graph-native primary database. Therefore the architecture needs a deliberate way to preserve graph-like product behavior within a relational system.

---

# Decision

Altair will model relationships as **first-class domain records**, rather than hiding them inside ad hoc foreign keys, embedded blobs, or inference-only projections.

The primary implementation pattern will be explicit relationship records stored in PostgreSQL.

These records will represent both:

- **canonical structural relationships**
- **user-created or inferred semantic relationships**

Each relationship may also carry metadata describing:

- source
- confidence
- status
- evidence
- timestamps
- lifecycle state

---

# Relationship Categories

Altair will treat relationships as belonging to at least three categories.

## 1. Canonical structural relationships

These are system-defined relationships essential to core domain structure.

Examples:

- quest belongs to initiative
- item belongs to household
- item stored in location
- attachment attached to note

These may still be modeled with traditional foreign keys where appropriate.

## 2. Explicit user-defined relationships

These are relationships intentionally created or confirmed by a user.

Examples:

- note references item
- note supports quest
- quest uses item
- note linked to note

These should be modeled as explicit relationship records.

## 3. Inferred or suggested relationships

These are created by:

- AI pipelines
- semantic search
- rules engines
- import heuristics
- duplicate detection
- parsing workflows

Examples:

- note may relate to initiative
- item may match shopping list line
- note may duplicate another note
- quest may depend on previously completed task pattern

These should also be modeled as explicit relationship records, with richer metadata and lifecycle states.

---

# Rationale

## 1. Relationships are a core product primitive

Altair’s value depends on helping users see and use connections across Guidance, Knowledge, and Tracking.

If relationships are not persisted and queryable as first-class records, then much of that value becomes fragile, opaque, or impossible to reuse.

## 2. Relationship lifecycle matters

The system must be able to answer questions like:

- who created this relationship?
- was it user-created or AI-suggested?
- how confident is the system?
- was it accepted or dismissed?
- what evidence supported it?
- when was it last confirmed?

A simple foreign key or one-off join table is not enough for this.

## 3. Works well with PostgreSQL

This design preserves graph-like behavior without requiring a graph-native primary database.

It also works cleanly with:

- PowerSync replication
- SQLite local clients
- search/index pipelines
- AI enrichment pipelines
- audit/history needs

## 4. Supports future automation and ranking

First-class relationship records make it possible to build:

- suggested next actions
- related-note discovery
- inventory-to-task linkage
- maintenance reminders
- semantic relevance boosts
- household-aware guidance
- explanation UI for AI-generated links

Without persisted relationship records, these features become harder to explain, tune, and trust.

---

# Decision Details

## Structural relationships

Use conventional relational modeling where the relationship is inherently part of the entity’s identity or lifecycle.

Examples:

- `quest.initiative_id`
- `item.household_id`
- `attachment.note_id`

These remain valid and should not be replaced with abstraction for abstraction’s sake.

## Cross-entity semantic relationships

Use explicit relationship records for relationships that are:

- many-to-many
- cross-domain
- user-created
- inferred
- evidence-bearing
- reviewable
- query-worthy in their own right

## Baseline relationship record shape

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
status
confidence
evidence_json
created_by_user_id
created_by_process
created_at
updated_at
last_confirmed_at
```

### Possible enums

#### relation_type
- references
- supports
- requires
- stored_with
- duplicates
- similar_to
- depends_on
- related_to
- blocks
- generated_from

#### source_type
- user
- ai
- import
- rule
- migration
- system

#### status
- accepted
- suggested
- dismissed
- rejected
- expired

---

# Consequences

## Positive

- relationships become durable and explainable
- graph-like product features remain possible in PostgreSQL
- inferred links can be reviewed and reused later
- cross-domain queries become more deliberate
- relationship metadata can influence ranking, automation, and UX

## Negative

- additional schema and indexing complexity
- more careful API design required
- some structural relationships will exist both as foreign keys and as semantic relationships, requiring discipline
- relationship lifecycle management becomes an explicit product concern

## Neutral / Follow-on implications

- relationship indexing strategy must be designed carefully
- semantic/inference pipelines should write relationship records, not just ephemeral suggestions
- UI needs patterns for:
  - viewing relationships
  - accepting/rejecting suggestions
  - seeing evidence/explanations

---

# Alternatives Considered

## Only use foreign keys and join tables

### Pros

- simple
- conventional
- easy to understand at first

### Cons

- poor support for inferred or evidence-bearing links
- hard to manage lifecycle and confidence
- weak foundation for relationship-centric product features

### Why rejected

Rejected because it under-models a core source of product value.

## Keep inferred relationships only in search/vector indexes

### Pros

- simpler primary DB schema
- keeps semantic logic outside transactional model

### Cons

- suggestions become ephemeral
- hard to explain or review
- hard to preserve user acceptance/rejection over time
- weak support for downstream automation

### Why rejected

Rejected because relationships need persistent lifecycle and reuse.

## Use a graph-native primary database instead

### Pros

- elegant graph traversal model
- natural edge records

### Cons

- conflicts with chosen PostgreSQL + PowerSync direction
- increases infrastructure and delivery risk

### Why rejected

Rejected because the database decision has already been made in favor of PostgreSQL, and this relationship strategy provides an effective relational implementation path.

---

# Design Guardrails

1. Do not create relationship records for every plain ownership link just because the table exists.
2. Do create relationship records when the relation is valuable as a domain artifact.
3. Keep structural truth and semantic relationships conceptually distinct.
4. Preserve evidence and provenance for inferred relationships.
5. Design APIs so clients can:
   - fetch related entities
   - inspect relationship metadata
   - accept/reject suggestions
   - create manual links

6. Ensure sync scopes include relationship records needed for local relevance.

---

# Example Use Cases

## Knowledge to Tracking
A note about replacing HVAC filters may reference:
- the HVAC unit item
- a filter SKU item
- a maintenance routine
- a household location

These links should be queryable and reusable later.

## Guidance to Tracking
A quest such as “replace server UPS batteries” may require:
- inventory items
- purchase reminders
- attached notes
- maintenance history

These relationships should be visible to planning and tracking workflows.

## AI-assisted linking
An AI enrichment process may infer that:
- a captured receipt image relates to specific household items
- a maintenance note relates to a specific asset
- a note probably supports an active initiative

Those suggestions should be persisted with confidence and evidence, not discarded after one UI session.

---

# Revisit Triggers

Revisit this ADR if:

- relationship records become unmanageably complex
- the domain requires a more formal ontology system
- performance characteristics strongly suggest separate graph projections
- UX indicates users do not benefit from persisted relationship lifecycle

---

# Final Note

This ADR is the bridge between:

- a relational primary database
- a graph-shaped product experience

It is one of the most important modeling decisions in Altair because it protects the product’s long-term ability to surface meaning, not just store data.
