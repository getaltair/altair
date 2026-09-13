# e07s03: Export Backup and Restore Integrity

## 1. Metadata

- **Story ID:** e07s03
- **Epic:** e07 Recovery And Portability
- **Type:** feat
- **Context:** portability
- **Status:** todo
- **BCPs:** 3
- **Risk:** P1

## 2. User Story

As the workspace owner, I want portable exports and a verified operational
backup/restore process so that I retain ownership of my content and can recover
the whole workspace without broken links or attachments.

## 3. Problem

Individual note export does not protect projects, tasks, resources, links, or
files. A database backup taken independently from object storage can restore
metadata that points to missing or wrong bytes. An import that partially
publishes data can corrupt stable identities and relationship integrity.

## 4. Outcome

Notes export as canonical Markdown, workspace export carries structured records
and attachments with an integrity manifest, and operational backup/restore
reconstructs a consistent authoritative PostgreSQL/Garage state. Validation
rejects incomplete or altered material before it becomes active. Explicit
deployment retention selections govern temporary export and backup artifacts,
and verified cleanup proves that expired or abandoned artifacts do not persist.

## 5. In Scope

- Single-note canonical Markdown export.
- Versioned workspace export of records, stable IDs, links, attachment metadata, and bytes.
- Checksums, counts, schema version, and referential-integrity validation.
- Documented, automated database/object backup and empty-target restore verification.
- Retrieval-index rebuild after authoritative restore.
- Explicit retention selection for completed exports, incomplete export staging,
  and temporary backup/restore staging, with automated cleanup verification.

## 6. Out Of Scope

- Continuous synchronization with another application or cloud backup vendor.
- Importing arbitrary third-party workspace formats.
- Incremental point-in-time recovery UI, cross-version downgrade, or merging two workspaces.

## 7. Domain Terms

- **Workspace export:** User-requested portable package of canonical application data.
- **Operational backup:** Deployment-owned recoverable snapshot of PostgreSQL and Garage.
- **Manifest:** Versioned inventory of records, relationships, objects, sizes, and checksums.
- **Restore validation:** Checks performed before restored data is exposed as authoritative.
- **Derived data:** Search indexes and embeddings that can be rebuilt from authoritative content.
- **Temporary artifact:** A generated export or backup/restore staging file that is not authoritative workspace data.
- **Retention selection:** Deployment configuration that states how long each temporary artifact class remains eligible for use before cleanup.

## 8. Preconditions

- Export requires an authenticated session and CSRF protection for creation/download commands.
- Backup tooling has least-privilege access to PostgreSQL and Garage.
- Restore targets an empty, compatible deployment and records its schema compatibility.
- Encryption and retention of backup artifacts are deployment configuration, not browser defaults.
- Retention values for each temporary artifact class are explicitly selected before export or backup jobs run.

## 9. Dependencies

- e01 migrations, configuration, authentication, and operational command interface.
- e02 through e05 stable domain records and relationships.
- e07s01 attachment metadata/object invariants.
- e07s02 lifecycle semantics for active and archived records.
- e06 rebuildable retrieval indexing and embedding provenance.

## 10. Data

The workspace manifest contains export format version, creation time, record
counts by kind/lifecycle, stable IDs, relationship endpoint IDs/kinds,
attachment IDs, relative object paths, byte sizes, and cryptographic checksums.
Canonical note Markdown is stored independently of editor state. Credentials,
sessions, transient drafts, caches, and vectors are excluded. Restore validates
unique IDs, known kinds, endpoint existence, object inventory, size/checksum,
and schema compatibility before publication. Temporary artifact metadata records
its class, job/backup ID, creation and expiry times, completion state, checksum,
and cleanup state without storing credentials. Durable operational backups and
temporary staging artifacts have distinct retention selections.

## 11. API

- Export one note as canonical Markdown with a safe filename and explicit content type.
- Start and inspect a workspace export job without placing personal content in logs.
- Download completed export through an authenticated, non-cacheable response.
- Report an expired export as unavailable after cleanup without regenerating it implicitly.
- Operational backup/restore runs through repeatable server-side commands, not a public anonymous endpoint.
- Restore reports validation failures by safe manifest location and never silently skips records or objects.

## 12. UI

- Note export is available from note actions without requiring metadata cleanup.
- Workspace export shows preparing, ready, failed, and expired-download states.
- The UI states what is included and excludes secrets, sessions, local drafts, and derived indexes.
- Restore remains an operational workflow in v0.1; no misleading in-app merge/import control is shown.

## 13. Security

- Require authentication and CSRF protection; use `Cache-Control: no-store` for export status and downloads.
- Prevent path traversal, archive bombs, unsafe filenames, and cross-workspace object references during validation.
- Never include credentials, provider tokens, session records, or deployment secrets.
- Protect backup/export artifacts at rest and remove temporary server artifacts according to explicit deployment retention.
- Cleanup logs artifact IDs, classes, and outcomes but never filenames containing personal data, archive contents, or credentials.

