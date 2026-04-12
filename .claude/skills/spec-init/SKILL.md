# Skill: Initialize Spec Templates

## Purpose
Generate the Spec-Driven Design template set for a project. Two modes: **greenfield** (blank templates pre-filled from project context) and **backfill** (convert existing documentation into template format). Output goes to `docs/specs/` with the `NN-TYPE-NNN-name` naming convention.

## When to use
- Setting up a new project that will use spec-driven development
- Converting existing project documentation into the template format
- User says "init specs", "create spec templates", "set up spec docs", "convert docs to specs"

## Prerequisites
- Project CLAUDE.md should exist (run `/setup` first if not)
- For backfill mode: existing documentation to convert

## Workflow

### Step 1: Detect project context
1. Read `CLAUDE.md` for project overview, tech stack, architecture, conventions
2. Scan for existing documentation:
   - `docs/` directory (any existing docs)
   - `README.md` (project description)
   - Source code structure (for domain model inference)
   - `Context/Features/` (existing Cortex specs)
3. Check if `docs/specs/` already exists (avoid clobbering)

### Step 2: Determine mode

Present findings and ask the user which mode to use:

**Greenfield** (no existing docs found, or user prefers blank templates):
- Generates all templates with `{PLACEHOLDER}` markers
- Pre-fills project name, tech stack, and directory structure from CLAUDE.md
- User fills in the content manually or with AI assistance

**Backfill** (existing docs found):
- Reads existing documentation and maps content to template sections
- Generates filled templates with content extracted from source docs
- Flags gaps where existing docs don't cover a template section
- Preserves original docs (does not modify or delete them)

**Selective** (user picks which templates to generate):
- User selects specific templates from the full set
- Useful when only some templates are needed (e.g., just PRDs + invariants)

### Step 3: Create output directory

```
mkdir -p docs/specs
```

### Step 4: Generate templates (per mode)

#### Greenfield Mode

Generate each template with project-specific pre-fills from CLAUDE.md:

1. **`00-project-overview.md`** -- Pre-fill: project name, tech stack table, platform list
2. **`01-PRD-001-{primary-feature}.md`** -- Pre-fill: project name in headers. If CLAUDE.md describes a primary feature, use that as the first PRD name. Otherwise use `core`.
3. **`02-domain-model.md`** -- Pre-fill: scan source code for entity/model classes and list them as starting points in the entity hierarchy
4. **`03-invariants.md`** -- Pre-fill: project name. Add placeholder invariant categories based on detected stacks (e.g., "Sync Invariants" if a sync layer exists)
5. **`04-architecture.md`** -- Pre-fill: architecture pattern from CLAUDE.md, module names from project structure, detected stacks in the layer diagrams
6. **`05-erd.md`** -- Pre-fill: if database schema files exist (migrations, models), extract table names and columns as starting points
7. **`06-state-machines.md`** -- Pre-fill: scan for enum/sealed class definitions that suggest state machines
8. **`07-user-flows.md`** -- Pre-fill: project name, platform names
9. **`10-PLAN-001-{project-name}.md`** -- Pre-fill: module names as steps, detected stacks for parallel track table

Skip `08-SUB-*` and `09-PLAT-*` unless the project clearly has subsystems or secondary platforms.

For each generated file:
- Replace `{PROJECT_NAME}` with the actual project name
- Replace `{PLACEHOLDER}` markers with project-specific hints where possible
- Keep `<!-- GUIDANCE: -->` comments intact for the author
- Delete `<!-- CORTEX INTEGRATION: -->` comments only if Cortex is not installed

#### Backfill Mode

Read existing docs and map their content to template sections. The mapping depends on what exists:

**From existing PRDs / requirements docs:**

| Source Content | Target Template | Mapping |
|---------------|----------------|---------|
| Feature overview / problem statement | `01-PRD-NNN` Overview, Problem Statement | Direct copy, reformat |
| User stories / personas | `01-PRD-NNN` User Personas, Use Cases | Expand into sequence diagrams |
| Requirements lists | `01-PRD-NNN` Functional Requirements | Assign FR IDs, set priorities |
| Acceptance criteria | `01-PRD-NNN` Testable Assertions | Convert to A-NNN format |
| UI mockups / wireframes (described) | `07-user-flows.md` Screen flows | Convert to Mermaid flowcharts |
| Data models / entities | `02-domain-model.md` Entity Hierarchy | Extract into class diagrams |
| Business rules | `03-invariants.md` | Assign IDs, add enforcement column |
| Architecture decisions | `04-architecture.md` Key Decisions | Mark as ADR-CANDIDATE |
| Database schemas | `05-erd.md` | Convert to Mermaid ERD + SQL |
| State diagrams / lifecycles | `06-state-machines.md` | Convert to Mermaid stateDiagram |
| Implementation steps | `10-PLAN-NNN` | Add agent assignments, milestones |

