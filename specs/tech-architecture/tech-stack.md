# Technical Stack

`docs/initial-build.md` is the source of truth for the accepted technical foundation.

| Area | Selection |
|---|---|
| Client | Vue 3, TypeScript, Vite, Pinia |
| Frontend tooling | Bun for package management and frontend scripts |
| Component system | Naive UI |
| Icons | XIcons with `@vicons/ionicons5` |
| Editor | CodeMirror 6 with Vim support |
| Server | Rust and Axum |
| Persistence | SQLx and PostgreSQL 18 |
| Retrieval | PostgreSQL full-text search, pg_trgm, and pgvector |
| Identity | Authentik through OIDC authorization code flow with PKCE |
| Attachments | Garage S3-compatible storage |
| Inference | Configured OpenAI-compatible `/v1/embeddings` endpoint |

Organize code by product domain. Keep external integrations behind focused infrastructure modules.

Bun, Naive UI, and XIcons Ionicons 5 are explicit implementation constraints
for this build. They remain replaceable through an accepted technical decision.
Add frontend dependencies with Bun when the client is scaffolded; do not
hand-edit a lockfile.

## Domain Invariants

- A record is either active or archived until an explicit final deletion.
- Archived records are excluded from default domain lists, search, and recall.
- Archived records remain available through an explicit archived view and can
  be restored without changing their stable identity or relationships.
- Archiving a project never changes task state silently. The archive action
  defaults to the project alone and may optionally include tasks associated
  only with that project. Tasks shared with another project never cascade.
- A task has one status even when it is associated with multiple projects.
- Project association is distinct from a general related-record link and is the
  only relationship that controls project-board membership.
- A relationship reuses both endpoint records and is unique for the endpoint
  pair and relationship kind; unlinking never removes either endpoint.
- Missing resource quantity means unknown. Incrementing an unknown quantity is
  invalid until the user supplies a known total.

## Record Lifecycle

`active -> archived -> active` is the reversible archive and restore cycle.
`archived -> permanently deleted` requires an explicit manual purge. Records
never expire automatically in v0.1.

Task status permits explicit transitions among `to_do`, `doing`, and `done` in
either direction. Archival is a separate record-lifecycle transition and not a
task status.

Resource quantity permits `unknown -> known` through an explicit set action,
`known -> known` through set or increment actions, and `known -> unknown`
through an explicit clear action. Quantity and unit are acknowledged together;
changing a unit never converts or silently reinterprets the recorded number.
It never infers a value for `unknown`.

A dismissed recall candidate enters `visible -> dismissed` for the current
context key. The key hashes normalized recall inputs: domain kind, draft or
record identity, recall-relevant text fields, active note paragraph, selected
project associations, and explicit relationship anchors. Normalization ignores
case and repeated whitespace in prose but preserves punctuation and case in
identifiers. Canonical JSON uses fixed field order, explicit nulls for absent
values, sorted ID arrays, Unicode NFC, and SHA-256. A candidate remains
suppressed while the key is unchanged and may return only when the key changes
or in another draft. Dismissal is not a permanent ban.

## Draft Lifecycle

Draft content and save progress are orthogonal states. Draft content is
`clean`, `modified`, or `conflict`; save progress is `idle`, `saving`, or
`error`. The UI shows Saved only for `clean + idle`, Saving for any `saving`,
and Error for any `error`. Editing changes `clean -> modified`; editing during
`saving` keeps save progress `saving` and changes draft content to `modified`.
A network failure changes save progress `saving -> error` without changing or
discarding draft content. Further edits retain `error`; retry first resolves the
failed operation, then sends the latest coalesced draft if needed.

Each edit increments a draft revision. A draft permits one in-flight save;
later edits coalesce into one pending save. Each request captures the draft
revision, a monotonic save generation, a stable operation ID, and the expected
server revision. The server applies an operation only when that expected
revision matches atomically and durably replays the same acknowledgement for a
repeated operation ID. This distinguishes a lost acknowledgement from a true
conflict. After the current response, the latest pending draft is sent against
the newly acknowledged server revision.

Only the current save generation can update client save progress, and
acknowledged server revisions never move backward. An acknowledgement changes
draft content to `clean` only when its draft revision still equals the current
draft revision; otherwise content remains `modified`. A stale server revision
changes draft content to `conflict` and save progress to `idle`, retaining both
local and server content. Explicit reload enters `clean + idle` with server
content; explicit replacement uses a new operation ID against the displayed
server revision; manual merge enters `modified + idle` with a new draft
revision. Tests cover edits during save, lost acknowledgements, idempotent retry,
and success, failure, and conflict outcomes around pending saves.
Last-write-wins is forbidden.

## Archive And Purge Consistency

Archiving a project alone or with its exclusively associated tasks uses a
serializable transaction with retry. It locks the project and candidate task
rows in stable ID order, then re-evaluates associations before changing state.
Every project-association command follows the same task-row lock discipline, so
a task cannot become shared while archive eligibility is decided. Race tests
must prove that shared tasks never cascade. Each archived task then has an
independent lifecycle; restoring the project does not silently restore tasks.

Stored objects transition `available -> deletion_pending -> deleted`.
Attachment-reference creation locks object metadata and accepts only
`available` objects. A purge transaction locks the same metadata, removes the
record references, and marks an unreferenced object `deletion_pending`. The
worker rechecks references under that lock before deleting bytes; surviving
references cancel deletion, and storage failure leaves durable pending work for
retry. Tests race reference creation against purge and preserve shared objects.
Derived vectors cannot remain actionable. Application restore is impossible
after purge; recovery thereafter is an operational backup-restore procedure.

For bidirectional `related` links, endpoint ordering is canonical by domain kind
and stable ID. The database uniqueness key is relationship kind plus the two
canonically ordered endpoints, preventing both exact and reversed duplicates.

## Operational Boundary

Hosting placement and the inference runtime/model are deployment configuration,
not compile-time domain decisions. Browser access uses authenticated HTTPS.
Server-to-server inference uses either a private connection or an appropriately
authenticated TLS endpoint, the declared OpenAI-compatible embeddings contract,
and validated embedding provenance. Deployment and retention selections remain
governed by `docs/initial-build.md` and do not block this interface-design gate.