## 14. Failure Modes

- Export failure leaves authoritative data unchanged and reports a retryable or permanent typed error.
- Missing/mismatched bytes fail validation before restore publication.
- Unsupported manifest/schema versions stop restore with no partial active workspace.
- Interrupted backup is marked incomplete and cannot be selected as restorable.
- Missing retention configuration fails the affected export/backup operation closed; it does not retain temporary artifacts indefinitely.
- Cleanup failure remains durable and retryable, and verification fails while an expired artifact remains accessible or present.
- Index rebuild failure leaves authoritative records usable while recall reports degraded coverage.

## 15. Concurrency

Workspace export reads a database-consistent snapshot and includes only the
attachment inventory bound to that snapshot. Object collection verifies each
manifest checksum; concurrent later edits belong to a later export. Backup
coordinates the database snapshot with a fixed object inventory and marks the
set complete only after both parts validate. Restore is single-writer, stages
and validates data before activation, and is idempotent for the same restore ID.
Download/restore use leases serialize with cleanup: cleanup can remove only an
expired artifact with no active lease, and a new lease cannot start after cleanup
selection. Repeated cleanup is idempotent and rechecks artifact state before deletion.

## 16. Accessibility

- Export controls have explicit names and are keyboard/touch operable.
- Progress and completion are announced without stealing focus.
- Download errors identify a recovery action and do not rely on color.
- Inclusion/exclusion details use semantic lists readable by assistive technology.

## 17. Acceptance Criteria

#### ADDED: Portable exports and verified restore preserve workspace integrity

### Happy Path

```gherkin
Scenario: Export a canonical note
  Given a note contains authored Markdown and editor-specific state
  When I export the note
  Then the downloaded file contains the canonical authored Markdown
  And it excludes editor state and generated assertions

Scenario: Restore a complete workspace backup
  Given a validated backup contains records, archived records, relationships, and attachments
  When I restore it into an empty compatible deployment
  Then all stable IDs and relationship endpoints are preserved
  And every attachment checksum matches and opens from its referencing records
  And retrieval indexes can be rebuilt from restored authoritative content
```

### Edge Case

```gherkin
Scenario: Reject a missing attachment object
  Given a restore manifest references an object that is absent
  When restore validation runs
  Then restoration fails before any workspace data is published
  And the error identifies the safe manifest entry without exposing credentials

Scenario: Export while a record changes
  Given a workspace export has acquired its database snapshot
  When a record and attachment change concurrently
  Then the export contains the internally consistent pre-change snapshot
  And the later change remains authoritative in the live workspace

Scenario: Retrieval rebuild is unavailable after restore
  Given authoritative records and attachments restored successfully
  When semantic indexing is unavailable
  Then restored content, direct links, and non-semantic search remain usable
  And recall reports degraded coverage rather than data loss

Scenario: Clean up expired temporary artifacts
  Given explicit retention selections exist for export and backup staging artifacts
  And one completed export and one abandoned backup staging artifact have expired
  When the cleanup command runs and verification inspects storage and metadata
  Then neither artifact remains downloadable or present in temporary storage
  And durable operational backups and authoritative workspace data remain unchanged
```

## 18. Test Strategy

- Golden-file tests cover canonical Markdown and versioned manifest serialization.
- Property/integration tests validate IDs, endpoint closure, counts, paths, sizes, and checksums.
- Tamper tests cover missing, altered, duplicate, unknown-version, traversal, and oversized archive entries.
- A fixture workspace round trip compares semantic authoritative content before export and after restore.
- Operational tests restore a fresh backup into an empty test deployment and rebuild derived indexes.
- Retention tests cover required configuration, expiry boundaries, active download/restore leases, abandoned staging, retryable deletion, idempotent cleanup, and post-cleanup storage verification.

## 19. Implementation Notes

Separate portable workspace export from deployment backup while sharing manifest
integrity rules where useful. Callers are note actions, authenticated export
handlers, cleanup workers, and operational commands. Contracts are canonical Markdown, stable IDs,
referential closure, byte checksums, no secrets, and no partial restore
publication. This story owns explicit retention selection and verified cleanup
for temporary export and backup/restore artifacts; it does not apply application
record purge policy. Use streaming I/O from the accepted stack and add no package until
measured format or memory needs justify one.

## 20. Definition Of Done

- Canonical note and complete workspace exports pass deterministic format tests.
- Backup and restore preserve stable IDs, links, lifecycle states, and attachment bytes.
- Corrupt or incompatible artifacts fail before publication with actionable safe errors.
- Derived indexes rebuild independently and failures degrade honestly.
- Temporary export and backup/restore artifacts use explicit retention selections and cleanup verification proves expired bytes and access are removed.
- Security, round-trip, lint, test, and production build gates pass.
- A final `just preflight` passes after all story implementation and verification tasks.
