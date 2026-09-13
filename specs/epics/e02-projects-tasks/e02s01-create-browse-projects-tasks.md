# e02s01: Create browse and edit projects and tasks

## 1. Metadata

- **ID:** e02s01
- **Type:** feat
- **Context:** domain
- **Status:** todo
- **BCPs:** 5
- **Risk:** P0
- **Epic:** e02 Projects And Tasks

## 2. User Story

As the workspace owner, I want to create, browse, and edit projects, standalone
tasks, and project-associated tasks so that I can maintain work in useful
context without manufacturing placeholder projects or duplicate tasks.

## 3. Problem

Work may belong to no project, one project, or several projects. A generic list
cannot provide useful project context, while copying a task into each project
would split its identity and status.

## 4. Outcome

Projects and tasks have stable identities and safely editable, revisioned
authoritative records. The workspace provides a project list, a standalone task
list, and boards that reuse tasks in `to_do`, `doing`, and `done` lanes.

## 5. In Scope

- Create projects from a name or meaningful description.
- Create tasks from a title or meaningful description, defaulting to `to_do`.
- Optionally record descriptions and task due dates.
- Edit project names/descriptions and task titles/descriptions/due dates through
  the e01s03 shared `DraftSaveCoordinator` and its conflict decisions.
- Associate one task with zero, one, or several projects without copying it.
- Browse active projects, standalone tasks, project details, and project boards.
- Display useful fallback labels for records without user-authored titles.
- Recover unsent drafts locally by default, with a clear control to disable
  persistent local recovery on managed or shared devices.
- Once e06 exists, supply current text and surrounding project/task context from
  every create and edit surface as recall input without requiring a save.

## 6. Out Of Scope

- Recurrence, dependencies, subtasks, priorities, estimates, and assignees.
- Calendar integration, reminders, notifications, and automatic completion.
- General related-record links and recall ranking or presentation behavior.
- URL and file attachment UI integration, which is deferred to e07s01.
- Individual archive/restore commands, which are owned by e01s03, and the project
  archive choice/cascade coordination owned by e02s03.

## 7. Domain Terms

- **Project:** A continuing activity that gathers work and supporting material.
- **Task:** One action record with one stable identity and one status.
- **Project association:** Explicit board-membership relation between a task and project.
- **Standalone task:** An active task with no active project association.
- **Board:** A project view grouping associated tasks by authoritative status.

## 8. Preconditions

- An authenticated application session exists.
- The projects and tasks workspace is available through the responsive shell.
- PostgreSQL migrations can establish stable IDs, revisions, lifecycle state,
  task status, and unique project-task associations.

## 9. Dependencies

- e01s03 supplies revision-aware persistence, generic individual archive/restore
  commands, and the shared `DraftSaveCoordinator`; this story defines no parallel
  save, retry, or conflict semantics.
- e05 later adds general cross-domain relationships; this story only owns
  project-task associations.
- e06 later consumes every project/task create and edit surface as a live recall
  input; e02s01 exposes current draft context but does not own retrieval.
- e07s01 later integrates URL and file attachment controls into these surfaces.
- e02s03 owns only project archive choice and exclusive-task cascade coordination.
- No new external package is required.

## 10. Data

- Project: stable ID, optional name, optional description, revision, lifecycle,
  created timestamp, and updated timestamp; name or description must be meaningful.
- Task: stable ID, optional title, optional description, optional due date,
  status enum (`to_do`, `doing`, `done`), revision, lifecycle, and timestamps;
  title or description must be meaningful.
- Project-task association: project ID and task ID with a uniqueness constraint.
- Active records are returned by default; associations reuse records and never
  copy task content.
- Local recovery stores unsent form state separately from acknowledged records
  and can be disabled and cleared through the managed/shared-device control.

## 11. API

- Provide authenticated create, list, detail, and revision-checked update
  resources for projects and tasks.
- Task creation accepts zero or more project IDs and atomically validates and
  creates the requested associations with the task.
- Project detail returns board membership sufficient to group each task by its
  authoritative status; standalone listing returns tasks with no active project.
- Commands use typed validation, not-found, conflict, and authentication errors.
- Mutation success contains the server-acknowledged record and revision.
- Project/task editors adapt their fields to the e01s03 `DraftSaveCoordinator`
  command/result contract rather than defining domain-specific conflict transitions.
- Once e06 exists, expose bounded current project/task draft context to its recall
  query contract without persisting the draft as a record.

## 12. UI

- Provide distinct project list, task list, project detail, and three-lane board surfaces.
- Create and edit forms make title/name alternatives clear and do not require optional fields.
- Show due dates only when supplied and never imply a reminder.
- Offer touch-usable creation and navigation on Android; board content may use a
  lane selector or grouped vertical layout rather than a shrunken desktop board.
- Render save, retry, and conflict decisions from the shared e01s03
  `DraftSaveCoordinator` while preserving all project/task field values.
