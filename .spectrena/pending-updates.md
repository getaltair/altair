# Pending Template Updates

These files have been modified from the original template.
Use `/spectrena.review-updates` to review and merge changes.

---

## .spectrena/templates/spectrena-gitignore

**Your version hash:** `51622cc914d7`
**New version hash:** `51622cc914d7`

### Diff

```diff
(diff not available)
```

### New Version Content

```markdown
# Spectrena update tracking (machine-generated)

.template-hashes.json
pending-updates.md

# Temporary files

_.tmp
_.bak
\*~
```

---

## .spectrena/templates/agent-file-template.md

**Your version hash:** `55ed438c2e86`
**New version hash:** `55ed438c2e86`

### Diff

```diff
(diff not available)
```

### New Version Content

````markdown
# [PROJECT NAME] Development Guidelines

Auto-generated from all feature plans. Last updated: [DATE]

## Active Technologies

[EXTRACTED FROM ALL PLAN.MD FILES]

## Project Structure

```text
[ACTUAL STRUCTURE FROM PLANS]
```
````

## Commands

[ONLY COMMANDS FOR ACTIVE TECHNOLOGIES]

## Code Style

[LANGUAGE-SPECIFIC, ONLY FOR LANGUAGES IN USE]

## Recent Changes

[LAST 3 FEATURES AND WHAT THEY ADDED]

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->

````

---

## .spectrena/templates/backlog-template.md

**Your version hash:** `38beb0755964`
**New version hash:** `38beb0755964`

### Diff

```diff
(diff not available)
````

### New Version Content

```markdown
# Spec Backlog

> Ordered list of specs to implement. Use `/spectrena.specify {spec-id}` to start.

## Status Legend

| Emoji | Meaning     |
| ----- | ----------- |
| ⬜    | Not started |
| 🟨    | In progress |
| 🟩    | Complete    |
| 🚫    | Blocked     |

---

## Phase 1: Foundation

### core-001-project-setup

**Scope:** Project structure, build system, tooling configuration

| Attribute      | Value                   |
| -------------- | ----------------------- |
| **Weight**     | STANDARD                |
| **Status**     | ⬜                      |
| **Depends On** | (none)                  |
| **References** | ARCH §Project Structure |

**Covers:**

- Repository structure
- Build tooling setup
- Development environment

**Does NOT cover:**

- Application implementation
- Database setup

---

### core-002-database-schema

**Scope:** Database schema and migrations setup

| Attribute      | Value               |
| -------------- | ------------------- |
| **Weight**     | STANDARD            |
| **Status**     | ⬜                  |
| **Depends On** | core-001            |
| **References** | ARCH §Database, DOM |

**Covers:**

- Schema definition
- Migration tooling
- Seed data

**Does NOT cover:**

- Application queries
- ORM setup

---

## Phase 2: Core Features

### core-003-authentication

**Scope:** User authentication system

| Attribute      | Value                     |
| -------------- | ------------------------- |
| **Weight**     | FORMAL                    |
| **Status**     | ⬜                        |
| **Depends On** | core-001 core-002         |
| **References** | REQ §Auth, ARCH §Security |

**Covers:**

- Login/logout flows
- Session management
- Password handling

**Does NOT cover:**

- OAuth providers (separate spec)
- Authorization/permissions

---

