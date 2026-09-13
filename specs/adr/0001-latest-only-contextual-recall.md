---
type: architecture-decision
context: altair-v0.1-contextual-recall
status: accepted
---

# Latest-Only Contextual Recall

Use a latest-only frontend recall session over a stateless server recall
operation. Centralizing quiet-period timing, cancellation, context revisions,
dismissal scope, browser lifecycle, and publication stability prevents each
editor from reimplementing race-sensitive behavior. The server remains a
single-request/single-response boundary so retrieval infrastructure stays
replaceable and semantic failure can return honest degraded coverage without
streaming complexity.

Alternatives were a stateless interface that makes every editor own concurrency
correctness, and a progressive stream that can return lexical results earlier
but adds transport and interaction-ordering complexity. The selected boundary
adds one stateful client coordinator while keeping server retrieval stateless.
Its detailed invariants, lifecycle, and verification obligations live in
`specs/tech-architecture/DESIGN_PLAN_LATEST.md`.
