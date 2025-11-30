# AGENTS.md

> **Working with AI agents** — Guidelines for contributors using Claude and
> other AI tools

---

## Overview

This project uses AI agents to accelerate development while maintaining quality.
This document explains:

1. How to use Claude Code effectively
2. Spec-driven development workflow
3. When agents should ask vs. act
4. Quality expectations

---

## Claude Code

### What Claude Code Knows

Claude Code reads `CLAUDE.md` for project context. It understands:

- Project structure and tech stack
- Key architectural decisions
- Terminology and domain model
- Development patterns and code style

### Effective Prompts

**Good prompts:**

```text
# Specific, references existing patterns
"Add a Tauri command for updating quest status, following the pattern in
 create_quest"

# Points to relevant docs
"Implement the wiki-link parsing from knowledge-002 spec"

# Explains the 'why'
"Refactor the sync engine to batch changes - currently it's making one
 request per record"
```

**Less effective prompts:**

```text
# Too vague
"Make the app better"

# Missing context
"Add a feature for tasks"  # We call them Quests

# Assumes knowledge we haven't documented
"Do it like we discussed"  # Claude Code doesn't have chat history
```

### When to Use Claude Code

| Use Case              | Example                                              |
| -------------------- | ---------------------------------------------------- |
| **Implement from spec** | "Implement FR-001 from guidance-001-quest-crud spec" |
| **Add boilerplate**   | "Create a new Tauri command for X following..."     |
| **Refactor**          | "Extract the validation logic into a separate..."   |
| **Debug**             | "This test is failing with X error, here's..."      |
| **Explain code**      | "Explain what this sync algorithm does"             |

### When NOT to Use Claude Code

| Situation                  | Do Instead                                     |
| -------------------------- | ---------------------------------------------- |
| **Architectural decisions** | Discuss in PR/issue, update decision-log.md   |
| **Changing specs**         | Update spec first, then implement              |
| **Security-sensitive code** | Review manually, use for suggestions only      |
| **Performance-critical paths** | Benchmark, don't trust estimates           |

---

## Spec-Driven Development (SDD)

### Workflow

```text
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌───────────┐
│  Spec   │ →  │  Plan   │ →  │  Tasks  │ →  │ Implement │
│ (what)  │    │  (how)  │    │ (steps) │    │  (code)   │
└─────────┘    └─────────┘    └─────────┘    └───────────┘
```

### Using Spectrena

Spectrena is an AI-assisted spec workflow. Key commands:

| Command                | Purpose                                    |
| ---------------------- | ------------------------------------------ |
| `/spectrena.specify`   | Create spec from feature description       |
| `/spectrena.clarify`   | Resolve `[NEEDS CLARIFICATION]` markers    |
| `/spectrena.plan`      | Generate implementation plan from spec     |
| `/spectrena.tasks`     | Break plan into executable tasks           |
| `/spectrena.implement` | Execute task with code generation          |

### Spec Structure

```bash
specs/
├── core-001-project-setup/
│   ├── spec.md           # What to build
│   ├── plan.md           # How to build it
│   └── tasks.md          # Step-by-step tasks
├── guidance-001-quest-crud/
│   ├── spec.md
│   ├── plan.md
│   └── tasks.md
└── ...
```

### Writing Specs

Use the template at `specs/template.md`. Key sections:

1. **Quick Reference** — One-paragraph summary
2. **Problem Statement** — Current vs desired state
3. **Requirements** — FR-XXX (functional), NFR-XXX (non-functional)
4. **Data Model** — Entities and relationships
5. **Interfaces** — Operations (not implementation)
6. **Test Requirements** — Acceptance criteria

### Spec Weights

| Weight | Effort | Required Sections |
|--------|--------|-------------------|
| LIGHTWEIGHT | <2 days | Quick Reference, Problem, Requirements, Acceptance |
| STANDARD | 2-10 days | All LIGHTWEIGHT + Data Model, Security, Tests |
| FORMAL | >10 days | All sections, full review |

---

## Agent Boundaries

### Claude Code Should Ask When

- **Ambiguous requirements** — "The spec says X but the code does Y, which is
  correct?"
- **Missing information** — "I don't see a migration for this table, should I
  create one?"