<!-- Add more specs following this pattern -->
```

---

## .spectrena/templates/tasks-template.md

**Your version hash:** `db6127360461`
**New version hash:** `db6127360461`

### Diff

```diff
(diff not available)
```

### New Version Content

````markdown
---
description: 'Task list template for feature implementation'
---

# Tasks: [FEATURE NAME]

**Input**: Design documents from `/specs/[###-feature-name]/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/`
- Paths shown below assume single project - adjust based on plan.md structure

<!--
  ============================================================================
  IMPORTANT: The tasks below are SAMPLE TASKS for illustration purposes only.

  The /spectrena.tasks command MUST replace these with actual tasks based on:
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - Feature requirements from plan.md
  - Entities from data-model.md
  - Endpoints from contracts/

  Tasks MUST be organized by user story so each story can be:
  - Implemented independently
  - Tested independently
  - Delivered as an MVP increment

  DO NOT keep these sample tasks in the generated tasks.md file.
  ============================================================================
-->

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create project structure per implementation plan
- [ ] T002 Initialize [language] project with [framework] dependencies
- [ ] T003 [P] Configure linting and formatting tools

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

Examples of foundational tasks (adjust based on your project):

- [ ] T004 Setup database schema and migrations framework
- [ ] T005 [P] Implement authentication/authorization framework
- [ ] T006 [P] Setup API routing and middleware structure
- [ ] T007 Create base models/entities that all stories depend on
- [ ] T008 Configure error handling and logging infrastructure
- [ ] T009 Setup environment configuration management

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - [Title] (Priority: P1) 🎯 MVP

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 1 (OPTIONAL - only if tests requested) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T010 [P] [US1] Contract test for [endpoint] in tests/contract/test\_[name].py
- [ ] T011 [P] [US1] Integration test for [user journey] in tests/integration/test\_[name].py

### Implementation for User Story 1

- [ ] T012 [P] [US1] Create [Entity1] model in src/models/[entity1].py
- [ ] T013 [P] [US1] Create [Entity2] model in src/models/[entity2].py
- [ ] T014 [US1] Implement [Service] in src/services/[service].py (depends on T012, T013)
- [ ] T015 [US1] Implement [endpoint/feature] in src/[location]/[file].py
- [ ] T016 [US1] Add validation and error handling
- [ ] T017 [US1] Add logging for user story 1 operations

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - [Title] (Priority: P2)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 2 (OPTIONAL - only if tests requested) ⚠️

- [ ] T018 [P] [US2] Contract test for [endpoint] in tests/contract/test\_[name].py
- [ ] T019 [P] [US2] Integration test for [user journey] in tests/integration/test\_[name].py

### Implementation for User Story 2

- [ ] T020 [P] [US2] Create [Entity] model in src/models/[entity].py
- [ ] T021 [US2] Implement [Service] in src/services/[service].py
- [ ] T022 [US2] Implement [endpoint/feature] in src/[location]/[file].py
- [ ] T023 [US2] Integrate with User Story 1 components (if needed)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - [Title] (Priority: P3)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Tests for User Story 3 (OPTIONAL - only if tests requested) ⚠️

- [ ] T024 [P] [US3] Contract test for [endpoint] in tests/contract/test\_[name].py
- [ ] T025 [P] [US3] Integration test for [user journey] in tests/integration/test\_[name].py

### Implementation for User Story 3

- [ ] T026 [P] [US3] Create [Entity] model in src/models/[entity].py
- [ ] T027 [US3] Implement [Service] in src/services/[service].py
- [ ] T028 [US3] Implement [endpoint/feature] in src/[location]/[file].py

**Checkpoint**: All user stories should now be independently functional

---

[Add more user story phases as needed, following the same pattern]

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] TXXX [P] Documentation updates in docs/
- [ ] TXXX Code cleanup and refactoring
- [ ] TXXX Performance optimization across all stories
- [ ] TXXX [P] Additional unit tests (if requested) in tests/unit/
- [ ] TXXX Security hardening
- [ ] TXXX Run quickstart.md validation

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together (if tests requested):
Task: "Contract test for [endpoint] in tests/contract/test_[name].py"
Task: "Integration test for [user journey] in tests/integration/test_[name].py"

# Launch all models for User Story 1 together:
Task: "Create [Entity1] model in src/models/[entity1].py"
Task: "Create [Entity2] model in src/models/[entity2].py"
```
````

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- **All tasks must pass linting, formatting, and type-checking (if applicable) before being marked complete**
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

````

---

## .spectrena/templates/vscode-settings.json

**Your version hash:** `23b875cddedb`
**New version hash:** `23b875cddedb`

### Diff

```diff
(diff not available)
````

### New Version Content

```markdown
{
"chat.promptFilesRecommendations": {
"spectrena.constitution": true,
"spectrena.specify": true,
"spectrena.plan": true,
"spectrena.tasks": true,
"spectrena.implement": true
},
"chat.tools.terminal.autoApprove": {
".spectrena/scripts/bash/": true,
".spectrena/scripts/powershell/": true
}
}
```

