# ADR-003: Sync Layer Selection

| Field | Value |
|---|---|
| **ADR** | ADR-003 |
| **Title** | Select PowerSync as the Client Synchronization Layer |
| **Status** | Accepted |
| **Date** | 2026-03-11 |
| **Deciders** | Altair architecture/design |
| **Related Docs** | `altair-architecture-spec.md`, `ADR-002-primary-database-selection.md` |

---

# Context

Altair requires a synchronization architecture that supports:

- **offline-first clients**
- **SQLite on clients**
- **Android as a first-class client**
- **desktop clients on Linux and Windows**
- **web access**
- **eventual multi-user shared state**
- **partial replication**
- **reasonable real-time propagation**
- **conflict-aware behavior**
- **self-hosted deployment**

Potential sync approaches considered included:

- **PowerSync**
- **Electric**
- **custom sync layer**
- **CRDT-first approaches**

The project initially leaned toward a custom operation-based sync engine. However, after evaluating the actual requirements and stack choices, it became clear that Altair already has enough difficult product and platform problems without also building a full sync substrate from scratch.

---

# Decision

Altair will use **PowerSync** as the primary synchronization layer between:

- **PostgreSQL** on the server
- **SQLite** on supported clients

PowerSync will be used to:

- replicate selected subsets of canonical server data to local client databases
- support offline-first reads and writes on clients
- propagate changes across devices and users within supported sync scopes
- reduce the need for a custom sync protocol in v1

Attachments are **out of scope** for PowerSync’s main data path and will be handled separately through object storage and attachment APIs.

---

# Rationale

## 1. Best fit for Postgres → SQLite architecture

Altair’s selected architecture is:

- PostgreSQL as primary DB
- SQLite on Android and desktop clients
- offline-first local operation

PowerSync aligns unusually well with this design. It reduces impedance mismatch between backend and client data stores.

## 2. Reduces one of the highest-risk engineering problems

A custom sync engine would require designing and operating:

- mutation logs
- device checkpoints
- resumable transfer logic
- retry queues
- fan-out propagation
- conflict handling
- schema compatibility strategy
- sync debugging tooling

That is a substantial engineering subsystem on its own.

Choosing PowerSync shifts effort toward product behavior rather than sync plumbing.

## 3. Supports partial replication

Altair clients should not receive the entire universe of data.

Examples of likely sync scopes:

- a user’s current initiatives
- a household’s inventory
- a user’s routines and active quests
- notes related to active/shared contexts
- attachment metadata for synced entities

PowerSync is a good fit for this style of selective client working set.

## 4. Compatible with multi-user shared state

Altair is not only a single-user multi-device system. Over time it is expected to support shared contexts such as:

- household inventory
- shared chores
- shared maintenance tasks
- shared lists

These are mostly structured-state synchronization problems rather than collaborative text editing problems.

PowerSync is sufficient for this level of shared state, provided sync scopes remain deliberate and well-modeled.

## 5. Avoids premature CRDT complexity

Altair does not currently require full CRDT semantics for most of its core domains.

Examples like:

- completing “take out the trash”
- decrementing inventory
- recording a maintenance event

are better modeled as structured state updates or event-log operations than as CRDT-first collaboration.

---

# Consequences

## Positive

- strong fit with PostgreSQL + SQLite architecture
- reduced implementation risk
- faster time to first usable cross-device prototype
- simpler local-first client architecture
- less need to invent/debug low-level sync machinery
- workable path for shared household state

## Negative

- PowerSync becomes a significant architectural dependency
- sync scope design must be shaped around what is practical to replicate
- relationship-heavy or graph-derived replication scopes may become awkward
- schema evolution and sync compatibility must still be handled carefully

## Neutral / Follow-on implications

- complex graph traversal should not be used as the primary method for deciding client sync scope
- sync scopes should be based on clear boundaries such as:
  - user
  - household
  - initiative
  - project
  - shared context

- attachment binaries should remain outside the main sync mechanism
- some domains may still need explicit conflict UX and audit history

---

# Alternatives Considered

## Electric

### Pros

- attractive for shared, real-time, multi-user state
- strong conceptual fit for subset syncing of Postgres data
- more interesting if Altair becomes heavily collaboration-centric

### Cons

- less directly aligned with the chosen SQLite-on-client model
- more infrastructure overhead for current goals
- stronger fit for different local-store assumptions in some cases

### Why not chosen now

Electric remains a credible alternative, especially if Altair’s future collaboration requirements become more central than currently expected.

It was not chosen because PowerSync is the clearer fit for the current architecture and delivery priorities.

## Custom sync layer

### Pros

- maximum control
- exact domain-specific semantics
- no third-party sync dependency

### Cons

- very high complexity
- large delivery risk
- significant maintenance burden
- high chance of spending v1 effort on plumbing instead of product

### Why rejected

Rejected because building a custom sync system is too expensive and risky relative to the current stage of the project.

## CRDT-first system

### Pros

- excellent for collaborative editing and merge-heavy concurrent data

### Cons

- poor fit for much of Altair’s structured domain state
- adds complexity not justified by current workflows

### Why rejected

Rejected because Altair’s collaboration needs are currently centered on shared state, not simultaneous rich-text collaboration.

---

# Decision Details

PowerSync is selected as the default synchronization strategy for v1.

This decision assumes:

- PostgreSQL remains the primary source of truth
- SQLite remains the client-local database
- sync scopes are modeled deliberately
- file attachments are handled separately
- conflict handling is still designed at the product/domain level where needed

This is a practical choice, not a claim that PowerSync is universally optimal.

---

# Constraints and Guardrails

To keep this decision healthy, Altair should:

1. avoid sync-scope definitions that require overly complex relationship traversals
2. separate attachment binaries from row-level sync
3. model shared scopes explicitly (user, household, initiative, etc.)
4. build a few real prototype workflows early:
   - shared household inventory
   - shared chores/tasks
   - cross-linked initiative/task/note subset sync

5. revisit only if implementation reveals material mismatch

---

# Revisit Triggers

This ADR should be reconsidered if:

- PowerSync cannot express or operate required sync scopes cleanly
- multi-user/shared-state behavior becomes the dominant architectural constraint
- schema evolution becomes unreasonably painful
- collaboration requirements move significantly closer to live co-editing
- client local storage strategy changes away from SQLite

---

# Final Note

This decision optimizes for **delivery speed**, **client architecture fit**, and **risk reduction**.

The team should treat PowerSync as:

- the current best architectural fit
- chosen until disproven by implementation
- not sacred

That is a much healthier engineering posture than trying to achieve false certainty in a domain where the real truth usually appears during implementation.
