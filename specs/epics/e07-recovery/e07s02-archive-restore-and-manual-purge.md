# e07s02: Archive Restore and Manual Purge

## 1. Metadata

- **Story ID:** e07s02
- **Epic:** e07 Recovery And Portability
- **Type:** feat
- **Context:** recovery
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0

## 2. User Story

As the workspace owner, I want one place to find archived records, invoke their
existing restore commands, or explicitly purge them so that reversible lifecycle
behavior stays generic and permanent data loss is always deliberate.

## 3. Problem

Generic archive/restore commands and project cascade coordination need one
consistent presentation boundary. Without a
common archived view and filtering contract, removed records can leak into normal
lists and recall or become difficult to restore. Purge must remove authoritative
records, relationships, attachments, and derived data without deleting shared
objects. This story consumes generic archive/restore commands from e01s03 and
project cascade coordination from e02s03; it owns neither command family.

## 4. Outcome

All domain-owned archive results appear in a common, explicitly opened archived
view. Archived records are hidden by default, can invoke the generic independent
restore command, and can be explicitly purged. Generic lifecycle commands remain
owned by e01s03. Project-only and project-plus-exclusive-task coordination remains
implemented and tested by e02s03; this story owns only archived views and final purge.

## 5. In Scope

- Common, domain-filterable archived views for projects, tasks, notes, and resources.
- Archived-view integration of generic e01s03 archive/restore acknowledgements and
  both e02s03 project cascade outcomes.
- Restore controls that invoke e01s03 generic restore commands without redefining them.
- Final manual purge commands and confirmation for all record kinds.
- Consistent archived exclusion across active views, search, recall, direct open, and caches.
- Relationship, attachment-reference, and derived-index cleanup after purge.
- Integration tests for project archive outcomes and race tests for restore, purge, and object references.

## 6. Out Of Scope

- Automatic expiry, scheduled trash emptying, retention timers, and bulk policy administration.
- Restoring a purged record from the application; that requires operational backup restore.
- Cascading restoration of tasks when a project is restored.
- Implementing project archive modes, exclusive-task eligibility, association locking, or archive retry; e02s03 owns those behaviors.
- Reimplementing domain-specific archive commands in the common recovery module.
- Defining generic archive or restore commands, lifecycle transitions, or independent
  restore semantics; e01s03 owns those contracts.
- Implementing project cascade coordination, eligibility, or locking; e02s03 owns them.

## 7. Domain Terms

- **Archive:** Reversible transition from active to archived with stable identity retained.
- **Restore:** Explicit transition from archived to active for one selected record.
- **Exclusive task:** A task associated only with the project being archived at transaction evaluation time.
- **Shared task:** A task associated with another project and therefore never cascade-archived.
- **Purge:** Explicit irreversible removal of an archived authoritative record.

## 8. Preconditions

- Generic archive/restore requests use e01s03 authentication, CSRF, operation ID,
  expected revision, and lifecycle contracts.
- Purge accepts archived records only and requires a separate destructive confirmation.
- e01s03 generic archive/restore commands and e02s03 project cascade coordination
  are implemented before their common-view integration.
- Operational backup recovery is documented before real data is eligible for purge.

## 9. Dependencies

- e01s03 generic archive/restore commands, lifecycle transitions, stable identities,
  revisions, typed conflicts, and idempotent mutation results.
- e02s03 project cascade coordinator, including project archive modes,
  exclusive-task eligibility, serializable locking/retry, and coordinated results.
- e03 and e04 domain records.
- e05 bidirectional links and unique relationship identities.
- e07s01 shared attachment-reference deletion discipline.

## 10. Data

Each record exposes lifecycle state, archive timestamp, revision, and audit-safe
operation identity through the common archived-record projection. Domain archive
commands preserve stable IDs, fields, project associations, general links, and
attachment references. Purge transactionally removes the
record and its relationship endpoints, removes its attachment references, marks
newly unreferenced objects `deletion_pending`, and makes derived indexing entries
non-actionable. Operation replay returns the prior durable result.

## 11. API

- Consume domain archive acknowledgements without defining a second archive endpoint or transaction.
- List archived records only through an explicit, domain-filterable archive query.
- Invoke e01s03 to restore one archived record without restoring linked archived records.
- Purge one archived record only after explicit confirmation; active or stale targets return typed errors.
- Default list, search, direct-open, and recall endpoints exclude archived records unless their explicit contract includes them.

## 12. UI

- Archive and Restore actions invoke e01s03 generic commands; project cascade choices
  additionally invoke e02s03 coordination unchanged.