---

## .spectrena/templates/spec-template.md

**Your version hash:** `3ade084645cd`
**New version hash:** `3ade084645cd`

### Diff

```diff
(diff not available)
```

### New Version Content

```markdown
# Specification: {FEATURE_TITLE}

<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<!--
SPEC WEIGHT: [ ] LIGHTWEIGHT  [ ] STANDARD  [x] FORMAL

Weight Guidelines:
- LIGHTWEIGHT: Bug fixes, small enhancements, config changes (<2 days work)
  Required sections: Quick Reference, Problem/Solution, Requirements, Acceptance Criteria

- STANDARD: New features, integrations, moderate complexity (2-10 days work)
  Required sections: All LIGHTWEIGHT + Data Model, Security, Test Requirements

- FORMAL: Major systems, compliance-sensitive, cross-team impact (>10 days work)
  Required sections: All sections, full sign-off

Mark your weight above, then delete non-required sections for lighter specs.

CLARIFICATION MARKERS:
Use [NEEDS CLARIFICATION: description] for decisions requiring stakeholder input.
These markers trigger the /spectrena.clarify workflow.
Maximum 3 markers per spec—use only for critical ambiguities.
Remove markers once resolved (move decision to Key Decisions table).
-->
<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->

**Spec ID**: {SPEC_ID}
**Component**: {COMPONENT}
**Weight**: {WEIGHT}
**Version**: 1.0
**Status**: DRAFT | IN REVIEW | APPROVED | IMPLEMENTING | COMPLETE
**Created**: {DATE}
**Author**: [Name]

---

## Quick Reference

> One-paragraph executive summary. Write this last.

**What**: [Single sentence: what capability this adds]
**Why**: [Single sentence: what problem this solves]
**Impact**: [Single sentence: measurable outcome when complete]

**Success Metrics**:

| Metric             | Target  | How Measured |
| ------------------ | ------- | ------------ |
| [Primary metric]   | [Value] | [Method]     |
| [Secondary metric] | [Value] | [Method]     |

---

## Problem Statement

### Current State

[2-3 paragraphs describing the current situation. What exists today? What pain points do users experience? Include concrete examples.]

### Desired State

[2-3 paragraphs describing the target state. What will be true when this is complete? How will the user experience change?]

### Why Now

[1 paragraph: Why is this the right time to solve this? What's the cost of delay? What enables us to solve it now?]

---

## Solution Overview

### Approach

[2-4 paragraphs describing the high-level solution approach. Stay at the "what" level—implementation details belong in plan.md.]

### Scope

**In Scope**:

- [Capability or behavior that IS included]
- [Capability or behavior that IS included]
- [Capability or behavior that IS included]

**Out of Scope**:

- [Capability explicitly NOT included] — [Why excluded or where it's tracked]
- [Capability explicitly NOT included] — [Why excluded or where it's tracked]

**Future Considerations**:

- [Capability that may be added later] — [Conditions for inclusion]

### Key Decisions

<!--
Record significant decisions made during spec development.
Move resolved [NEEDS CLARIFICATION] items here with their resolution.
-->

| Decision           | Options Considered   | Rationale         |
| ------------------ | -------------------- | ----------------- |
| [What was decided] | [Option A, Option B] | [Why this choice] |

---

## Requirements

### Functional Requirements

<!--
Priority levels:
- CRITICAL: System unusable without this
- HIGH: Major functionality gap without this
- MEDIUM: Important but workarounds exist
- LOW: Nice to have

Use [NEEDS CLARIFICATION: ...] in Notes column for ambiguous requirements.
Example: [NEEDS CLARIFICATION: auth method - email/password vs SSO vs OAuth?]
-->

| ID     | Requirement           | Priority | Notes |
| ------ | --------------------- | -------- | ----- |
| FR-001 | [The system shall...] | CRITICAL |       |
| FR-002 | [The system shall...] | HIGH     |       |
| FR-003 | [The system shall...] | MEDIUM   |       |

### Non-Functional Requirements

| ID      | Requirement                                        | Priority | Notes |
| ------- | -------------------------------------------------- | -------- | ----- |
| NFR-001 | [Performance: Response time < Xms for Y operation] |          |       |
| NFR-002 | [Availability: System available 99.X% of time]     |          |       |
| NFR-003 | [Scalability: Support N concurrent users]          |          |       |

### User Stories

<!--
Format:
- **As** a [role],
- **I** need to
  - [action]
  - ...
- **so** that [outcome].
Each story needs testable acceptance criteria.
Stories map to tasks during /spectrena.tasks phase.
-->

**US-001: [Story Title]**

- **As** a [role],
- **I** need to
  - [action]
  - ...
- **so** that [outcome].

Acceptance:

- [ ] [Specific testable condition]
- [ ] [Specific testable condition]

Independent Test: [How this can be tested in isolation]

**US-002: [Story Title]**

- **As** a [role],
- **I** need to
  - [action]
  - ...
- **so** that [outcome].

Acceptance:

- [ ] [Specific testable condition]
- [ ] [Specific testable condition]

Independent Test: [How this can be tested in isolation]

---

## Data Model

<!--
SPEC BOUNDARY: Define WHAT data exists and business rules.
PLAN BOUNDARY: Define HOW it's stored (schemas, indexes, SQL).

Key Entities format matches spectrena convention for /spectrena.plan parsing.
-->

### Key Entities

<!--
List entities with business meaning only. No implementation details.
Format: [Entity]: [What it represents, key attributes without types]
-->

- **[Entity 1]**: [What it represents, key attributes without implementation]
- **[Entity 2]**: [What it represents, relationships to other entities]
- **[Entity 3]**: [What it represents, lifecycle notes]

### Entity Details

<!-- Expand each entity. Delete this section for LIGHTWEIGHT specs. -->

**[Entity Name]**

- **Purpose**: [What this entity represents in business terms]
- **Key Attributes**: [List essential attributes and their business meaning—no types]
- **Relationships**: [How this relates to other entities]
- **Lifecycle**: [When created, what triggers updates, when deleted]
- **Business Rules**:
  - [Rule about this entity]
  - [Rule about this entity]
- **Open Questions**: [NEEDS CLARIFICATION: retention policy not specified] _(if any)_

**[Entity Name]**

- **Purpose**: [What this entity represents]
- **Key Attributes**: [Essential attributes]
- **Relationships**: [Related entities]

### State Transitions

<!-- Include if entity has meaningful state. Delete otherwise. -->
```