- Recover unsent local drafts by default and provide an understandable setting
  that disables and clears persistent recovery on managed or shared devices.
- Once e06 exists, all project/task create and edit forms provide live recall
  input while remaining usable when recall is unavailable.
- Do not show URL or file attachment controls until e07s01 integrates them.

## 13. Security

- Require the authenticated server-owned session for every read and mutation.
- Apply CSRF protection to create and association mutations.
- Validate IDs and payload lengths/types at the Axum boundary.
- Return only workspace-owned records and do not expose raw SQL or secrets in errors.
- Treat locally recovered drafts as personal data and remove persisted drafts when
  the managed/shared-device recovery control is disabled.

## 14. Failure Modes

- Reject a project with neither meaningful name nor description.
- Reject a task with neither meaningful title nor description.
- Reject invalid due dates, status values, missing projects, and archived projects.
- Treat duplicate association requests idempotently or as a typed conflict without
  creating another association.
- On request failure or stale update, delegate save-state and competing-content
  decisions to `DraftSaveCoordinator`; domain adapters retain project/task fields.
- Recall failure never blocks editing or saving, and disabling local recovery
  clearly explains that unsent drafts will not survive browser/device loss.

## 15. Concurrency

- Creation and initial association writes are one transaction.
- Editors rely on the e01s03 coordinator's generation and expected-revision rules;
  this story adds no independent last-write-wins, retry, or conflict state machine.
- Unique project-task associations make concurrent repeated association requests safe.
- Lists and boards derive from acknowledged server state; optimistic placeholders
  cannot be presented as durably saved records.

## 16. Accessibility

- Every input has a programmatic label and validation errors are associated with it.
- Lists and board lanes use semantic headings; status is not conveyed by color alone.
- All creation, opening, and association actions are keyboard operable.
- Touch targets meet the application minimum and layouts remain usable at Android widths.

## 17. Acceptance Criteria

#### ADDED: Create and browse stable project and task records

### Happy Path

```gherkin
Scenario: Create a project and browse one associated task
  Given I have an authenticated session
  When I create a project named "ESP32 Clock"
  And I create a task titled "Wire display" associated with that project
  Then the task is acknowledged with status "To do"
  And the project board shows that same task in the "To do" lane
  And the task list and board refer to one stable task identity
  When I edit the project description and task due date using current revisions
  Then both acknowledged records show the edits with newer revisions
```

### Edge Case

```gherkin
Scenario: Create a standalone description-only task
  Given I have an authenticated session
  When I create a task with no title and description "Order replacement fuse"
  Then the task is saved without a project or due date
  And the standalone task list shows a meaningful description-derived label
  And no placeholder project is created
```

```gherkin
Scenario: Creation fails without losing entered content
  Given I entered a valid task draft
  When the server cannot acknowledge task creation
  Then the interface does not report the task as saved
  And my complete task draft remains available for retry or copy
```

```gherkin
Scenario: Disable persistent local recovery on a managed device
  Given an unsent project or task draft is recoverable on this device
  When I disable persistent local draft recovery and confirm the limitation
  Then the persisted local draft is cleared
  And later input is not persisted locally while the control remains disabled
```

## 18. Test Strategy

- Domain unit tests cover minimum-input rules, default status, fallback labels,
  standalone classification, and revision-checked edits.
- SQLx integration tests cover transactional creation, unique associations,
  active-only queries, and one task identity across several projects.
- Axum tests cover authentication, CSRF, validation, updates, typed errors, and acknowledged revisions.
- Vue component tests cover create/edit adapters to `DraftSaveCoordinator`, board
  grouping, local recovery, its disable-and-clear control, and later e06 input handoff.
- Playwright covers desktop and Android-width create, browse, edit, failure, and recovery workflows.

## 19. Implementation Notes

- Purpose: the projects/tasks domain owns project and task records, project-task
  board membership, and task status semantics.
- Callers: Axum project/task handlers, Vue project/task stores, create/edit forms,
  lists, details, boards, and the later e06 recall coordinator.
- Contracts: stable IDs, one task record, default `to_do`, unique associations,
  active-only default views, and the e01s03 shared draft/save contract.
- Keep domain types independent of Axum extractors, SQLx rows, and Vue state.
- Use explicit SQLx migrations and a transaction for task plus initial associations.

## 20. Definition Of Done

- Every acceptance scenario is automated at the appropriate test level.
- Project and task records persist with stable IDs and revision-aware contracts.
- Desktop and Android-width workflows create, browse, and edit all required record forms.
- Authentication, CSRF, validation, shared-coordinator adoption, and configurable
  local draft recovery are verified.
- Every create/edit surface supplies current recall input once e06 is present;
  URL/file controls remain deferred to e07s01.
- `just test`, `just lint`, and `just build` pass.
- Final `just preflight` reports no new security findings in affected e02s01 paths.