- Archived views clearly label record kind and archived state and provide Restore and Purge.
- Purge uses a destructive confirmation naming the record and stating that application restore is impossible.
- Restore success reflects e01s03 acknowledgement and returns the record to its
  domain without this story mutating related records.

## 13. Security

- Require authenticated, CSRF-protected mutations and revalidate lifecycle/revision server-side.
- Do not expose archived personal data through default URLs, search, recall, or caches.
- Avoid logging record content during archive, restore, purge, and cleanup.
- Treat purge confirmation as user intent validation, not a substitute for server authorization.

## 14. Failure Modes

- Stale revisions or invalid transitions return typed conflicts and preserve current state.
- A failed domain archive acknowledgement is not inserted optimistically into the common archived view.
- Attachment cleanup failure leaves durable pending work without rolling back acknowledged record purge.
- Derived-index cleanup failure cannot leave a purged result actionable and is retried from durable work.
- Lost acknowledgement is resolved through operation replay.

## 15. Concurrency

Generic archive/restore concurrency is specified, implemented, and race-tested by
e01s03. Project cascade concurrency is specified, implemented, and race-tested by
e02s03. This story contract-tests that their acknowledged outcomes are reflected
accurately in the common archived view. Purge locks record and
object metadata in stable order; new attachment references and cleanup use the
same object lock. Concurrent restore/purge is decided by expected revision and
exactly one valid lifecycle transition commits.

## 16. Accessibility

- Archive, Restore, and Purge have explicit text labels and keyboard/touch access.
- Confirmation focus is trapped, starts on the safe action, and returns predictably.
- Destructive purpose and consequences are announced in text, not color alone.
- Archive lists expose lifecycle labels to assistive technology and preserve logical reading order.

## 17. Acceptance Criteria

#### ADDED: Archived records remain recoverable until explicit purge

### Happy Path

```gherkin
Scenario: Archive and restore a record
  Given an active note with links and an attachment
  When e01s03 archives and later restores that note through the archived view
  Then its stable identity, content, links, and attachment references are unchanged
  And it is excluded from default views only while archived

Scenario: Integrate an acknowledged project archive result
  Given e02s03 archived a project and one exclusive task while leaving a shared task active
  When I open the common archived view
  Then the archived project and exclusive task appear with their record kinds and lifecycle labels
  And the active shared task does not appear
```

### Edge Case

```gherkin
Scenario: Restore a project with archived tasks
  Given a project and an exclusive task were archived together
  When I invoke e01s03 to restore only the project
  Then the project becomes active
  And the task remains archived until I restore it explicitly

Scenario: Purge races restore
  Given an archived resource at a known revision
  When purge and restore race using that revision
  Then exactly one transition commits
  And the losing command receives a typed conflict without partial cleanup

Scenario: Purge an attachment shared by another record
  Given an archived record and an active record reference the same object
  When I purge the archived record
  Then the active record can still open the attachment
  And the shared object is not deleted
```

## 18. Test Strategy

- Domain tests cover common archived projections and final-purge transition matrices;
  generic archive/restore transition tests remain in e01s03.
- SQLx tests cover relationship cleanup, purge idempotency, e01s03 lifecycle-result
  integration, and e02s03 cascade-result integration without re-testing either dependency's internals.
- Deterministic races cover generic-restore/final-purge at the integration boundary,
  reference/purge, and cleanup/reference interleavings; lifecycle command races remain
  in e01s03 and association/archive races remain in e02s03.
- API tests prove archived exclusion from defaults and explicit labeling when included.
- Playwright tests cover archive choices, independent restore, destructive confirmation, and keyboard flow.

## 19. Implementation Notes

Consume e01s03 generic archive/restore commands and e02s03 project cascade
coordination without reproducing their transitions, transactions, eligibility, or
lock logic. This story owns only the common archived query/view, explicit final
purge, and archived-state integration with lists, search, recall, export, and
cleanup workers. Its Restore controls are clients of e01s03, not a restore-command
implementation. Preserve contracts for stable identity, default archived
exclusion, and shared-object safety. Purge is not a general retention subsystem.

## 20. Definition Of Done

- Every e01s03 archive/restore result integrates into one explicit archived view,
  and every archived record can be passed to generic restore or final purge.
- Project cascade invokes e02s03 without duplicate command, transaction, or association-race implementation.
- This story defines no archive or restore command; ownership is limited to archived views and final purge.
- Purge cannot delete shared attachment bytes or leave actionable derived results.
- User-visible destructive actions meet accessibility and authoritative-acknowledgement rules.
- Unit, integration, end-to-end, lint, and production build gates pass.
- A final `just preflight` passes after all story implementation and verification tasks.