**From Cortex Context/Features/ specs:**

If the project already has Cortex feature specs in `Context/Features/`:

| Cortex Artifact | Target Template | Mapping |
|----------------|----------------|---------|
| Spec.md Requirements (Must/Should/Won't) | `01-PRD-NNN` FRs (P0/P1/P2) | Priority translation |
| Spec.md Testable Assertions | `01-PRD-NNN` Testable Assertions | Copy directly, preserve IDs |
| Tech.md Architecture Overview | `04-architecture.md` relevant section | Merge into project-wide doc |
| Tech.md Key Decisions | `04-architecture.md` Key Decisions | Add ADR-CANDIDATE markers |
| Steps.md Team Orchestration | `10-PLAN-NNN` Cortex Appendix | Extract agent assignments |
| Steps.md Contracts | `05-erd.md` Contract Surfaces | Extract contract paths |
| ADRs in Context/Decisions/ | `04-architecture.md` Key Decisions | Back-reference ADR IDs |

**From source code (supplement, not primary):**

Scan source code to fill gaps the docs don't cover:
- Model/entity classes -> `02-domain-model.md` entity list
- Enum/sealed class definitions -> `06-state-machines.md` state lists
- Database migration files -> `05-erd.md` table schemas
- Route/endpoint definitions -> `01-PRD-NNN` use cases (API-level)
- Test files -> infer invariants from test assertions

Source code scanning is supplemental. Never generate a full template purely from code -- always flag sections that need human review.

### Step 5: Cross-reference validation

After generating templates, validate cross-references:

1. **Assertion coverage**: Every P0 requirement in PRDs has at least one assertion (A-NNN)
2. **Invariant-to-assertion mapping**: Every invariant in `03-invariants.md` maps to at least one assertion in a PRD
3. **Entity consistency**: Entities in `02-domain-model.md` match tables in `05-erd.md`
4. **State machine coverage**: Every entity with a `phase` or `status` field has an entry in `06-state-machines.md`
5. **Contract surfaces populated**: `05-erd.md` Contract Surfaces table has entries for key interfaces
6. **Document index**: `00-project-overview.md` Document Index lists all generated files

Report any gaps found:
```
Cross-Reference Report:
  [PASS] 12/12 P0 requirements have assertions
  [WARN] 2 invariants missing assertion mappings: H-3, C-4
  [WARN] Entity "RoutineExecution" in domain model but not in ERD
  [PASS] All stateful entities have state machines
  [WARN] Contract Surfaces table is empty -- fill after architecture solidifies
```

### Step 6: Generate gap report

Create `docs/specs/GAPS.md` listing sections that need human attention:

```markdown
# Spec Template Gaps

Generated by `/spec-init` on YYYY-MM-DD.
Review and fill these sections before starting implementation.

## Empty Sections (need content)
- [ ] `01-PRD-001-core.md` > Problem Statement -- needs 2-3 sentences
- [ ] `02-domain-model.md` > Value Objects -- no value objects identified
- [ ] `04-architecture.md` > Deployment Architecture -- not covered in existing docs

## Placeholder Content (needs review)
- [ ] `02-domain-model.md` > Entity Hierarchy -- inferred from source code, verify accuracy
- [ ] `05-erd.md` > Indices -- generated from query patterns, verify performance

## Cross-Reference Issues
- [ ] Invariant H-3 has no corresponding assertion in any PRD
- [ ] Entity "RoutineExecution" in domain model but missing from ERD

## Recommended Next Steps
1. Fill empty sections marked above
2. Review placeholder content for accuracy
3. Run `/blueprint` for first feature -- plan-from-specs will use these docs
```

### Step 7: Present summary

Present to the user:
- Files generated (count and list)
- Mode used (greenfield/backfill/selective)
- Cross-reference validation results
- Gap report location
- Recommended next steps

---

## Template Source

This skill generates templates based on the Spec-Driven Design template set structure. Each template follows a defined section structure.

If template source files exist at `docs/specs/` already (e.g., the user downloaded the template set), the skill reads them as scaffolding and fills in project-specific content. If no templates exist, the skill generates from the structure definitions below.

### Template Structures

Each template's required sections (used for both generation and validation):

**00-project-overview.md** (singleton):
Executive Summary, Philosophy, Design Principles, Target Users, Platform Strategy, Success Metrics, Document Index

**01-PRD-NNN-{name}.md** (multi-instance):
Overview, Problem Statement, Goals (P0/P1/P2), Key Concepts, User Personas, Use Cases (with sequence diagrams), Testable Assertions, Functional Requirements (with FR-IDs), Non-Functional Requirements, UI Requirements, Data Requirements, Invariants, State Machine, Integration Points, Success Metrics, Dependencies, Open Questions

**02-domain-model.md** (singleton):
Bounded Contexts (with Agent Type column), Entity Hierarchy, Aggregate Roots, Core Entities (with ERDs), Value Objects, Enumerations, Domain Events, Relationships Summary, Consistency Rules, Query Patterns

**03-invariants.md** (singleton):
Invariant Categories, Entity Invariants (per entity), Behavioral Invariants, Domain Invariants, System Invariants, Summary Table, Invariant-to-Assertion Mapping, Test Case Patterns

**04-architecture.md** (singleton):
High-Level Architecture, Layer Responsibilities, Module Structure, Data Flow (write + sync), Background Processing, Key Architecture Decisions (with ADR-CANDIDATE markers), Dependency Injection, Error Handling Strategy, Security Architecture, Performance Considerations, Deployment Architecture

**05-erd.md** (singleton):
Complete ERD, Local Database Schema, Indices, Remote Schema (if applicable), Data Type Mappings, Migration Strategy, Query Examples, Contract Surfaces

**06-state-machines.md** (singleton):
Primary Entity Lifecycle, Entity Status, Child Entity Types, Process Execution, Recovery/Intervention Sessions, Notification State, Implementation Patterns (sealed class + engine), Transition Summary Tables

**07-user-flows.md** (singleton):
Flow Index, First Launch/Onboarding, Primary Creation Flow, Primary Screen, Action Detail Flow, Multi-Step Process Flow, Recovery/Edge Case Flow, Settings, Error States, Empty States, Interaction Specs, Animations, Haptics, Accessibility

**08-SUB-NNN-{name}.md** (multi-instance):
Philosophy, Categories/Channels, Types (with message templates), Message Guidelines, Scheduling Architecture, Action Handling, User Preferences, Platform Behavior, Technical Implementation, Metrics

**09-PLAT-NNN-{name}.md** (multi-instance):
Philosophy, Constraints, Feature Scope, Components, Surface Designs, App Screens, Data Sync Architecture, Offline Behavior, Subsystem Behavior, UI Components, Performance, Implementation Notes, Testing

**10-PLAN-NNN-{name}.md** (multi-instance):
Dependency Graph, Parallel Tracks, Shared Module Strategy, Steps (with What to build + Done when), Cortex Appendix (Steps.md template, Contract Surface Index, Complexity Assessment)

---

## Rules

- Never overwrite existing files in `docs/specs/` without explicit user permission
- Always preserve original source docs -- backfill copies content, never moves or deletes
- Flag all inferred content with `<!-- INFERRED: verify this -->` comments so the user knows what to review
- In backfill mode, prefer existing doc content over source code inference when both are available
- Generate Mermaid diagrams for all sections that call for them -- do not leave diagram placeholders empty
- Assign sequential IDs (A-001, FR-1.1, H-1) starting from 001 -- do not skip numbers
- If a template section has no content to fill, include the section header with a `<!-- TODO: ... -->` comment rather than omitting it entirely
- Cross-reference validation runs after all files are generated, not per-file
- The GAPS.md file is always generated, even if no gaps are found (report "All sections populated")
- For multi-instance templates (PRD, SUB, PLAT, PLAN), ask the user how many instances to create and what to name them
- Template files use the `NN-TYPE-NNN-name.md` naming convention: singletons get `NN-name.md`, multi-instance get `NN-TYPE-NNN-name.md`
