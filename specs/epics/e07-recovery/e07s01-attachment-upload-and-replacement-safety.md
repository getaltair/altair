# e07s01: Attachment Upload and Replacement Safety

## 1. Metadata

- **Story ID:** e07s01
- **Epic:** e07 Recovery And Portability
- **Type:** feat
- **Context:** attachments
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want to attach files and structured URLs while creating
or editing records, with safe replacement, cancellation, and removal, so that an
interrupted or concurrent operation never loses acknowledged data or breaks
another record's attachment.

## 3. Problem

Garage stores file bytes while PostgreSQL stores records, structured URLs,
attachment metadata, and references. Initial record creation, upload,
replacement, cancellation, and deletion cross that boundary. In-place object
replacement, non-atomic record/reference creation, premature success, or
unchecked abandoned cleanup can leave empty records, dangling references,
orphan bytes, or delete an object still shared by another record.

## 4. Outcome

Existing records in every domain accept files and structured URLs. Quick capture
can create an ordinary note from an attachment alone, and a resource can be
created from an identifying attachment alone. Projects and tasks retain their
text minimums while accepting initial files and URLs. Initial record and
attachment-reference writes commit atomically after file verification;
replacement uses a new key and preserves the old attachment until the swap
commits. Abandoned cleanup is durable and retryable, and object deletion occurs
only after a locked reference recheck proves that no reference survives.

## 5. In Scope

- File upload and structured-URL attachment for existing projects, tasks, notes, and resources.
- Attachment-based global quick capture and identifying-attachment-only resource creation.
- Initial project/task creation with files or structured URLs while preserving their text minimums.
- Atomic initial record-plus-attachment-reference creation after file verification.
- Upload, retry, cancellation, replacement, unlink, abandoned-upload cleanup, and object cleanup.
- Transition and precedence tests for interruption and concurrent commands.
- Shared-object reference preservation and delayed physical deletion.
- User-visible progress, failure, cancellation, and acknowledged completion.
- Attachment-name integration with global lexical search after attachment metadata exists.
- Integration of attachment metadata and references with existing search/recall
  extension contracts and adapters for post-e07 responsive and release workflows;
  this story owns attachment integration added after e07 rather than delegating it
  to later epics. e07s02 and e07s03 retain their own purge and export/restore scope.

## 6. Out Of Scope

- S3 bucket versioning or a user-facing file-version history.
- OCR, attachment body indexing, resumable multipart uploads, and media editing.
- Automatic replacement based on matching filenames.
- Fetching, crawling, previewing, or deriving content from structured URLs.

## 7. Domain Terms

- **Object:** Bytes under an immutable Garage object key.
- **Attachment metadata:** PostgreSQL identity, filename, media type, size, checksum, object key, and lifecycle state.
- **Reference:** A record-to-attachment association; multiple records may share one object.
- **Acknowledged attachment:** An `available` object whose reference transaction committed.
- **Replacement:** An explicit swap from one reference to a newly uploaded object.
- **Structured URL:** A validated user-authored URL attachment stored without fetching its content.
- **Initial attachment-only record:** An ordinary note or resource whose accepted minimum input is an attachment.
- **Abandoned upload:** Pending or uploaded bytes that no live operation can complete and that durable cleanup may reclaim after the selected grace period.

## 8. Preconditions

- The user has an authenticated application session and CSRF-valid mutation request.
- An existing target record is active and supplies its current revision, or an
  initial create request satisfies the target domain's minimum-input rule.
- Garage and PostgreSQL attachment infrastructure are configured.
- Object keys are generated server-side and are never reused for replacement.

## 9. Dependencies

- e01 persistence, sessions, typed errors, and background-work foundation.
- e02 project/task minimum-input rules and create contracts.
- e03 ordinary-note quick capture and the e04 resource record contract.
- e05 relationship conventions where attachment ownership is exposed across records.
- e06 global lexical search and recall extension contracts; e07s01 supplies the
  attachment-name candidate source only after attachment metadata exists, so e06
  does not depend on e07.
- PostgreSQL transactions and Garage's S3-compatible object API.

## 10. Data

