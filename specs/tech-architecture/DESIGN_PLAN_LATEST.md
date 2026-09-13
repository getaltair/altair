---
type: interface-design
context: altair-v0.1-contextual-recall
---

# Contextual Recall Interface Design

## Problem

Every project, task, note, and resource editor needs live recall from current,
potentially unsaved input. The interface must make obsolete-response rejection,
bounded waits, and semantic degradation consistent without coupling domain
behavior to Vue, Axum, SQLx, PostgreSQL ranking details, or inference payloads.

## Options Compared

| Option | UX | Complexity | Extensibility | Performance |
|---|---|---|---|---|
| Stateless request/response | Correct only when every editor implements debounce and stale checks | Lowest server complexity; repeated client concurrency logic | Easy to call from new clients | Cancellation is caller-dependent |
| Latest-only editor session plus stateless server operation | Consistent waiting, refreshing, degraded, and stale-result behavior | One focused stateful client coordinator; simple server boundary | New editors submit the same domain snapshots | Cancels obsolete work while retaining revision checks |
| Progressive result stream | Can show lexical results before semantic results | Highest transport, ordering, and lifecycle complexity | Supports future incremental channels | Lowest first-result latency but risks movement under user actions |

## Decision

Use a latest-only frontend recall session over a stateless server recall
operation.

The frontend session accepts immutable domain-specific snapshots and owns the
quiet period, best-effort cancellation, monotonically increasing context
revision, monotonically increasing dispatch generation, context-scoped
dismissals, visibility pause/resume, and atomic result publication. The server
accepts one validated recall request and returns one immutable batch.
Cancellation saves resources; matching the session, context revision, and
dispatch generation remains the correctness mechanism.

The public result reports candidate identity, record kind, a label and detail
snapshot sourced from authoritative data, existing-link state, understandable
evidence, source record revision, and retrieval coverage. It does not expose
raw vector distances, SQL ranks, provider payloads, or mutation operations.
Every mutation command revalidates current record revision, archive state, and
relationship state rather than trusting the recall snapshot.

## Invariants

1. Unsaved content is recall input only and is never persisted by recall.
2. Only a response matching the current session, context revision, and latest dispatch generation can replace displayed results.
3. Cancellation is an optimization and never the stale-response correctness mechanism.
4. Candidates are saved records, unique by stable identity, and exclude the active and archived records by default.
5. Semantic failure returns available non-semantic results with explicit degraded coverage.
6. Recall is read-only; preview, open, link, edit, quantity, and task-status actions use separate commands.
7. Result limits and breadth affect retrieval; presentation controls remain separate.
8. Labels, excerpts, and details come from authoritative record data rather than generated assertions.
9. An explicitly included archived candidate is labeled archived; default recall excludes it.
10. While a pointer, keyboard, or touch action targets a suggestion, retain at most the newest completed batch. Publish it only after that interaction completes and its session, context revision, and dispatch generation are all still current.

## Privacy And Security

- Recall requires the authenticated application session and the same CSRF protections as other application requests.
- Application, proxy, telemetry, and inference logs must not record raw recall text or response excerpts.
- Recall request bodies and results are transient and are not placed in shared HTTP or server-response caches.
- Recall responses use `Cache-Control: no-store`.
- Axum sends content only to the explicitly configured inference endpoint; no provider fallback is permitted.
- Server-owned deadlines bound inference, and semantic timeout returns degraded non-semantic coverage.
- Deployment must configure the inference endpoint and intervening proxies not to retain request bodies.

## Lifecycle

The frontend state machine starts at `idle`. With automatic refresh enabled,
non-empty input creates a context revision and enters `waiting`; quiet-period
expiry creates a dispatch generation and enters `loading`. With automatic
refresh disabled, input invalidates the current generation and remains `idle`
with any prior batch retained as stale cache but removed from presentation when
its context key differs from the current context.
A manual refresh with non-empty context bypasses the quiet period, creates a
new dispatch generation, and enters `loading` from any visible, non-disposed
state. While paused, manual refresh records no request and resume reevaluates
the current context. An empty context enters `idle`, invalidates the generation,
and clears candidates.
A successful latest request enters `ready`, including an explicitly empty
complete batch. New input from `waiting`, `loading`, `ready`, or `unavailable`
invalidates prior generations and follows the enabled or disabled automatic
refresh rule. Transport failure enters `unavailable` without discarding the
editor draft; retry creates a new generation and enters `loading`.
Semantic-only failure enters `ready` with degraded coverage.

Page hiding enters `paused`, invalidates the dispatch generation, cancels
current work, and retains the current published batch without refreshing it.
Foreground resume reads the latest snapshot and enters `waiting` only when
automatic refresh is enabled and context is non-empty; otherwise it enters
`idle`. A retained batch with a different context key remains hidden as stale;
the user must request fresh results. Resume never revives an old timer.
Disposing invalidates the generation, enters terminal `disposed`, and prevents
all later publication.
Automatic presentation can be hidden while manual refresh remains available.
Dismissals use the deterministic context key defined in `tech-stack.md`, so
restoring an unchanged normalized context does not immediately revive a
dismissed candidate.

## Consequences

- CodeMirror and ordinary forms share one concurrency-safe recall coordinator.
- Android foreground resume submits the current snapshot instead of reviving a throttled timer.
- The initial HTTP contract stays single-request/single-response; streaming can be reconsidered only if measured latency requires it.
- The boundary can be tested with deterministic fake gateways and a stateless fake server recall implementation.
- Required client tests cover out-of-order responses, same-context refresh races, cancellation, disposal, empty context, retry, automatic refresh disabled, semantic timeout degradation, manual recall with automatic presentation hidden, latest-generation interaction-held publication, Android pause/resume, deterministic dismissal stability, and draft preservation.
- Required server and deployment checks cover authentication, CSRF rejection, `Cache-Control: no-store`, payload-safe logging, configured-provider-only routing, proxy and inference non-retention, server-owned timeouts, and mutation revalidation.

Before real content reaches inference, verification writes
`specs/verifications/INFERENCE_PRIVACY_LATEST.md` with the configured endpoint
owner, transport mode, retention-disabled setting or self-hosted logging policy,
and evidence from a canary request that application, proxy, and inference logs
omit request text. The check also proves there is no fallback endpoint and that
recall responses use `Cache-Control: no-store`; it records no credentials or
personal payloads.

The rationale is recorded in `specs/adr/0001-latest-only-contextual-recall.md`.

## Planning Boundary

This document fixes the module boundary, ownership, and safety invariants. The
epic plans must define executable transition tables and tests for attachment
upload/replacement/deletion, editing while a draft conflict is active, and
freezing a displayed recall target throughout an active interaction. Those are
story-level acceptance details, not additional architecture decisions.
