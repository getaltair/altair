# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repository is right now

**Design documents, the instance's write and read paths, and a terminal client that runs.** Everything in `docs/` specifies a self-hosted personal system ("Altair"); `crates/` is the instance and the terminal client. Wave 0, all five Wave 1 lanes, Waves 2.1 through 2.3, all three Wave 3 lanes and all six stages of Wave 4 have landed — store bootstrap and the audience predicate, block division, the object store, token validation, the outbox conformance suite, the intent spine, type content across all three domains, file bodies, literal retrieval, the change stream, health, and the client's device store, outbox, replica, shell and thirteen screens. **Wave 2.4 — reclamation, the retention windows, the horizon value — was skipped and is still outstanding**, and Waves 3 and 4 proceeded without it. All six calls in the interface are served, and `altair` runs. **Wave 5, semantic retrieval, is next, and is re-planned at this boundary before anything is built.**

Consequences for working here:

- **Changing a document is the work.** Treat prose with the care normally reserved for code: these documents are the authority the implementation is checked against.
- **Verify code changes by running the suite** — see [Commands](#commands). Do not claim to have verified behaviour you did not run.
- `README.md` is the human-facing summary of much of this file. If one changes, check the other.

## Commands

```bash
mise install                 # toolchain: rust, prek, mado. Docker is the only other prerequisite
cp .env.example .env         # DATABASE_URL, read by the test harness only
mise run test                # docker compose up -d --wait, then cargo test --workspace
mise run conformance         # the outbox scenarios, against the real client — a real gate now
prek run --all-files         # rustfmt, clippy -D warnings, mado, hygiene. CI runs these same hooks
```

- **No `protoc` is needed.** Codegen goes through `protox`, which is pure Rust.
- **Tests need a real PostgreSQL**, never a mock: the audience predicate, both indexes, and `SKIP LOCKED` are Postgres behaviour. Migrations are applied once into a template database and each test branches it with `CREATE DATABASE … TEMPLATE`.
- **Set `ALTAIR_TEST_PREFIX` per worktree** when lanes run in parallel against one Postgres, or they fight over the template name.

## Two guards that will deny your tool calls

Both are `PreToolUse` hooks in `.claude/settings.json`, both deny rather than ask, and both are unlocked out-of-band by a human for 30 minutes.

- **`docs/` is gated.** Any create, edit, move, or delete under `docs/`, by any means including shell redirection, `sed -i`, or `git restore`, is refused. Reading is always fine. To proceed, ask the user to run `touch .claude/.docs-unlock` — do not attempt a workaround, and do not create the marker yourself; the guard defends it and itself.
- **Pre-commit hooks may not be skipped.** `--no-verify`, `-n`, `SKIP=`, and `core.hooksPath` are refused. A merge commit made with `--no-verify` once shipped a `Cargo.lock` matching neither branch, and CI failed two pushes later under an unrelated hook's name. The human unlock is `touch .claude/.hooks-unlock`.

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
  │           └── altair-wave-4-plan.md   the next wave only; sequencing and
  │                                       verification, deliberately no module tree
  └── DR-001 … DR-007                     product/technology choices made against the above
```

Rules that follow from this and are easy to violate:

- **Assembled documents decide nothing.** `altair-data-model.md` and `altair-component-model.md` restate what their authorities require. Where one disagrees with the substrate spec, the domain PRDs, or the architecture, **the assembled document is wrong** and gets corrected — never the other way around.
- **`docs/altair-scratchpad.md` has no authority.** Entries are `open` / `leaning` / `parked`, each with a destination. Writing something there is not deciding it. Its `Rejected` section exists so "why not just do X?" has an answer without re-arguing.
- **The vision's Must / Should / Won't are product identity, not a backlog.** v0 narrowing a Must is temporary and must be recorded under "Deferred Musts" in `altair-v0-scope.md`; it does not amend the vision.
- **Design before architecture before tools.** Specs say what must be true; the architecture says what structure delivers it; DRs choose products. A spec that names a product, or an architecture document that picks a database, has escaped its layer.
- **Scope discipline.** The substrate holds no domain behaviour; if a substrate section starts describing what a quest does, it has escaped. Domain PRDs hold no mechanism.

## Documents that are already accepted as artefacts

- `crates/altaird/migrations/0001_initial.sql` — the structured store, adopted as **migration one**. Applied and exercised on PostgreSQL 18.6 with pgvector 0.8.6. Do not re-derive it.
- `crates/altaird/migrations/0002_write_provenance.sql` — **migration two**, taken at the start of 2.1. `entity_part_counter` holds, per part, the counter it last moved at and the member who moved it, because conflict detection asks which parts moved between two counter values and nothing else could answer it. A relation gains a lifecycle, the time it was removed, and the act that removed it. Both shapes rejected for the first are recorded in the file.
- `proto/altair/v1/altair.proto` — the public interface (`package altair.v1`). **Field numbers are permanent; removed fields are reserved, never reused.** Closing one of its recorded gaps means adding a field, not editing one.
- `conformance/scenarios.md` — the outbox specification made executable, written once and run against every implementation (Rust and Kotlin). Scenarios observe behaviour at two boundaries only: what the person sees, and what reaches the instance.

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

What has actually been built so far, which is observation rather than a settled tree:

```text
crates/altaird/            the instance: store/ (audience, tx, entity, relation, search, health,
                           ids), body/, objects/, auth/, read/ (changes),
                           write/ (parts, provenance, changes, intent, entity, relation, body,
                           specific/ one module per domain), service.rs (all six calls served)
crates/altaird/migrations/ 0001_initial.sql and 0002_write_provenance.sql, not re-derived
crates/altair-proto/       generated contract types (protox + tonic)
crates/altair-conformance/ the outbox harness, the fake instance, and a null client stub
crates/altair-tui/         the terminal client: device.rs (SQLite, migrated), wire.rs,
                           sender.rs (the outbox), replica.rs, session.rs, signals.rs,
                           editor.rs, config.rs, ui/ (theme, glyphs, chrome, help, view,
                           screens/), app/ (the loop and compose), adapter.rs (the
                           conformance mode), bin `altair`
```

**The conformance suite was red on purpose and is now a real gate.** Wave 4.1 wrote the outbox it judges; thirty-four scenarios pass, A3 is skipped for a recorded reason, and its CI job is no longer `continue-on-error`. It still sits behind the `run-conformance` feature, because it launches a client process per scenario and drives it for seconds at a time. Read `crates/altair-conformance/README.md` before touching it — in particular before deciding a skip is a gap.

Seven waves. Waves 1–3 are planned against reality; Wave 5 onward names outcomes and deliberately leaves shape open. **Re-plan at each wave boundary, and only the next wave.**

| Wave | Produces | Notes |
|---|---|---|
| 0 · Plumbing | Cargo workspace, proto codegen in the build, migration runner (`0001_initial.sql` as migration one), integration harness, CI | Harness stands up a **real PostgreSQL**, not a mock — the audience predicate, both indexes and `SKIP LOCKED` are Postgres behaviour. Done when a fresh checkout runs one command and passes an empty suite. |
| 1 · Foundations | 1.1 store bootstrap · 1.2 block division · 1.3 object store · 1.4 token validation · 1.5 outbox conformance suite | Five genuinely independent lanes, one worktree each. 1.5's deliverable is a **red suite** — every scenario runs and fails. |
| 2 · Write path | 2.1 intent spine, **including relations and the served submission call** — landed · 2.2 type content, all three domains — landed · 2.3 file bodies — landed · 2.4 reclamation — **not started** | 2.1 is sequential and load-bearing; do not parallelise the spine, do parallelise what hangs off it. Re-planned at the Wave 1 boundary: **migration two** opens 2.1, adding per-part write provenance and a lifecycle on a relation. |
| 3 · Read path | 3.1 literal retrieval arm · 3.2 change stream and horizon · 3.3 health — all landed | Literal only. The six served calls include **no way to fetch an entity by identity and no way to list what a container holds**: `Query` is the literal arm, it cannot enumerate, and it answers with entities and never relations. The change stream is the only source of either. |
| 4 · Terminal client | **Landed, all six stages.** 4.1 the device store and outbox (the suite is green) · 4.2 the canary, the replica, the shell, thirteen screens, and a running client | **The first useful day.** The TUI carries the whole editing surface; there is no browser and no second client. `altair-wave-4-plan.md` sequenced it in six stages; 4.1 is stages one and two. |
| 5 · Semantic | derivation worker · inference boundary and bi-encoder · semantic arm and fusion | The embedding model is chosen **here**, against a real corpus, and fixes the schema dimension. |
| 6 · Second client and ops | message bridge · packaging · backup/restore/upgrade | The bridge is the first test of whether the interface carries the obligations rather than the TUI's code. |

**Two lanes cross wave boundaries and should not wait:** the terminal client shell can run against a fake instance from Wave 1, and the conformance suite needs nothing but the generated contract.

Specifics worth knowing before touching the relevant lane:

- **The audience predicate is written once**, in the store bootstrap layer, and both paths call it. A test asserts it appears in exactly one place.
- **Block division is a pure function over text**, run only at the instance (DR-004). Atomic constructs never split (fenced code, tables, diagrams); list items do split; identity survives edits to a block and to its neighbours. A long unbroken stretch of prose being one block is correct, not a bug.
- **The intent spine**: idempotent replay returns the original acknowledgement, the intent row is written in the transaction it acknowledges, a stale base is never a rejection (reject-and-retry is the familiar pattern and it is wrong), both values are retained on a same-part conflict, and refusal on audience is indistinguishable from refusal on nonexistence.
- **The change sequence allocates from a single position row in the write transaction, so every write serialises on it. That is intended** — a sequence leaves gaps and a poller can read past an uncommitted position and never see it. Record why prominently near the code; this is exactly the shape of thing a future reader removes as an obvious bottleneck.
- **`crates/altaird/migrations/0001_initial.sql` already contains Wave 2's test plan**, under *"Checks this file cannot make, which the write path owes"* (line 886). Turn that list into the suite. The plan document still calls this file `altair-schema.sql`; the name changed when it was adopted as migration one. **The list spans the whole wave rather than 2.1 alone** — the plan says which item closes which lines.
- **The schema's `ON DELETE CASCADE`s never fire under erasure**, because every one hangs off a delete of the `entity` row and erasure leaves a tombstone. The erase path removes blocks, dates, assignments, property values, side-table rows, event records, embeddings, derived text, and the relations at either end explicitly.
- **Two schema gaps were named for Wave 0 and neither closed there, correctly.** The embedding dimension stays a placeholder until Wave 5 chooses the model, which is its own trigger. Cycle prevention in nested locations and categories was a write-path check owned by 2.2 and landed there, in `write/specific/nesting.rs`.
- **Outstanding derivation is computed from provenance in the store, never from the queue.** Losing the queue costs time, not reportability.
- **The read path cannot write, and this is enforced rather than reviewed.** `store::begin_read` makes a genuine `INSERT`/`UPDATE`/`DELETE` from inside `read/` a database error. The other half of the constraint — no record of what was asked — is the module's own to keep.
- **A relation reaches the change stream only when the member can see both of its endpoints**, via `relation::endpoints_visible`. The row-level filter in the page query covers entities; relations are filtered after loading, and dropping that check is a leak the SQL will not catch.
- **The horizon is null, or longer than every other retention window. A middle value is a bug**, not a tuning choice. Nothing implements one yet: 2.4 did not land, so there is no retention constant anywhere and nothing trims the change sequence — which is the only reason a client can still rebuild from position zero.

**Deferred decisions have triggers, not dates** — the table at the end of the plan names the moment each becomes cheap to take correctly (embedding model at Wave 5, bridge transport at 6.1, retention constants at 2.4, the surrogate key on `entity` only when insert cost tracks table size). Do not take one early.

### Workflow the plan assumes

Built with Claude Code + hyperskills. The plan maps situations to skills: `plan` at a wave boundary (next wave only), `orchestrate` with one worktree per lane for a wave with 3+ parallel lanes, `cross-model-review` before merging anything touching the write path, conflict detection, or audience, `brainstorm` then `research` when a deferred decision hits its trigger, `implement` for everything else.

At every wave boundary ask: what did this wave teach that the next wave's plan does not know, which deferred decision just hit its trigger, and has the shape drifted from the component model? The third catches sprawl that green tests do not.

## What Wave 4.1 settled, for the lanes that follow

Observations from building the device store and the outbox that 4.2, and the Kotlin outbox after it, would otherwise re-derive.

- **The client's write transactions are `IMMEDIATE`, and that word is load bearing.** The sender and the person's surface hold a SQLite connection each. A deferred transaction that reads and then writes cannot take its write lock under a write-ahead log if another connection wrote in between — SQLite answers `SQLITE_BUSY` *without* honouring the busy timeout, because waiting could not help. It showed up as a capture refused for "database is locked" while the sender happened to be recording an acknowledgement, which is acceptance failing for a reason acceptance is not allowed to fail for. `Store::write` is the one place a write begins.
- **A wait is told from a fault by `Status::source()`, and this was measured rather than reasoned.** A connection torn down mid-call and an answer the client cannot classify both arrive as gRPC code `Unknown`, so the code cannot be the discriminator. A status tonic synthesised from a transport failure carries the underlying error as its source; one the instance actually sent carries none. That is a property of a library, not of the contract, and nothing would fail loudly if it changed — so `crates/altair-tui/tests/wire_conditions.rs` asserts it against the fake instance.
- **Taking at most one outstanding intent per entity per pass is what makes the ordering rules hold**, and there is no other sequencing anywhere. A create reaches the instance before its own edits, an edit carries the counter the write before it was acknowledged with, and nothing is required about where an unrelated entity's intents fall — all three fall out of that one choice in `Store::next_to_send`.
- **The one number the client states is counted from the store, not held in memory.** How many the instance refused is a fact about what is in the outbox, so it survives a restart and cannot drift from it. Nothing anywhere counts what is waiting.
- **A3 is skipped and it is not a gap.** Its condition is a device that has never been signed in, and every client the harness launches is handed a token in its environment; the terminal client binds from that token, so the condition cannot arise. A client whose binding is a separate act — the Android one — declares `unbound_capture` and runs it. Deciding a skip is a gap and "fixing" it is how a green suite stops meaning anything.
- **The suite finds the client binary by path.** Cargo only sets `CARGO_BIN_EXE_*` for binaries of the package under test, and the client is deliberately a different package — it depends on the generated contract and not on the harness. `mise run conformance` builds it first; a scenario failing on a missing binary means that step was skipped.
- **The conformance adapter is a mode of `altair`, behind the `conformance` feature.** The feature exists so the shipped client does not link the harness that judges it. What it drives — the store, the sender, the signals — is not behind a feature and is exactly what the screens will use.

## What Wave 4.2 settled, for the waves that follow

- **A terminal is a surface nobody can review from a diff**, so a snapshot records three things: the palette, the grid, and a colour key naming the token behind every run. Each catches what the others cannot — the grid alone goes green on a theme rendered entirely in one colour, and the key alone goes green when a token keeps its name and changes its value, which is what editing a palette does. Both failure modes were induced and watched before the snapshots were trusted.
- **Snapshots cannot see what running it sees.** Three defects came out of driving the real binary in a pty and none of them could have failed a snapshot: framing a screen subtracted the chrome's four lines from a terminal of height zero and panicked; the faults screen listed everything unsent under a heading counting it, inverting *waiting is silent* and printing the one forbidden number; and every row wore a "not sent" marker, which is an indication of how many as soon as there is more than one. **Run it before believing it.**
- **The device store has migrations and the instance's reasoning does not apply.** `CREATE TABLE IF NOT EXISTS` builds a new store correctly and does nothing to an existing one, so a person who updated the client would find their unsent captures unreadable. Additive and forward only is enough: what is on a device is either an outbox, which is never restructured, or a copy of the instance's, which can be thrown away and read again.
- **The overlay is one rule and it is worth stating exactly.** The local view outranks the instance's *only* while something local has not got there yet. With nothing pending the instance's copy is the newer one and wins, which is what makes somebody else's edit arrive at all.
- **The convergence property test catches paging-sensitivity and nothing else.** Both sides run the same code, so an order-independent bug passes it — and its first version passed over two index tables that were never written, because the edit meant to maintain them silently did not apply. The projection tests are the other half.
- **Two glyph sets, and the hazard is checked mechanically.** A character is Ambiguous when `width` and `width_cjk` disagree, which is the definition and therefore the test. One check asserts the *signature* set still carries the hazard: the day it does not, the narrow-safe set has stopped earning its place and should go rather than linger.
- **The help surface is an assembled document.** It decides nothing and is checked for the shapes that would make it a lie — a number it may not state, a promise the client does not keep. It no longer claims to keep a buffer as the person types, because composition is a handoff now.

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

## What Wave 2.1 settled, for the lanes that follow

Observations from building the intent spine that 2.2, 2.3, and 2.4 would otherwise re-derive.

- **A part is named in one place.** `crates/altaird/src/write/parts.rs` holds the wire field number, the store's text name, and the conflict row's spelling, and `tests/write_parts.rs` keeps them in step. Adding a type-specific part in 2.2 means adding it there, not beside the code that writes it.
- **Type content is applied, as of 2.2.** 2.1 created each type's side-table row with defaults and read `content.specific` for the type tag alone; `write/specific/` now writes guidance, knowledge, tracking and categories. What still refuses is a type v0 defines and never writes — routines, focus sessions, check-ins — and it refuses distinguishably, with `tests/unwritten_tables.rs` saying which.
- **The three refusals 2.1 named have all expired.** A file create landed at 2.3, an anchor on a relation at 2.2, and an explicit `category_position` is now accepted in `write/content.rs`. Kept here because the shape is the useful part: a refusal that waits on a named lane, carrying a detail saying which.
- **Erasure's dependent-table list is a constant**, `DEPENDENT_TABLES` in `write/entity.rs`. A new table holding entity content is one line there and one line in `tests/write_lifecycle.rs`, and forgetting is silent because the schema's cascades never fire.
- **The guards are blunt on purpose and were sharpened twice here.** `WritePath::new` tripped the object-store boundary on `Path::`, and the wire's audience field tripped the one-predicate column check. Both now match at a word boundary, and the two whole-tree checks that reason about the instance skip test sources — a test asserting that both paths call one predicate has to be able to read past it. Watch a guard fail on a deliberate violation before and after touching one.