File attachment metadata uses `upload_pending`, `available`, `deletion_pending`,
and `deleted`; structured URLs are validated PostgreSQL child records and never
enter the object lifecycle. A new file upload creates `upload_pending` metadata before bytes are sent.
Successful byte verification permits `upload_pending -> available`; failure or
cancellation permits `upload_pending -> deletion_pending`. An available object
enters `deletion_pending` only when a transaction removes the intended reference
and finds no remaining references. The cleanup worker rechecks under the same
metadata lock before `deletion_pending -> deleted`. A surviving/new reference
causes `deletion_pending -> available`. Initial record creation and all requested
file/URL references use one PostgreSQL transaction, so either the record and
complete reference set commit or none do. A failed create leaves verified,
unreferenced files eligible for durable cleanup. Each record-to-file reference
has its own identity; adding or removing one reference leaves every other
reference intact, and only the zero-reference transition may select object
deletion. Operation IDs make repeated
create, commit, cancel, replace, and unlink commands idempotent.

## 11. API

- Start upload returns an operation ID, attachment ID, immutable object key, and upload instructions.
- Complete upload validates expected size/checksum and makes the object available before creating or swapping a reference transactionally.
- Create a quick-capture note or resource from an available attachment without requiring text; create projects/tasks with valid text plus initial available files or validated structured URLs.
- Commit each initial record and its complete attachment-reference set atomically; replaying the create operation returns the same record and references.
- Add an available file reference to another authorized record idempotently; replacement or unlink changes only the selected reference and preserves all others.
- Cancel marks incomplete work for cleanup; it does not remove an acknowledged reference.
- Replace accepts target record revision, old attachment ID, new attachment ID, and operation ID.
- Add, replace, and remove structured URLs through revision-checked, idempotent record commands without fetching remote content.
- Remove unlinks one reference and reports success only after PostgreSQL acknowledgement; byte deletion remains asynchronous.
- Repeated operation IDs return the durable prior result; stale record revisions return a typed conflict.
- Global lexical search reads normalized attachment names from acknowledged metadata,
  returns the authorized referencing records with stable-identity deduplication, and
  excludes pending, deleted, and archived results by default.

## 12. UI

- Show selected, uploading, finalizing, attached, failed, and cancelled states.
- Offer attachment-based quick capture and identifying-attachment-only resource creation without inventing titles, and file/URL controls on project/task create and edit forms.
- Keep the old attachment openable while replacement uploads or finalization fails.
- Enable retry or cancel without clearing the selected filename or record draft.
- Show replacement/removal success only after the reference transaction is acknowledged.
- Do not present background byte cleanup as part of the user's required workflow.

## 13. Security

- Require authenticated sessions, CSRF protection, and authorization to the target record for every mutation.
- Generate opaque object keys server-side and use bounded, content-length-limited upload authorization.
- Validate filename, declared media type, byte size, and checksum as untrusted boundary data.
- Do not expose Garage credentials, internal keys, or another record's attachment through errors.

## 14. Failure Modes

- Garage upload failure leaves the current attachment unchanged and queues partial bytes for retryable cleanup.
- Initial creation failure commits neither the record nor any reference and retains the draft/selection for retry; verified unreferenced bytes enter cleanup.
- Lost completion acknowledgement is resolved by replaying the operation ID, not by uploading another copy.
- Database failure after byte upload leaves durable cleanup work and no claimed reference.
- Abandoned `upload_pending` work is selected only after its configured grace period and is rechecked under lock before cleanup.
- Cleanup storage failure leaves `deletion_pending` durable for bounded retry.
- Checksum or size mismatch rejects finalization and never publishes the attachment.

## 15. Concurrency

Completion, initial record/reference creation, and cancellation serialize on
upload metadata. Cancellation wins only
before an `available` reference commit; after commit it returns the committed
result and removal requires a separate command. An initial create locks all
requested attachment metadata in stable ID order and commits the record and
references in one PostgreSQL transaction; any invalid/unavailable attachment
rolls back the entire create. Concurrent replacement commits
use target record revision so at most one swaps the displayed reference. Reference
creation and purge lock object metadata identically: creation accepts only
`available`; purge removes its reference, marks zero-reference objects pending,
then the worker rechecks references under lock. A surviving reference always
prevents physical deletion.

## 16. Accessibility

- Progress and state changes are announced through a polite status region without moving focus.
- Cancel, retry, replace, download, and remove are keyboard and touch operable with persistent text labels.
- Errors identify the affected filename and recovery action without relying only on color.
- Focus returns to the initiating control or the replacement attachment after completion.

## 17. Acceptance Criteria

#### ADDED: Attachments remain safe through creation, replacement, and cleanup

### Happy Path

