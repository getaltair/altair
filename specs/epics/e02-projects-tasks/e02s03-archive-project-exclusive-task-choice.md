# e02s03: Archive project with explicit exclusive-task choice

## 1. Metadata

- **ID:** e02s03
- **Type:** feat
- **Context:** domain
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0
- **Epic:** e02 Projects And Tasks

## 2. User Story

As the workspace owner, I want an explicit choice when archiving a project so
that I can archive only the project or also its exclusively associated tasks,
while shared tasks remain active.

## 3. Problem

Cascading archive based on a stale association snapshot can remove work that is
shared or becoming shared. Domain-specific lifecycle commands would also duplicate
the generic individual archive/restore ownership established by e01s03.

## 4. Outcome

Project archive coordination defaults to project-only. The optional project-plus-
exclusive-tasks flow re-evaluates eligibility in one serializable transaction and
uses e01s03 generic archive commands; shared tasks never cascade.

## 5. In Scope

- Own only the project archive choice and exclusive-task cascade coordination.
- Present project-only and project-plus-exclusive-tasks choices, defaulting to project-only.
- Define exclusive eligibility at transaction time from active project associations.
- Coordinate e01s03 generic individual archive commands for the project and
  eligible tasks atomically in a serializable transaction with retry.
- Serialize association/archive races using stable task-row lock order.
- Keep shared tasks active and preserve all associations and stable IDs.

## 6. Out Of Scope

- Permanent purge, retention policy, automatic expiry, and attachment deletion.
- Individual project/task archive and restore command semantics, owned by e01s03.
- Automatically restoring tasks when a project is restored.
- Archiving shared tasks, changing task status, or removing associations implicitly.
- Bulk restore and general archived-record search beyond required selection access.

## 7. Domain Terms

- **Project-only archive:** Archive the project and no tasks.
- **Exclusive task:** At commit eligibility time, a task associated with this
  project and no other active project.
- **Shared task:** A task associated with at least one other active project.
- **Generic archive command:** The e01s03 operation that archives one record.
- **Association/archive race:** Concurrent project-association and archive commands.

## 8. Preconditions

- e02s01 supplies stable project/task IDs and associations; e01s03 supplies
  lifecycle state and generic individual archive/restore commands.
- All association commands can follow the shared task-row lock discipline.
- The database supports serializable transactions and retry classification.

## 9. Dependencies

- Depends on e02s01 associations and archived-state filtering.
- Uses e01s03 generic archive/restore commands, authenticated mutation, CSRF,
  typed conflicts, idempotency, and transaction support.
- No e02 story defines individual archive/restore semantics; e02s03 defines only
  the project choice, exclusivity rule, and association/archive race coordination.
- e07 may later provide consolidated archived access and permanent purge without
  redefining e01s03 lifecycle commands or this cascade rule.
- No new external package is required.

## 10. Data

- Project and task lifecycle values and individual transitions come from e01s03.
- Associations remain stored while either endpoint is archived.
- Archive command includes project ID, mode, expected project revision, and operation ID.
- Response identifies archived project and task IDs and skipped shared task IDs.

## 11. API

- Provide one project archive coordinator with modes `project_only` and
  `project_and_exclusive_tasks`; omitted mode resolves to `project_only`.
- Invoke only e01s03 generic archive commands for individual lifecycle mutations;
  callers use e01s03 generic restore directly.
- Require authentication and CSRF protection for every lifecycle mutation.
- Return typed validation, not-found, already-archived, serialization-exhausted,
  and revision-conflict results without claiming partial success.
- Idempotent operation replay returns the original acknowledged result.

## 12. UI

- Archive opens a confirmation that clearly defaults to “Archive project only.”
- The optional destructive choice names the current candidate exclusive-task count
  but explains that final eligibility is checked on save and shared tasks stay active.
- Report success only after the complete transaction acknowledgement.
- Active lists hide archived records; explicit archived access labels lifecycle state.
- Individual restore controls invoke e01s03 directly and are not owned by this story.

## 13. Security

- Require an authenticated session and CSRF protection for archive coordination.
- Validate mode, IDs, expected revisions, and operation IDs at the boundary.
- Determine exclusivity from authoritative database associations, never client claims.
- Keep record content out of transaction retry and error logs.

