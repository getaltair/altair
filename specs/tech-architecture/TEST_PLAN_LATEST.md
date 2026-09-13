---
type: test-plan
context: altair-v0.1
---

# Test Plan

Use Rust and TypeScript unit tests for domain transitions, integration tests for
PostgreSQL, Garage, Authentik, and inference boundaries, Vue Test Utils for
component behavior, and Playwright for critical desktop and Android workflows.

P0 coverage includes authentication, revision conflicts, atomic create-and-link,
archive association races, attachment replacement/deletion, indexing durability,
stale recall responses, and backup/restore integrity. Each task ledger starts
failing and names its runnable `just` gate. `just preflight` runs tests, lint,
and production builds before forward work proceeds.

The release UAT must exercise the five scenarios in `docs/initial-build.md`,
managed-device draft recovery disablement, degraded semantic inference, offline
save failure, cross-device conflict, and touch workflows at 320x568 and 360x800.
