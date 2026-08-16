# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repository is right now

**Design documents only. There is no source code, no build system, and no tests yet.** Everything in `docs/` specifies a self-hosted personal system ("Altair") that has not been built. The first code lands as Wave 0 of `docs/altair-v0-implementation-plan.md`.

Consequences for working here:

- There is nothing to build, lint, or run. Do not invent commands or claim to have verified behaviour.
- Changing a document is the work. Treat prose with the care normally reserved for code: these documents are the authority the implementation will be checked against.
- When implementation starts, it starts at Wave 0 — see [The codebase that will be built](#the-codebase-that-will-be-built).

## Document authority

The documents form a strict hierarchy. **Every document names its own position in its header** (`Status`, `Date`, `Governed by`, `Related`) — read that header before editing anything.

```text
altair-vision.md                          normative, permanent product identity
  ├── altair-substrate-spec.md            cross-cutting behaviour (the layer under all three domains)
  ├── altair-guidance-prd.md              \
  ├── altair-knowledge-prd.md              }  domain behaviour
  ├── altair-tracking-prd.md              /
  ├── altair-relation-types-spec.md       relation type behaviour (belongs to no single domain)
  ├── altair-architecture-foundations.md  what kind of system, before any component
  │     └── altair-architecture.md        components and what they owe each other
  │           └── altair-component-model.md  assembled: every boundary, and what each side owes on absence
  ├── altair-data-model.md                assembled: every persistent thing, decides nothing
  ├── altair-v0-scope.md                  sequencing; defers, never amends
  │     └── altair-v0-implementation-plan.md
  └── DR-001 … DR-007                     product/technology choices made against the above
```

Rules that follow from this and are easy to violate:

- **Assembled documents decide nothing.** `altair-data-model.md` and `altair-component-model.md` restate what their authorities require. Where one disagrees with the substrate spec, the domain PRDs, or the architecture, **the assembled document is wrong** and gets corrected — never the other way around.
- **`docs/altair-scratchpad.md` has no authority.** Entries are `open` / `leaning` / `parked`, each with a destination. Writing something there is not deciding it. Its `Rejected` section exists so "why not just do X?" has an answer without re-arguing.
- **The vision's Must / Should / Won't are product identity, not a backlog.** v0 narrowing a Must is temporary and must be recorded under "Deferred Musts" in `altair-v0-scope.md`; it does not amend the vision.
- **Design before architecture before tools.** Specs say what must be true; the architecture says what structure delivers it; DRs choose products. A spec that names a product, or an architecture document that picks a database, has escaped its layer.
- **Scope discipline.** The substrate holds no domain behaviour; if a substrate section starts describing what a quest does, it has escaped. Domain PRDs hold no mechanism.

## Documents that are already accepted as artefacts

- `docs/altair-schema.sql` — the structured store, adopted as **migration one**. Applied and exercised on PostgreSQL 18.6 with pgvector 0.8.6. Do not re-derive it.
- `docs/altair.proto` — the public interface (`package altair.v1`). **Field numbers are permanent; removed fields are reserved, never reused.** Closing one of its recorded gaps means adding a field, not editing one.
- `docs/altair-outbox-conformance.md` — the outbox specification made executable, written once and run against every implementation (Rust and Kotlin). Scenarios observe behaviour at two boundaries only: what the person sees, and what reaches the instance.

## Architecture in one pass

**One self-hosted instance serves one household.** The instance core is the authority: it owns the public interface, every write, audience enforcement, and retrieval orchestration. Stores sit behind it, clients in front, inference beside it and never authoritative. Process count is packaging, not architecture.

- **Structured store** (PostgreSQL, DR-002): entities, relations, categories, membership, versions, derived text, embeddings, **and both search indexes**. The indexes live here because the audience predicate must sit *inside* the candidate query.
- **Object store** (filesystem behind put/get/delete/enumerate, DR-003): file bodies and nothing else. `enumerate` is load-bearing — reclamation sweeps for unreferenced bytes.
- **Derivation worker**: embeddings and their provenance. Queue is a table claimed with `SELECT … FOR UPDATE SKIP LOCKED`; it is an optimisation over work computable from the store, and may be lost without loss.
- **Inference**: several independently-absent models. An instance without a cross-encoder is conforming, not degraded.

**Three domains over one entity model** — Guidance (campaign → arc → quest), Knowledge (notes, files), Tracking (items, locations). Cross-domain relations and one-pass cross-domain search are the load-bearing product claim, in scope from the first running build.

**Write path**: acceptance is local and durable on the device before the person is told; credentials and connectivity never participate. A durable append-only outbox replays to the instance. Writes are field-scoped and carry a base counter. Overlapping concurrent changes retain both sides.

**Read path**: literal and similarity arms generate candidates in the store, fused by rank position, optionally reranked. Literal matching is a permanent arm, not a fallback — which is why a just-captured entity is findable by its words before derivation runs. Retrieval degrades per stage and says so; it does not fail.

**Change stream** is polled with a client-held position, and is **per member** — audience broadening and narrowing must arrive as creation and deletion to the affected member. One shared stream filtered late is where a leak happens.

## The codebase that will be built

`docs/altair-v0-implementation-plan.md` sequences it. **It prescribes no file list and no directory layout** — every item is an outcome with a verification condition ("Done when: …"), and the layout is decided while implementing. Preserve that: do not invent a canonical module tree and treat it as settled.

Seven waves. Waves 1–3 are planned against reality; Wave 5 onward names outcomes and deliberately leaves shape open. **Re-plan at each wave boundary, and only the next wave.**

| Wave | Produces | Notes |
|---|---|---|
| 0 · Plumbing | Cargo workspace, proto codegen in the build, migration runner (`altair-schema.sql` as migration one), integration harness, CI | Harness stands up a **real PostgreSQL**, not a mock — the audience predicate, both indexes and `SKIP LOCKED` are Postgres behaviour. Done when a fresh checkout runs one command and passes an empty suite. |
| 1 · Foundations | 1.1 store bootstrap · 1.2 block division · 1.3 object store · 1.4 token validation · 1.5 outbox conformance suite | Five genuinely independent lanes, one worktree each. 1.5's deliverable is a **red suite** — every scenario runs and fails. |
| 2 · Write path | 2.1 intent spine · 2.2 type content, all three domains · 2.3 file bodies · 2.4 reclamation | 2.1 is sequential and load-bearing; do not parallelise the spine, do parallelise what hangs off it. |
| 3 · Read path | 3.1 literal retrieval arm · 3.2 change stream and horizon · 3.3 health | Literal only. Build the instance half of the change stream even though the TUI need not consume it — omitting the instance side is one-way. |
| 4 · Terminal client | 4.1 Rust outbox (turns 1.5 green) · 4.2 ratatui client | **First useful day.** The TUI carries the whole editing surface; there is no browser and no second client. |
| 5 · Semantic | derivation worker · inference boundary and bi-encoder · semantic arm and fusion | The embedding model is chosen **here**, against a real corpus, and fixes the schema dimension. |
| 6 · Second client and ops | message bridge · packaging · backup/restore/upgrade | The bridge is the first test of whether the interface carries the obligations rather than the TUI's code. |

**Two lanes cross wave boundaries and should not wait:** the terminal client shell can run against a fake instance from Wave 1, and the conformance suite needs nothing but the generated contract.

Specifics worth knowing before touching the relevant lane:

- **The audience predicate is written once**, in the store bootstrap layer, and both paths call it. A test asserts it appears in exactly one place.
- **Block division is a pure function over text**, run only at the instance (DR-004). Atomic constructs never split (fenced code, tables, diagrams); list items do split; identity survives edits to a block and to its neighbours. A long unbroken stretch of prose being one block is correct, not a bug.
- **The intent spine**: idempotent replay returns the original acknowledgement, the intent row is written in the transaction it acknowledges, a stale base is never a rejection (reject-and-retry is the familiar pattern and it is wrong), both values are retained on a same-part conflict, and refusal on audience is indistinguishable from refusal on nonexistence.
- **The change sequence allocates from a single position row in the write transaction, so every write serialises on it. That is intended** — a sequence leaves gaps and a poller can read past an uncommitted position and never see it. Record why prominently near the code; this is exactly the shape of thing a future reader removes as an obvious bottleneck.
- **`altair-schema.sql` already contains Wave 2's test plan**, under *"Checks this file cannot make, which the write path owes."* Turn that list into the suite.
- **Two schema gaps close in Wave 0**: the embedding dimension stays a placeholder in its own late migration, and cycle prevention in nested locations and categories becomes a write-path check rather than a constraint.
- **Outstanding derivation is computed from provenance in the store, never from the queue.** Losing the queue costs time, not reportability.
- **The horizon is null, or longer than every other retention window. A middle value is a bug**, not a tuning choice.

**Deferred decisions have triggers, not dates** — the table at the end of the plan names the moment each becomes cheap to take correctly (embedding model at Wave 5, bridge transport at 6.1, retention constants at 2.4, the surrogate key on `entity` only when insert cost tracks table size). Do not take one early.

### Workflow the plan assumes

Built with Claude Code + hyperskills. The plan maps situations to skills: `plan` at a wave boundary (next wave only), `orchestrate` with one worktree per lane for a wave with 3+ parallel lanes, `cross-model-review` before merging anything touching the write path, conflict detection, or audience, `brainstorm` then `research` when a deferred decision hits its trigger, `implement` for everything else.

At every wave boundary ask: what did this wave teach that the next wave's plan does not know, which deferred decision just hit its trigger, and has the shape drifted from the component model? The third catches sprawl that green tests do not.

## Branching and pull requests

This repository uses [GitHub Flow](https://docs.github.com/en/get-started/using-github/github-flow): `main` is the single trunk, always in a working state, and every change reaches it through a pull request from a short-lived branch.

- Branch off `main` for any change, including a documents-only edit. Name branches for what they contain (`docs/...`, `wave-0/...`, or similar) — there is no enforced prefix scheme.
- Open the pull request early, even in draft, once the branch has something worth showing — GitHub Flow treats the PR as the place discussion happens, not just a final gate.
- `main` is protected on GitHub: pushes go through a pull request (no direct pushes, no force-pushes, no branch deletion), enforced for every user including admins. Review approval is not currently required — the repository is a solo effort right now; revisit `required_approving_review_count` on the `main protection` ruleset when that changes.
- Merge with **squash** only — `main` keeps one commit per PR. The merge commit title/body default to the PR's title/body, so keep PR titles and descriptions accurate; they become the permanent commit message. Merge commits and rebase-merges are disabled at the repository level.
- Branches are deleted automatically on merge (`delete_branch_on_merge`). Don't rely on a merged branch still existing on the remote.
- Merging into `main` is not a substitute for [document authority](#document-authority) — a PR that changes `docs/` still owes the review the hierarchy above implies, independent of the GitHub review count.

## Standing constraints

From `docs/altair-v0-implementation-plan.md`. These are the ones cheapest to violate and most expensive to repair, and they apply to prose changes as much as to code:

- The audience predicate lives **inside** the candidate query, on every path including similarity — never a filter afterwards. One predicate implementation, called by both paths.
- Nothing crosses from the read path to the write path. The read path writes nothing, including no record of what was asked.
- A stale base counter is never a rejection.
- Acceptance is shown only after local durability.
- Bytes before the record on creation. Record before the bytes on erasure.
- Relation types are declarations the system interprets, not branches.
- Derived data is never canonical; a person's edit to it outranks recomputation.
- Nothing may foreclose export (DR-001).
- Waiting is silent, faults signal. An unreachable instance and an expired session are the same wait. No counter that rises while the person is away; queue depth is never reported.
- No TUI habit becomes load-bearing — what crosses the boundary is the component model's list, not what the first client happened to need.

## Technology, as decided

Rust instance serving gRPC (tonic, with an Axum router for non-RPC HTTP); ratatui terminal client first; a message bridge for capture away from that surface; Android in Kotlin/Compose later; desktop toolkit deliberately undecided (DR-006). Authentik issues tokens, the instance validates them and reads a member claim — it never authenticates (DR-005). Every identifier is a random UUIDv4, generated by whatever brings the thing into existence, and nothing ever reads an identifier (DR-007).

## Writing conventions in `docs/`

Match the existing register — these documents have a deliberate and consistent style:

- Each opens with a `**Status:**` / `**Date:**` / `**Governed by:**` / `**Related:**` block, then a "What this document is" section stating what it decides and what it explicitly does not.
- Claims are bolded lead-ins followed by the reasoning. Rejected alternatives are recorded with why, so they are not re-argued.
- Mermaid diagrams **carry the same weight as the prose** — each states the same thing as the surrounding text rather than illustrating part of it. If prose changes, check the diagram beside it.
- Decisions are marked **one-way** (reversing it means rewriting dependants) or **reversible** (changeable behind a boundary). Where something is common industry practice rather than a judgement call, it says so.
- Documents end with "Deliberately not decided" and "Open questions". Move something out of "Open questions" only when it has actually been decided somewhere normative.
- Decision records are `DR-NNN-kebab-title.md` with Context / Decision / Alternatives considered / Consequences / Deliberately not decided here.
