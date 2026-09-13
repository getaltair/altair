# e02s02: Change shared task board status

## 1. Metadata

- **ID:** e02s02
- **Type:** feat
- **Context:** domain
- **Status:** todo
- **BCPs:** 3
- **Risk:** P1
- **Epic:** e02 Projects And Tasks

## 2. User Story

As the workspace owner, I want to change a task's board status from any project
that displays it so that shared work has one truthful status everywhere.

## 3. Problem

A task may appear on several boards. Board-local state would allow the same task
to appear as both Doing and Done, violating its single-record identity.

## 4. Outcome

An explicit status command transitions one authoritative task among `to_do`,
`doing`, and `done`; every board and task view reflects the acknowledged result.

## 5. In Scope

- Transition active tasks among all three states in either direction.
- Invoke transitions with drag-and-drop and an equivalent menu/tap action.
- Refresh every visible representation of the shared task after acknowledgement.
- Reject stale revisions without hiding either competing status.

## 6. Out Of Scope

- Custom workflows, completion gates, status history, and task automation.
- Project-specific task status, recurrence, notifications, and collaboration.
- Archiving tasks or projects.

## 7. Domain Terms

- **Task status:** The one authoritative workflow state of a task.
- **Shared task:** One task associated with more than one project.
- **Status command:** Explicit user request to transition a task.
- **Acknowledged status:** Status confirmed by the server at a specific revision.

## 8. Preconditions

- e02s01 provides active tasks, project associations, boards, and revisions.
- The task is active and visible on at least one project board.
- The user has an authenticated session.

## 9. Dependencies

- Depends on e02s01 project/task persistence and board queries.
- Uses e01s03 authentication, CSRF, typed errors, and generic revision-conflict
  contract; it does not define editor conflict semantics.
- No new external package is required.

## 10. Data

- Status is stored once on the task, never on a project-task association.
- Allowed values are `to_do`, `doing`, and `done`.
- A mutation includes task ID, target status, expected revision, and operation ID.
- The acknowledged response includes the new status and monotonically advanced revision.

## 11. API

- Provide one authenticated, CSRF-protected task-status command.
- Validate task lifecycle, target status, expected revision, and operation ID.
- Replaying an operation ID returns the same acknowledgement without another mutation.
- A stale expected revision returns a typed conflict containing current server state.

## 12. UI

- Support pointer drag where appropriate and a keyboard/touch menu on every board.
- Do not announce success or permanently move the card until server acknowledgement.
- Update all loaded boards and task details by stable task ID after acknowledgement.
- On failure, retain or restore the last acknowledged placement and expose retry.
- On conflict, preserve the status-command intent and present the e01s03 typed
  conflict result without introducing an editor conflict state machine.

## 13. Security

- Require the authenticated session and CSRF protection for status commands.
- Validate all enum and identifier values server-side.
- Do not trust project membership supplied by the client as authorization evidence.
- Avoid logging personal task titles in mutation diagnostics.

## 14. Failure Modes

- Reject transitions for missing or archived tasks.
- Reject unknown statuses and malformed revisions.
- Network failure retains the last acknowledged status and a retryable user intent.
- Conflict never silently overwrites the status acknowledged on another device.
- A failed update on one board cannot leave another loaded board claiming success.

## 15. Concurrency

- Compare expected revision and update status atomically.
- Operation IDs make retries safe after a lost acknowledgement.
- Only the latest command generation may update client progress indicators.
- Concurrent commands produce one success and one explicit conflict, not last-write-wins.

## 16. Accessibility

- Status can be changed without drag-and-drop using keyboard and touch controls.
- Lanes have semantic labels and status changes are announced through an ARIA live region.
- Focus remains on the moved task or its replacement position after acknowledgement.
- Color is supplementary; status text remains visible.

## 17. Acceptance Criteria

#### ADDED: One authoritative status for shared tasks

### Happy Path

```gherkin
Scenario: Change one shared task from either project board
  Given task "Wire display" is associated with projects A and B
  And its acknowledged status is "To do"
  When I change it to "Doing" from project A
  Then the server acknowledges one task revision with status "Doing"
  And project A and project B both show the task in "Doing"
  And no board-local task copy is created
```

### Edge Case

```gherkin
Scenario: Reject a stale status change without silent overwrite
  Given two devices display the same task revision
  When device A changes the task to "Done"
  And device B attempts to change its stale revision to "Doing"
  Then device B receives a conflict with the current "Done" status
  And the interface preserves device B's intended "Doing" choice for explicit retry
```

```gherkin
Scenario: Change status without dragging
  Given I use a keyboard or touch interface on a project board
  When I choose "To do" from the task status menu
  Then the same status command and acknowledgement behavior is used
  And focus remains associated with the task
```

## 18. Test Strategy

- Domain tests cover all forward and reverse transitions and reject invalid states.
- Integration tests cover atomic revision checks and idempotent operation replay.
- API tests cover auth, CSRF, archived tasks, validation, and typed conflicts.
- Store/component tests cover stable-ID fan-out, rollback, retry, and accessible menu use.
- Browser tests cover cross-board updates and desktop drag plus Android tap actions.

## 19. Implementation Notes

- Purpose: task status is domain state independent of any board component.
- Callers: status API handler, task store, project boards, and task details.
- Contracts: one status per stable task, explicit bidirectional transitions,
  e01s03 expected-revision conflicts, idempotent retry, and acknowledged UI state.
- Keep board grouping as a projection of task status, not mutable lane-owned state.

## 20. Definition Of Done

- Shared tasks update consistently across every loaded representation.
- Drag, keyboard, and touch paths use the same server command.
- Failure, lost-acknowledgement retry, and conflict behavior are automated.
- Accessibility announcements and focus behavior are verified.
- `just test`, `just lint`, and `just build` pass.
- Final `just preflight` reports no new security findings in affected e02s02 paths.