## 14. Failure Modes

- Missing, already archived, or stale project returns a typed error without task changes.
- Serialization failure retries a bounded number of times; exhaustion returns no success claim.
- Any task archive failure rolls back project and task lifecycle changes together.
- Shared or newly shared tasks are skipped from cascade and remain active.

## 15. Concurrency

- Use a serializable transaction with bounded retry for project-plus-exclusive-task archive.
- Lock the project and candidate task rows in stable ID order, then re-read associations.
- Every add/remove project-association command locks the same task row before changing membership.
- Race: association to another project wins first, so archive re-evaluates and keeps task active;
  archive wins first, so the association command observes archived lifecycle and fails explicitly.
- Race: removing the last other association wins first, so the task may become exclusive;
  archive wins first, so its locked snapshot determines eligibility before removal proceeds.
- Project-only archive locks only what is required and never mutates tasks.

## 16. Accessibility

- Confirmation identifies the default choice in text, not color or position alone.
- Options use a named radio group and remain keyboard and screen-reader operable.
- The destructive scope and resulting archived/skipped counts are announced clearly.
- Focus returns to a stable destination after the project leaves the active list.

## 17. Acceptance Criteria

#### ADDED: Explicit project archive scope protects shared tasks

### Happy Path

```gherkin
Scenario: Archive a project with only its exclusive tasks
  Given project A has exclusive task X and task Y shared with project B
  When I explicitly choose "Project and exclusive tasks" and confirm
  Then project A and task X are archived in one acknowledged transaction
  And task Y remains active with the same status and identity
  And all project-task associations remain recorded
```

```gherkin
Scenario: Restoring a project does not reverse the cascade
  Given project A and exclusive task X were archived together
  When I invoke the e01s03 generic restore command for project A
  Then project A is active and task X remains archived
  And e02s03 does not issue an automatic restore for task X
```

### Edge Case

```gherkin
Scenario: A task becomes shared while archive eligibility is decided
  Given task X is associated only with project A
  When one transaction associates task X with project B
  And another transaction archives project A with exclusive tasks concurrently
  Then serializable locking determines a valid order
  And task X is never archived while committed as shared
  And neither transaction reports a partial result
```

```gherkin
Scenario: Default archive changes only the project
  Given project A has two exclusive tasks
  When I confirm archive without selecting the optional task choice
  Then only project A is archived
  And both tasks remain active with unchanged statuses
```

## 18. Test Strategy

- Domain tests cover mode defaulting, exclusivity, shared-task exclusion, and
  orchestration through e01s03 generic archive commands.
- SQLx integration tests run real serializable transactions for add-association,
  remove-association, archive coordination, retry, rollback, and stable lock order.
- API tests cover auth, CSRF, stale revisions, idempotent replay, and typed retry exhaustion.
- UI tests cover explicit default selection, candidate explanation, acknowledgement,
  failures, and archived filtering.
- Playwright covers project-only, cascade, shared-task, and no-automatic-restore workflows.

## 19. Implementation Notes

- Purpose: project archive coordination preserves data while making cascade scope explicit.
- Callers: project archive coordinator, association commands, project details, and stores.
- Contracts: project-only default, transactional exclusive cascade, shared-task
  safety, stable lock order, bounded serializable retry, and persistent associations.
- This story owns only the exclusive-task prompt, cascade eligibility, and
  archive/association races; e01s03 owns individual archive/restore commands.
- Keep eligibility and lifecycle transition logic in the domain/application layer;
  SQLx supplies locking and transaction mechanics but does not define the rule.
- Reason for Depth: shared lock discipline is required to prevent a committed
  association from racing past exclusive-task eligibility and causing data loss.

## 20. Definition Of Done

- Real-database race tests prove shared tasks never cascade under association/archive races.
- Project-only and project-plus-exclusive-task paths are atomic and revision-aware.
- Coordination is tested against e01s03 generic archive commands, including proof
  that no restore is triggered automatically.
- UI defaults, warnings, failure handling, and accessibility are verified.
- `just test`, `just lint`, and `just build` pass.
- Final `just preflight` reports no new security findings in affected e02s03 paths.