[State diagram in text or mermaid - keep simple]

DRAFT → ACTIVE → COMPLETED
↓
CANCELLED

````

**Transition Rules**:

- DRAFT → ACTIVE: [What must be true]
- ACTIVE → COMPLETED: [What must be true]
- ACTIVE → CANCELLED: [What must be true]

---

## Interfaces

<!--
SPEC BOUNDARY: Define WHAT operations exist, inputs/outputs, behavior.
PLAN BOUNDARY: Define HOW they're implemented (REST/GraphQL, specific endpoints, schemas).

Operations here become API contracts in plan.md.
-->

### Operations

<!--
Define operations at business level. No HTTP verbs, no JSON schemas.
Implementation details (REST paths, request/response bodies) go in plan.md contracts/.
-->

**[Operation Name]**

- **Purpose**: [What business function this enables]
- **Trigger**: [What initiates this operation]
- **Inputs**:
  - `[input]` (required): [Business meaning, valid values]
  - `[input]` (optional): [Business meaning, default behavior]
- **Outputs**: [What information is returned]
- **Behavior**:
  - [What happens during normal execution]
  - [Side effects or state changes]
- **Error Conditions**:
  - [Condition]: [Expected behavior]
  - [Condition]: [Expected behavior]

**[Operation Name]**

- **Purpose**: [Business function]
- **Inputs**: [Key inputs]
- **Outputs**: [Key outputs]
- **Behavior**: [Summary]

### Integration Points

<!-- External systems this feature interacts with -->

