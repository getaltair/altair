# DR-004: Protocol buffers over gRPC are the wire contract

**Status:** Accepted
**Date:** 2026-08-15
**Supersedes:** nothing
**Related:** Altair Component Model, Altair Architecture Foundations, Altair Substrate Specification, Altair Data Model, Altair v0 Scope, DR-002

---

## Context

The component model states that the public interface attaches at the client boundary of the instance and nowhere else, and enumerates what crosses it in both directions. Naming the boundary is what made deferring its definition safe. This record defines it.

**The surface is small and is not resource-shaped.** Inbound: intents, which are create, edit, remove, erase, and restore; queries with their scope; a request for what changed since a stated position; a request for instance health. Outbound: acknowledgement of an accepted intent; results carrying enough to be accounted for; a change set assembled for one member; a statement that a stated position can no longer be answered; health, read only.

**This decision is taken before the runtime deliberately.** A language-neutral contract means a client in any language is generated rather than hand-written, which removes the instance's language from the question of what clients can exist. Taking the runtime first would have decided that by accident.

---

## Constraints the documents already impose

These are not choices. They are what the wire must carry to satisfy commitments already made.

- **Replay is idempotent.** A capture is never lost and never dropped, and the outbox retries. An intent therefore carries its own identity, distinct from the entity's, and resubmitting one returns the original acknowledgement rather than producing a second effect.
- **A batch expresses partial success.** A bulk capture is explicitly not one unit: two hundred files where the hundredth fails leaves ninety-nine captured. A batch returns a result per item and is never all or nothing.
- **Refusal on audience and refusal on nonexistence are indistinguishable.** One response shape covers both, and nothing in the wire, including status codes, distinguishes them.
- **A position past the horizon is an outcome, not an error.** The client's correct response is to rebuild, and the response says so plainly enough that no client treats it as a failure to retry.
- **Unavailable and refused are distinguishable.** Waiting is silent and a fault signals, and the client cannot honour that distinction if a refusal and a dead connection look alike.
- **A gateway challenge is a wait, not a response.** Access control is the operator's gateway, so an expired session may answer a replay with a redirect or a login page. A client must recognise that as the same wait as an unreachable instance, and never as a refusal and never as data.

---

## Decision

**Protocol buffers as the schema and the encoding, carried by gRPC over HTTP/2.**

1. **Binary on the wire, JSON available from the same schema.** One contract, two encodings. Binary is the default; the JSON encoding exists so that a capture can be inspected by hand when something has gone wrong, which for a single maintainer is a real operational need rather than a nicety.
2. **gRPC as the transport.** It is the standard carrier for protobuf, it is first-party in both Rust and Kotlin, and every outcome the boundary needs is a status code rather than something a client infers. Nothing sits between a client and the instance that has to understand it, because authentication is a validated token rather than a forward-auth proxy, per DR-005.
3. **The schema is the client contract, not the internal model.** Generated types describe what crosses the boundary. The write path maps them into internal types and validates there, per DR-002. Protobuf carries shape; it does not carry a type fixed at creation, properties permitted only on relation types that declare them, or audience rules.
4. **The change stream is polled with a position.** No per-client cursor is held by the instance, a client reports its own position, and a client that never returns costs nothing. Pushed delivery is an optimisation that adds a connection lifecycle to the path that must never lose a capture, and it is not taken in v0.
5. **Field numbers are permanent.** Removed fields are reserved, never reused. This is what makes an intent authored months ago by an older client decode correctly today, which is a durability guarantee already committed to rather than a protocol nicety.

### What an edit to a body submits

**The whole body, with its base counter.** The instance divides the incoming text, matches the recomputed boundaries against the blocks it holds, and writes only what changed. A client never runs the division rule, and the write still addresses a part rather than a record, because the part is derived where the write is applied.

The cost is sending an unchanged body along with a small edit, which at the size of a note is not worth optimising against. What it buys is that the division rule has exactly one implementation, which is what keeps devices from disagreeing about the units reconciliation is decided in.

### What a change set carries

**The identities of the blocks that changed**, not merely that the body moved. The instance has already divided, so those identities are facts it holds and can report.

This is what allows a client to detect a conflict before the instance does, which the foundations permit and which resolving locally makes disappear. A conflict is a stale base, the same part, and a different value. For an ordinary field a client compares field to field and is exact. For a body it compares block identities against the parts its own unsent edit touches, and is equally exact, without holding the division rule.

---

## Alternatives considered

### JSON over HTTP with a hand-written or generated schema

**Rejected, and it was the starting assumption.** It loses two things that are guarantees here rather than preferences. An intent that has waited months in an outbox must still decode against a moved instance, which field numbering makes structural and JSON leaves to convention and vigilance. And the outbox carries bytes, since a photo is a capture like any other, which JSON encodes as base64 at a third again the size on exactly the path where signal is worst.

### tRPC, or the equivalent within a single framework

**Rejected.** There is no schema artifact by design; the contract is inferred TypeScript shared between a client and server that ship together. Two things fail. Nothing exists for a Kotlin or Swift client to generate from, so the choice would quietly decide the instance's language forever. And the outbox assumes client and instance do not ship together, which is the one assumption the approach is built on.

### GraphQL

**Rejected.** It moves query shaping to the client, and the read path must control candidate generation with the audience predicate inside the query that produces candidates. A client composing its own traversal is a client asking a question the predicate was not applied to.

### REST with a resource per entity type

**Rejected.** It multiplies the surface per type and per domain in a substrate whose point is that types are uniform and anything relates to anything. The interface is five verbs over one model, and resource modelling would obscure that rather than express it.

### The Connect protocol

**Rejected, having been chosen first.** Its advantages were that it runs over ordinary HTTP POST with no proxy needed for a browser client, and that it keeps an ordinary request shape through a forward-auth gateway. Both premises are gone: there is no browser client in v0, and DR-005 replaced forward-auth with token validation, so nothing in the path needs to interpret the protocol. What remains is that gRPC is better served in both languages that will implement this boundary.

**A browser client returns cheaply if it returns at all.** grpc-web is served by a layer on the same server rather than a separate proxy, and its restriction to unary and server-streaming calls costs nothing here, since the change stream is polled and every other call is unary.

---

## Consequences

**Gained**

- A client in any language is generated from the contract, so native clients cost their platform work and no contract work
- Old intents decode against newer instances by construction rather than by care
- File bodies cross the boundary as bytes
- The interface definition is now a document rather than an obligation, and nothing in v0 forecloses it

**Given up**

- A build step, and the loss of reading a request by eye without decoding it. The JSON encoding recovers the second at the cost of remembering it exists.
- Field numbers become permanent commitments, which is cheap to honour continuously and expensive to repair afterwards.

**Obligations this creates**

- **The interchange format for export is a separate artifact.** It has different readers and different goals, and it stays human-inspectable rather than becoming this schema in another costume.
- **Nothing may reach the instance except through this contract.** The operator plane's capabilities included, per the component model.
- **A client must treat a gateway challenge as a wait.** This is the concrete form of the rule that an unreachable instance and an expired session are the same wait, and it is the failure most likely to be discovered by captures vanishing into a login page.

---

## Deliberately not decided here

- The runtime and language of the instance, which this record is intended to leave open
- Whether the change stream later gains a pushed transport, which is an optimisation over polling rather than a replacement for it
- The shape of query scoping beyond its presence, which belongs with retrieval design
