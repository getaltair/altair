# Skill: Plan From Existing Specs

## Purpose
When a project has deep specification documents (from the Spec-Driven Design template set), this skill enhances the Cortex planning phases by reading relevant spec docs instead of generating content from scratch. It acts as a bridge between persistent project-wide specs and Cortex's per-feature execution artifacts.

## When to use
- During `/blueprint` Phase 1 (plan-spec) when `docs/specs/01-PRD-*` files exist
- During `/blueprint` Phase 2 (plan-research) when `docs/specs/04-architecture.md` exists
- During `/blueprint` Phase 3 (plan-steps) when `docs/specs/10-PLAN-*` files exist
- Triggered automatically by plan-spec, plan-research, and plan-steps skills when spec docs are detected

## Detection

Check for spec templates in the project. Search these directories in order:
1. `docs/specs/`
2. `specs/`
3. `doc/specs/`
4. Project root

Look for these files:

```
00-project-overview.md        # Project overview
01-PRD-NNN-{name}.md          # Feature PRDs (may be multiple)
02-domain-model.md            # Domain model
03-invariants.md              # Business invariants
04-architecture.md            # System architecture
05-erd.md                     # Entity-relationship diagrams
06-state-machines.md          # State machines
07-user-flows.md              # User flows
08-SUB-NNN-{name}.md          # Subsystem designs
09-PLAT-NNN-{name}.md         # Platform integrations
10-PLAN-NNN-{name}.md         # Implementation plans
```

If none exist, fall back to standard Cortex planning (generate from scratch).

---

## Phase 1 Enhancement: Spec.md from PRDs

When generating Spec.md during `/blueprint` Phase 1:

1. **Find the relevant PRD**: Match the feature description against `01-PRD-*` filenames and content
2. **Extract requirements**: Pull P0 requirements as "Must Have", P1 as "Should Have", P2 as "Won't Have (this iteration)"
3. **Copy testable assertions**: The PRD's Testable Assertions table maps directly to Spec.md assertions. Preserve assertion IDs (A-001, A-002, etc.) for traceability
4. **Pull invariants**: Read `03-invariants.md` and include invariants relevant to this feature's entities as additional assertions or constraints
5. **Note open questions**: Carry forward any Open Questions from the PRD

**What still needs to be generated:**
- Problem Statement (summarize from PRD overview)
- User Stories (synthesize from PRD use cases)
- Feature-specific context not in the PRD

**Mapping table:**

| PRD Section | Spec.md Section |
|-------------|----------------|
| Overview | Overview + Problem Statement |
| Use Cases | User Stories |
| FR-X.X (P0) | Must Have requirements |
| FR-X.X (P1) | Should Have requirements |
| FR-X.X (P2) | Won't Have |
| Testable Assertions | Testable Assertions (copy directly) |
| Invariants (03-invariants.md) | Additional assertions or constraints |
| Open Questions | Open Questions |

---

## Phase 2 Enhancement: Tech.md from Architecture + ERD

When generating Tech.md during `/blueprint` Phase 2:

1. **Read architecture doc**: `04-architecture.md` provides the system architecture, module structure, data flow patterns, and error handling strategy
2. **Extract ADR candidates**: Look for `<!-- ADR-CANDIDATE -->` markers in the architecture doc. These decisions should be formalized as ADRs during this phase
3. **Read ERD for data details**: `05-erd.md` provides table schemas, indices, remote schemas, and query patterns relevant to this feature
4. **Read state machines**: `06-state-machines.md` provides entity lifecycle definitions that inform the technical approach
5. **Read subsystem designs**: Any `08-SUB-*` files relevant to this feature provide deep technical context

**What still needs to be generated:**
- Feature-specific architecture decisions not in the project-wide docs
- Integration points specific to this feature
- Risks and unknowns specific to this implementation

**ADR workflow from architecture doc:**

For each `<!-- ADR-CANDIDATE -->` marker in `04-architecture.md` that relates to this feature:
1. Check if an ADR already exists in `Context/Decisions/` for this decision
2. If not, create one using the options/rationale from the architecture doc
3. Consult the Advisor tool if the decision involves complex trade-offs

---

## Phase 3 Enhancement: Steps.md from Implementation Plan

When generating Steps.md during `/blueprint` Phase 3:

1. **Find the relevant step**: Match the feature to a STEP in `10-PLAN-*` files
2. **Use the step's structure**: The "What to build" bullet list becomes implementation tasks (S001-S999)
3. **Use the step's done criteria**: "Done when" checklist items become acceptance criteria
4. **Pull agent assignments**: Check `02-domain-model.md` Context Definitions table for the Agent Type column to determine which specialist handles each domain area
5. **Pull contract surfaces**: Read the "Contract Surfaces" table in `05-erd.md` to determine what files to declare as contracts at milestone boundaries
6. **Pull complexity assessment**: Read the "Complexity Assessment" table in the implementation plan appendix to determine the right execution route (/quick vs /impl vs /team-impl)
7. **Generate assertion references**: At each milestone boundary, reference the assertion IDs from the PRD that should be verified at that checkpoint
8. **Generate test tasks from invariants**: Read `03-invariants.md` Invariant-to-Assertion Mapping and Test Case Pattern tables to generate S###-T test scenarios

**What still needs to be generated:**
- Exact task decomposition from the step's high-level build list
- Dependency ordering between tasks
- Parallel execution flags
- Test scenarios beyond what invariants provide

---

## Spec Doc Loading for Agent Prompts

When `/impl` renders agent prompts (Step 5a in impl.md), the orchestrator should inject relevant spec doc content alongside the Spec.md/Tech.md excerpts:

| Agent Domain | Spec Docs to Inject |
|-------------|-------------------|
| Backend / data layer | `02-domain-model.md` (entities), `05-erd.md` (schemas), `03-invariants.md` (relevant invariants) |
| Frontend / UI | `07-user-flows.md` (screens, interactions), `06-state-machines.md` (UI states) |
| Validation / QA | `03-invariants.md` (all invariants), `06-state-machines.md` (transition rules) |
| Platform-specific | Relevant `09-PLAT-*` doc |
| Subsystem work | Relevant `08-SUB-*` doc |

Keep injected content focused -- only include sections relevant to the agent's assigned tasks. Do not dump entire spec docs into agent prompts.

---

## Integration Checklist

When this skill detects spec docs, it should:

- [ ] Note which spec docs were found in the planning output
- [ ] Cross-reference PRD requirement IDs in Spec.md
- [ ] Preserve assertion IDs for milestone traceability
- [ ] Flag any gaps between spec docs and the feature being planned
- [ ] Reference invariant IDs in generated test tasks
- [ ] Include contract surface file paths in milestone declarations
- [ ] Carry forward ADR-CANDIDATE decisions for formalization

---

## Rules
- Never silently ignore spec docs if they exist -- always read them
- Spec docs are the source of truth for project-wide decisions; Cortex artifacts are feature-scoped
- If a feature's requirements conflict with a spec doc, flag it for user resolution
- Preserve IDs (A-001, FR-1.1, H-1) when copying between systems -- traceability depends on stable IDs
- If spec docs are incomplete (some templates not filled in), use what exists and generate the rest from scratch
- This skill enhances, not replaces, the standard plan-spec/plan-research/plan-steps skills
- Only inject spec doc sections relevant to the agent's tasks -- do not bloat prompts with full documents