| System     | Direction | Purpose                    | Data Exchanged |
| ---------- | --------- | -------------------------- | -------------- |
| [System A] | Inbound   | [Why we receive from them] | [What data]    |
| [System B] | Outbound  | [Why we send to them]      | [What data]    |

---

## Workflows

### [Primary Workflow Name]

**Actors**: [Who participates]
**Preconditions**: [What must be true before starting]
**Postconditions**: [What is true after completion]

```mermaid
sequenceDiagram
    participant U as User
    participant S as System
    participant E as External

    U->>S: [Action]
    S->>E: [Request]
    E-->>S: [Response]
    S-->>U: [Result]
````

**Steps**:

1. [Actor] [does what] → [result]
2. [Actor] [does what] → [result]
3. [Actor] [does what] → [result]

**Alternate Flows**:

- At step [N], if [condition]: [what happens instead]

**Error Flows**:

- If [error condition]: [system behavior, user sees what]

---

## Security and Compliance

### Authorization

| Operation   | Required Permission  | Notes |
| ----------- | -------------------- | ----- |
| [Operation] | [Role or permission] |       |
| [Operation] | [Role or permission] |       |

### Data Classification

| Data Element | Classification                            | Handling Requirements |
| ------------ | ----------------------------------------- | --------------------- |
| [Data type]  | [Public/Internal/Confidential/Restricted] | [Special handling]    |

### Compliance Requirements

<!-- Delete sections that don't apply -->

**GDPR/HIPAA/ITAR/EAR/ETC** (if applicable):

- [ ] All data stored in compliant infrastructure
- [ ] Access limited to approved persons
- [ ] Export controls documented
- [ ] [Specific requirement]

**Audit Requirements**:

- Events to log: [List events requiring audit trail]
- Retention: [How long audit logs retained]
- Access: [Who can access audit logs]

**Data Protection**:

- PII handling: [Requirements]
- Encryption: [At rest, in transit requirements]
- Retention/deletion: [Policy]

---

## Test Requirements

### Success Criteria

<!--
Measurable outcomes that define "done".
Format matches spectrena SC-XXX convention for traceability.
These become validation targets in /spectrena.plan quickstart.md.
-->

| ID     | Criterion                                                                                            | Measurement    |
| ------ | ---------------------------------------------------------------------------------------------------- | -------------- |
| SC-001 | [Measurable metric, e.g., "Users can complete account creation in under 2 minutes"]                  | [How verified] |
| SC-002 | [Measurable metric, e.g., "System handles 1000 concurrent users without degradation"]                | [How verified] |
| SC-003 | [User satisfaction metric, e.g., "90% of users successfully complete primary task on first attempt"] | [How verified] |
| SC-004 | [Business metric, e.g., "Reduce support tickets related to X by 50%"]                                | [How verified] |

### Acceptance Criteria

<!--
Gherkin format for unambiguous testing.
Maps to user stories above.
-->

**Scenario**: [User action or workflow name] _(maps to US-001)_

```gherkin
Given [initial state]
When [action taken]
Then [expected result]
And [additional verification]
```

**Scenario**: [Another scenario] _(maps to US-002)_

```gherkin
Given [initial state]
When [action taken]
Then [expected result]
```

### Test Scenarios

<!-- Key scenarios that must be tested. Detailed test cases go in plan.md. -->

| ID     | Scenario                  | Type        | Priority | Maps To |
| ------ | ------------------------- | ----------- | -------- | ------- |
| TS-001 | [Happy path description]  | Functional  | CRITICAL | US-001  |
| TS-002 | [Error handling scenario] | Functional  | HIGH     | FR-003  |
| TS-003 | [Performance under load]  | Performance | HIGH     | NFR-001 |
| TS-004 | [Security boundary test]  | Security    | CRITICAL | NFR-002 |

### Performance Criteria

| Operation   | Metric        | Target    | Conditions        |
| ----------- | ------------- | --------- | ----------------- |
| [Operation] | Response time | < [X]ms   | [Load conditions] |
| [Operation] | Throughput    | > [X]/sec | [Conditions]      |

---

## Constraints and Assumptions

### Technical Constraints

- [Constraint]: [Impact on solution]
- [Constraint]: [Impact on solution]

### Business Constraints

- [Timeline, budget, resource constraint]
- [Policy or process constraint]

### Assumptions

<!-- Things we're assuming are true. If wrong, revisit spec. -->

- [Assumption about users, systems, or environment]
- [Assumption about dependencies]

### Dependencies

| Dependency          | Type                   | Status   | Impact if Delayed |
| ------------------- | ---------------------- | -------- | ----------------- |
| [What we depend on] | [System/Team/External] | [Status] | [Impact]          |

### Risks

| Risk                  | Likelihood | Impact | Mitigation          |
| --------------------- | ---------- | ------ | ------------------- |
| [What could go wrong] | H/M/L      | H/M/L  | [How we address it] |

---

## Open Questions

<!--
Track unresolved questions here AND inline with [NEEDS CLARIFICATION: ...] markers.
The /spectrena.clarify command scans for markers and uses this table.
Move resolved items to Key Decisions table with resolution rationale.

Maximum 4 [NEEDS CLARIFICATION] markers in spec—more indicates scope creep.
-->

| #   | Question                      | Location            | Owner         | Due    | Status |
| --- | ----------------------------- | ------------------- | ------------- | ------ | ------ |
| 1   | [Question needing resolution] | [Section or FR-XXX] | [Who decides] | [Date] | OPEN   |
| 2   | [Question needing resolution] | [Section or FR-XXX] | [Who decides] | [Date] | OPEN   |

### Clarifications Log

<!--
Record resolved clarifications here before moving to Key Decisions.
This creates an audit trail of how ambiguities were resolved.
-->

| Date | Question | Resolution | Decided By |
| ---- | -------- | ---------- | ---------- |
|      |          |            |            |

---

## References

### Internal

- [Link to related spec]
- [Link to architecture doc]
- [Link to prior art]

### External

- [Link to standard or regulation]
- [Link to vendor documentation]

---

## Approval

<!-- FORMAL weight requires sign-off. STANDARD/LIGHTWEIGHT may skip. -->

| Role                 | Name | Date | Status  |
| -------------------- | ---- | ---- | ------- |
| Author               |      |      | DRAFT   |
| Technical Review     |      |      | PENDING |
| Security Review      |      |      | PENDING |
| Product Owner        |      |      | PENDING |
| Compliance (if ITAR) |      |      | PENDING |

---

## Changelog

| Version | Date   | Author | Changes               |
| ------- | ------ | ------ | --------------------- |
| 1.0     | [Date] | [Name] | Initial specification |

---

<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<!--
SPECTRENA WORKFLOW INTEGRATION

This template is compatible with spectrena's /spectrena.\* commands:

1. /spectrena.specify — Creates this spec from feature description
   - Fills in sections from user input
   - Adds [NEEDS CLARIFICATION] markers for ambiguities
   - Creates checklists/requirements.md for validation

2. /spectrena.clarify — Resolves [NEEDS CLARIFICATION] markers
   - Scans spec for markers (max 3 recommended)
   - Prompts for resolution
   - Updates Clarifications Log and Key Decisions

3. /spectrena.plan — Creates implementation plan from this spec
   - Reads Key Entities → generates data-model.md
   - Reads Operations → generates contracts/api-spec.json
   - Reads Success Criteria → generates quickstart.md

4. /spectrena.tasks — Breaks plan into executable tasks
   - User Stories (US-XXX) map to task groups
   - Requirements (FR-XXX) map to acceptance criteria
   - Test Scenarios (TS-XXX) map to verification tasks

5. /spectrena.implement — Executes tasks with code generation

SECTION MAPPING:

- Quick Reference → plan.md header
- Key Entities → data-model.md
- Operations → contracts/
- Success Criteria → quickstart.md validation
- User Stories → tasks.md task groups
- Requirements → task acceptance criteria

================================================================================
SPECTRENA INTEGRATION (if using Spectrena lineage tracking)
================================================================================

Register this spec for lineage tracking:

spec_register(
spec_id="[PROJECT]-SPEC-[NNN]",
title="[Feature Name]",
spec_path="specs/[NNN]-[slug]/spec.md"
)

After /spectrena.plan, register the plan:

plan_register(
plan_id="[spec_id]:plan",
spec_id="[spec_id]",
plan_path="specs/[NNN]-[slug]/plan.md",
tech_stack={"backend": "...", "db": "..."}
)

After /spectrena.tasks, register each task:

task_register(
task_id="[spec_id]:task-01",
plan_id="[spec_id]:plan",
title="[Task title from tasks.md]"
)

Implement with full traceability:

/spectrena.implement [task_id]

Lineage enables:

- FR-XXX → task → code change tracing
- "What implemented this requirement?" queries
- "What spec drove this code?" reverse lookups
-->
<!-- markdownlint-enable -->
<!-- prettier-ignore-end -->

````

---

## .spectrena/templates/plan-template.md

**Your version hash:** `6149c63fdd75`
**New version hash:** `6149c63fdd75`

### Diff

```diff
(diff not available)
````