```gherkin
Scenario: Upload and attach a file
  Given an active record and no attachment operation in progress
  When I upload a file whose bytes match the declared size and checksum
Then a new immutable object becomes available
  And its reference is reported attached only after the database commit

Scenario: Create attachment-only records atomically
  Given I have one verified file and no authored text
  When I save it through quick capture and later create a resource from an identifying attachment
  Then each command creates one ordinary record and its attachment reference in one transaction
  And neither record receives invented authored text or classification

Scenario: Add files and structured URLs to project work
  Given valid project and task drafts satisfy their text minimums
  When I create them with files and structured URLs
  Then each acknowledged record contains its complete initial attachment set
  And no URL content is fetched or silently interpreted

Scenario: Find a record by attachment name
  Given an active record references an available attachment named esp32-clock-manual.pdf
  When I search globally for part of that attachment name
  Then the authorized referencing record appears once in lexical search results
  And pending deleted or archived attachment results remain excluded by default

Scenario: Replace an attachment safely
  Given a record references an available old attachment
  When a new upload becomes available and I confirm replacement
  Then the reference atomically points to the new attachment
  And the old object is retained when any other reference uses it
```

### Edge Case

```gherkin
Scenario: Cancel before upload completion
  Given an upload is pending and no reference has committed
  When cancellation and completion race
  Then exactly one durable outcome wins under the metadata lock
  And cancellation that wins leaves no record reference and queues byte cleanup

Scenario: Replacement fails after bytes upload
  Given the old attachment is acknowledged and openable
  When Garage accepts replacement bytes but the reference transaction fails
  Then the old reference remains unchanged
  And the unreferenced new object is durably scheduled for cleanup

Scenario: Initial record creation fails after file verification
  Given replacement-safe file bytes are available for a new attachment-only resource
  When the record and reference transaction fails
  Then no resource or attachment reference is published
  And the unreferenced file is selected for retryable abandoned cleanup after its grace period

Scenario: Purge races a new shared reference
  Given two operations target the same available object
  When one removes the last observed reference while another creates a reference
  Then lock ordering prevents deletion of an object with a surviving reference
  And the final metadata state agrees with the final reference count

Scenario: Replace one of two shared references
  Given two records reference the same available object
  When I replace the attachment on only one record
  Then the other record retains and can open its original reference
  And the original object remains available until its final reference is removed

Scenario: Remove acknowledgement is lost
  Given removal committed but its response was lost
  When the client retries with the same operation ID
  Then the server returns the original result without removing another reference
```

## 18. Test Strategy

- Domain transition-table tests cover every legal and rejected file/URL state and command pair.
- SQLx integration tests cover every domain, attachment-only note/resource creation, project/task initial attachments, idempotency, stale revisions, atomic initial creation/replacement, zero-reference decisions, and attachment-name search after metadata commit.
- Deterministic race tests pause at metadata locks for create/cancel, cancel/complete, replace/replace, abandoned cleanup/completion, reference/purge, and worker/reference interleavings.
- Garage contract tests cover upload verification, missing bytes, and retryable deletion failures.
- Vue tests cover draft preservation, authoritative success, status announcements, retry, and cancel.

## 19. Implementation Notes

Keep attachment lifecycle rules independent of Axum, SQLx rows, and S3 payloads.
The attachment module's purpose is consistent metadata/object coordination; its
callers are all domain create/edit commands, quick capture, attachment HTTP handlers,
search/recall adapters, post-e07 responsive/release adapters, export/restore
consumers, and cleanup workers. It owns every attachment integration introduced
after e07; e07s02 final purge and e07s03 export/restore consume its contracts without
becoming dependencies of e07s01. Earlier stories expose extension contracts but do
not depend on attachment metadata. Its contracts are immutable replacement keys, acknowledged
and atomic initial references, idempotent commands, abandoned cleanup, structured
URLs without fetching, and no deletion with surviving references. Use
the existing stack only; no external package or new abstraction is proposed.

## 20. Definition Of Done

- All lifecycle transitions and command precedence are implemented and documented by tests.
- Existing-record integration, attachment-only creation, initial project/task attachments, upload, replacement, cancellation, abandoned cleanup, shared-reference, deletion, and race tests pass.
- Attachment-name search is integrated only after acknowledged metadata exists and post-e07 attachment integrations use this story's contracts.
- The UI preserves drafts and the prior acknowledged attachment through failures.
- Security, accessibility, lint, test, and production build gates pass.
- No object with a surviving reference can be deleted in deterministic race coverage.
- A final `just preflight` passes after all story implementation and verification tasks.