- **Breaking changes** — "This would change the API contract, proceed?"
- **Security implications** — "This involves auth/crypto, please review my
  approach"
- **Significant refactoring** — "This touches 10+ files, here's my plan..."

### Claude Code Should Act When

- **Clear spec exists** — Implement exactly what's specified
- **Following established patterns** — New command like existing commands
- **Bug fixes with tests** — Fix passes tests, doesn't change behavior
- **Documentation** — Adding/updating docs and comments
- **Formatting/linting** — Mechanical code improvements

---

## Code Review Expectations

### What AI-Generated Code Should Have

| Requirement | Why |
|-------------|-----|
| **Follows existing patterns** | Consistency > cleverness |
| **Includes tests** | AI can generate tests too |
| **Updates types** | TypeScript bindings stay in sync |
| **Handles errors** | No unwrap() in production paths |
| **Has comments for non-obvious logic** | Future readers (including AI) need context |

### Review Checklist

When reviewing AI-generated PRs:

- [ ] Does it match the spec?
- [ ] Does it follow project patterns?
- [ ] Are there tests?
- [ ] Does it handle errors gracefully?
- [ ] Are types correct and complete?
- [ ] Any security concerns?
- [ ] Performance acceptable?

---

## Project-Specific Rules

### Terminology

Always use project terminology (see `docs/glossary.md`):

| ✅ Correct | ❌ Incorrect |
|-----------|-------------|
| Quest | Task, Todo |
| Campaign | Project, Epic |
| Note | Document, Page |
| Capture | Inbox item |
| Archive | Delete |

### Architectural Constraints

Don't violate these without updating `docs/decision-log.md`:

1. **Desktop uses Tauri IPC** — Not REST
2. **All tables have CHANGEFEED** — For sync
3. **Soft delete only** — No hard deletes
4. **Local embeddings** — No cloud dependency for search
5. **Plugin architecture for auth/AI** — Use traits, not hardcoding

### File Naming

| Type | Convention | Example |
|------|------------|---------|
| Rust modules | snake_case | `quest_commands.rs` |
| Svelte components | PascalCase | `QuestCard.svelte` |
| TypeScript utils | camelCase | `formatDate.ts` |
| Specs | `component-NNN-slug` | `guidance-001-quest-crud` |
| Migrations | `NNN_description.surql` | `001_initial_schema.surql` |

---

## Troubleshooting

### Claude Code Generates Wrong Patterns

**Symptom:** Code doesn't match project style

**Fix:**

1. Point to specific example: "Follow the pattern in `backend/src/commands/quest.rs`"
2. Update CLAUDE.md if pattern wasn't documented

### Spec and Code Diverge

**Symptom:** Implementation doesn't match spec

**Fix:**

1. Determine which is correct (spec or code)
2. Update the incorrect one
3. Note in PR what changed and why

### Type Generation Out of Sync

**Symptom:** TypeScript types don't match Rust

**Fix:**

```bash
cargo run --bin generate-bindings
pnpm --filter @altair/bindings build
```

### Agent Hallucinates APIs

**Symptom:** Uses functions/APIs that don't exist

**Fix:**

1. Check if it's in a dependency we haven't installed
2. If not, point to actual API: "Use `surrealdb::Surreal::query()`, not `execute()`"
3. Consider adding common APIs to CLAUDE.md

---

## Contributing with AI

### Recommended Workflow

1. **Pick a spec** from `docs/spec-backlog.md`
2. **Write/review the spec** (AI can help)
3. **Generate plan** with spectrena or manually
4. **Break into tasks** — Small, testable units
5. **Implement with Claude Code** — One task at a time
6. **Review and test** — Don't blindly trust
7. **PR with spec reference** — "Implements guidance-001-quest-crud"

### Commit Messages

```bash
type(scope): description

[spec-id] if implementing a spec

Examples:
feat(guidance): add quest CRUD commands [guidance-001]
fix(sync): handle offline queue overflow
docs: update CLAUDE.md with new patterns
refactor(db): extract query builders
```

### PR Description

```markdown
## What
Brief description of changes

## Why
Link to spec or issue

## How
Key implementation decisions

## Testing
How to verify this works

## Spec Reference
`guidance-001-quest-crud` (if applicable)
```
