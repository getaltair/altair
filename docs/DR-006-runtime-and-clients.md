# DR-006: Rust for the instance, a terminal client first, Android in Kotlin later

**Status:** Accepted
**Date:** 2026-08-15
**Supersedes:** nothing
**Related:** Altair v0 Scope, Altair Architecture Foundations, Altair Component Model, DR-002, DR-004, DR-005

---

## Context

Every choice before this one was decided against the documents. This one is not fully decidable that way, and pretending otherwise would be dishonest. The instance is behavioural work that is already specified; the surfaces are where the undetermined work lives, and the project is built by one person, unpaid, over years. What that person will still open in three years is therefore a requirement rather than a preference, and the most probable failure mode is not a wrong protocol but an abandoned project.

**Three earlier decisions narrowed this more than any argument about languages.** DR-004 made the boundary a language-neutral contract, so a client in any language is generated rather than hand-written and the instance's language stops constraining what clients can exist. DR-002 put retrieval in SQL, so the part of the read path expected to churn continuously is query text rather than compiled logic. And the amendment placing block division at the instance removed nearly all shared logic between instance and client, leaving only the contract.

Together those mean the instance runtime and the client toolkits are independent choices, and none of them is a one-way door except by the cost of rewriting a surface.

---

## Decision

**Rust for the instance. A terminal client first. A message bridge for capture. Android in Kotlin and Compose when it comes. The desktop toolkit stays open.**

1. **The instance is Rust**, serving gRPC with tonic, which sits on hyper and tower and carries an Axum-based router for the ordinary HTTP endpoints that are not RPC.
2. **The first client is a terminal application** built with ratatui, and it is the deliberate surface: where bodies are written and where the work that is not capture happens.
3. **Capture away from that surface is a message bridge**, an ordinary client of the public interface with the three obligations the scope states: it accepts only from the person it captures for, it tracks its own position and treats a gap as a fault, and it does not answer queries.
4. **Android is Kotlin and Compose**, later, and it is what closes the lookup gap v0 leaves open. No part of this decision is arranged to make that cheaper than it would otherwise be, because there is nothing worth sharing across that boundary except the contract, which is already shared.
5. **The desktop toolkit is not decided.** Iced and Dioxus and Slint each answer a different question and the terminal client may make the question smaller. Deciding it now would be choosing before the information exists.
6. **A conformance suite covers the outbox rather than an implementation covering it.** The outbox will exist twice, in Rust and in Kotlin, and its behaviour is already fully specified: durable across restart, ordered per entity, idempotent, non-blocking, and silent. The scenarios are written once and run against both: accept while offline, survive a kill, replay in order, replay after an expired token, replay an intent the instance already holds.

---

## Why Rust, stated honestly

**The strongest argument is the least technical.** It is what the author has been writing most recently, and it is the language he expects to still want to open. For a project whose founding principle is that returning after an absence costs nothing, the maintainer's own re-entry cost is a first-class requirement, and Cargo does not have the problem that argued against the JVM build story.

**The technical arguments are real but secondary.** The terminal ecosystem is strongest in Rust, so the client that answers the open question is cheapest in the language the instance is already in. The desktop options worth investigating are Rust ones. And the objection that Rust punishes an empirical, constantly changing read path is much weaker here than it would normally be, because DR-002 put that churn in SQL.

---

## Alternatives considered

### TypeScript on Bun, with a Svelte client

**Rejected, having been recommended first and at length.** The case for it was strong and remains coherent: Lattice already runs Elysia and Svelte on Bun in daily use, which is the only demonstrated evidence of a stack this author maintains unpaid, and one language would have covered instance and client.

It is rejected because the author is tired of building web applications, and that is a requirement rather than a mood. The largest risk to this project is that it is not finished, and a stack that costs willpower to return to acts directly on that risk. Every technical argument in its favour survives; none of them outweighs this.

### Kotlin, with Ktor and Compose Multiplatform

**Rejected, and it was the closest alternative.** It offered one language across instance, Android, desktop, and eventually Wear, with Compose for Web possibly restoring a browser client and Mosaic possibly covering the terminal.

Three things decided against it. It would have meant standing on Compose desktop, Compose for Web, and kotlinx-rpc's gRPC support at once, which are respectively distrusted by the author, beta, and a development preview whose APIs may break without notice. Gradle is a cost paid on every return rather than once at setup, which is the specific shape of cost this project exists to refuse. And the one-language benefit shrank to almost nothing once block division moved to the instance, since what remained shared was the contract, which protobuf already carries across languages.

### Elixir and Phoenix

**Rejected, and interesting enough to record why.** OTP supervision is the closest expression in any runtime of the foundations' rule that the thing which changes constantly must not break the thing that must never fail. But this design has already moved its hardest guarantees outside the runtime's reach: capture succeeds with nothing reachable, acceptance is local and durable, and derivation is a separate component that cannot gate acceptance. What BEAM is best at is keeping the instance up, which is the property this architecture spent its design effort making unnecessary. Phoenix's centre of gravity is also live and pushed experience, which the vision declines by name.

### One toolkit for every client

**Rejected as a question rather than an answer.** Clients share nothing but the contract, so the toolkit is a per-client choice. Choosing one for all of them would mean using at least one of them where it is weakest.

---

## Consequences

**Gained**

- The instance is written in the language most likely to still be maintained in three years, which is the risk that dominates
- The terminal client is the cheapest possible way to exercise the whole instance end to end, and it answers a question the author actually has
- Phone capture exists from the first release without an Android client, so the Compose decision carries no urgency
- Each client can use the toolkit that suits it

**Given up**

- The outbox is implemented twice, in Rust and in Kotlin
- No browser client, and therefore no reaching the instance from a device with nothing installed
- No lookup from a phone until Android exists
- Two toolchains once Android arrives, buying nothing in shared code

**Obligations this creates**

- **The outbox conformance scenarios are written before the second implementation exists**, not after it disagrees with the first.
- **The bridge's three obligations are behaviour, not configuration.** A bridge that accepts from anyone, or that skips a message silently, breaks a guarantee the rest of the system keeps.
- **Nothing about the terminal client may become the definition of a client.** The component model states what crosses the boundary, and a second client arriving must not discover that the first one's habits were load-bearing.

---

## Deliberately not decided here

- The desktop GUI toolkit
- When Android is built, beyond it being what closes the lookup gap
- Whether the terminal client remains the deliberate surface after it has been used for a while, which is the question it exists to answer
- Whether a browser client ever returns, which grpc-web makes cheap if it does