### New Version Content

````markdown
# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/spectrena.plan` command. See `.spectrena/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS CLARIFICATION]
**Primary Dependencies**: [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION]
**Storage**: [if applicable, e.g., PostgreSQL, CoreData, files or N/A]
**Testing**: [e.g., pytest, XCTest, cargo test or NEEDS CLARIFICATION]
**Target Platform**: [e.g., Linux server, iOS 15+, WASM or NEEDS CLARIFICATION]
**Project Type**: [single/web/mobile - determines source structure]
**Performance Goals**: [domain-specific, e.g., 1000 req/s, 10k lines/sec, 60 fps or NEEDS CLARIFICATION]
**Constraints**: [domain-specific, e.g., <200ms p95, <100MB memory, offline-capable or NEEDS CLARIFICATION]
**Scale/Scope**: [domain-specific, e.g., 10k users, 1M LOC, 50 screens or NEEDS CLARIFICATION]

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

[Gates determined based on constitution file]

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/spectrena.plan command output)
├── research.md          # Phase 0 output (/spectrena.plan command)
├── data-model.md        # Phase 1 output (/spectrena.plan command)
├── quickstart.md        # Phase 1 output (/spectrena.plan command)
├── contracts/           # Phase 1 output (/spectrena.plan command)
└── tasks.md             # Phase 2 output (/spectrena.tasks command - NOT created by /spectrena.plan)
```
````

### Source Code (repository root)

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation                  | Why Needed         | Simpler Alternative Rejected Because |
| -------------------------- | ------------------ | ------------------------------------ |
| [e.g., 4th project]        | [current need]     | [why 3 projects insufficient]        |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient]  |

````

---

## .spectrena/templates/checklist-template.md

**Your version hash:** `b4b99b7f7d87`
**New version hash:** `b4b99b7f7d87`

### Diff

```diff
(diff not available)
````

### New Version Content

```markdown
# [CHECKLIST TYPE] Checklist: [FEATURE NAME]

**Purpose**: [Brief description of what this checklist covers]
**Created**: [DATE]
**Feature**: [Link to spec.md or relevant documentation]

**Note**: This checklist is generated by the `/spectrena.checklist` command based on feature context and requirements.

<!--
  ============================================================================
  IMPORTANT: The checklist items below are SAMPLE ITEMS for illustration only.

  The /spectrena.checklist command MUST replace these with actual items based on:
  - User's specific checklist request
  - Feature requirements from spec.md
  - Technical context from plan.md
  - Implementation details from tasks.md

  DO NOT keep these sample items in the generated checklist file.
  ============================================================================
-->

## [Category 1]

- [ ] CHK001 First checklist item with clear action
- [ ] CHK002 Second checklist item
- [ ] CHK003 Third checklist item

## [Category 2]

- [ ] CHK004 Another category item
- [ ] CHK005 Item with specific criteria
- [ ] CHK006 Final item in this category

## Notes

- Check items off as completed: `[x]`
- Add comments or findings inline
- Link to relevant resources or documentation
- Items are numbered sequentially for easy reference
```

---
