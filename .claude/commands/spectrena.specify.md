# /spectrena.specify

Create or generate content for a specification from the spec backlog.

## Usage

```
# Create spec from backlog entry (preferred)
/spectrena.specify core-001-project-setup

# Create new spec with custom description (not in backlog)
/spectrena.specify "Brief description" -c COMPONENT

# Generate content for existing spec OR create new interactively
/spectrena.specify
```

## Arguments

| Argument        | Required | Description                                           |
| --------------- | -------- | ----------------------------------------------------- |
| spec-id         | No       | Spec ID from backlog (e.g., `core-001-project-setup`) |
| description     | No       | Brief title/description (if not using backlog)        |
| --component, -c | No       | Component prefix (only needed if not using backlog)   |

---

## Spec Backlog Integration

### Locating the Backlog

The spec backlog is at `docs/altair-spec-backlog.md` (or project root). It contains pre-defined specs with:

- **Scope** — What the spec covers
- **Weight** — LIGHTWEIGHT, STANDARD, or FORMAL
- **Dependencies** — Other specs that must complete first
- **References** — Which docs to consult
- **Covers / Does NOT cover** — Explicit boundaries

### Parsing the Backlog

When given a spec ID like `core-001-project-setup`:

1. **Find the spec section** — Search for `### {spec-id}` heading (exact match, case-insensitive)
2. **Extract the scope line** — Line starting with `**Scope:**` after the heading
3. **Parse the attribute table** — Extract Weight, Status, Depends On, References from markdown table
4. **Extract "Covers" list** — Bullet points after `**Covers:**`
5. **Extract "Does NOT cover" list** — Bullet points after `**Does NOT cover:**` (if present)
6. **Capture code blocks** — May contain schema hints, file structures, table names

**Example backlog entry:**

```markdown
### core-001-project-setup

**Scope:** Monorepo structure, build system, tooling

| Attribute      | Value                   |
| -------------- | ----------------------- |
| **Weight**     | STANDARD                |
| **Status**     | ⬜                      |
| **References** | ARCH §Project Structure |

**Covers:**

- pnpm workspace configuration
- Turborepo build pipeline
- App scaffolding (guidance, knowledge, tracking, mobile)
- Package scaffolding (ui, bindings, db, sync, storage, search)
- Shared TypeScript/ESLint/Prettier config
- Git hooks (husky, lint-staged)

**Does NOT cover:**

- Actual app implementation
- Database schema
- UI components
```

### Field Mapping

Extract and map these fields:

| Backlog Field    | Spec Section           | Notes                             |
| ---------------- | ---------------------- | --------------------------------- |
| Spec ID          | Folder name, branch    | `specs/{spec-id}/`                |
| Scope line       | Title, Description     | Primary description               |
| Weight           | Metadata               | LIGHTWEIGHT / STANDARD / FORMAL   |
| Depends On       | Dependencies           | Spec IDs, check their status      |
| References       | Reference docs to load | Parse doc abbreviations           |
| "Covers" bullets | Requirements           | Expand into detailed requirements |
| "Does NOT cover" | Non-Goals              | Explicit boundaries               |
| Code blocks      | Technical Notes        | Schema hints, structures          |

### Reference Doc Abbreviations

When the backlog specifies references like `ARCH §Project Structure, DOM §Quest`:

| Abbrev  | Document               | Path                                    |
| ------- | ---------------------- | --------------------------------------- |
| `REQ`   | Requirements           | `docs/altair-requirements-v2.md`        |
| `ARCH`  | Technical Architecture | `docs/altair-technical-architecture.md` |
| `DOM`   | Domain Model           | `docs/altair-domain-model.md`           |
| `UF`    | User Flows             | `docs/altair-user-flows.md`             |
| `DS`    | Design System          | `docs/altair-design-system.md`          |
| `ADR`   | Decision Log           | `docs/altair-decision-log.md`           |
| `GLOSS` | Glossary               | `docs/altair-glossary.md`               |

If `§Section Name` is specified, focus on that section when loading the doc.

---

## Behavior

### Mode 1: From Backlog (Preferred)

```
/spectrena.specify core-001-project-setup
```

1. **Parse backlog** — Find and extract spec entry by ID
2. **Check dependencies** — Warn if "Depends On" specs are incomplete (⬜)
3. **Load reference docs** — Read docs listed in References field
4. **Create spec using CLI:**
   ```
   $ spectrena new -c {component} "{scope}"
   ✓ Created specs/{spec-id}/
   ✓ Created branch spec/{spec-id}
   ```
5. **Generate full spec content** — Use backlog scope + reference docs
6. **Save to spec.md**
7. **Update backlog status** — Change ⬜ to 🟨

### Mode 2: Custom Description (Not in Backlog)

```
/spectrena.specify "Brief description" -c COMPONENT
```

1. Validate component (prompt if missing)
2. **Create spec using CLI:** `spectrena new -c {component} "{description}"`
3. If description is brief (< 20 words), ask 2-3 clarifying questions
4. Generate full spec content
5. Save to spec.md

### Mode 3: Without Arguments

**If in a spec directory (spec.md exists):**

1. Read existing spec.md
2. Check if spec ID exists in backlog — if so, use backlog context
3. Extract/enhance description
4. Generate content for empty sections
5. Update spec.md in place

**If NOT in a spec directory:**

1. Ask: "Do you have a spec ID from the backlog, or creating something new?"
2. If backlog ID provided → Mode 1
3. If new → ask clarifying questions, then Mode 2

---

## Clarification Guidelines

### When Using Backlog (Mode 1)

**Minimal clarification needed** — the backlog provides context. Only ask if:

- Backlog entry is unusually brief
- Technical decision isn't covered by reference docs
- Ambiguity in scope boundaries

```
Found core-001-project-setup in backlog. One quick question:

The scope mentions "Turborepo build pipeline" but I don't see this in the decision log. Should I:
1. Document it as the decided choice
2. Include alternatives evaluation in the spec

Or I can proceed assuming Turborepo is decided per project conventions.
```

**Max 2 clarification rounds** for backlog specs. Then proceed with stated assumptions.

### When Not Using Backlog (Mode 2/3)

Ask 2-3 focused questions if description is brief:

| Input              | Questions to Ask                                |
| ------------------ | ----------------------------------------------- |
| `"User auth"`      | "OAuth? Username/password? What providers?"     |
| `"Monorepo setup"` | "What tools? Melos? Nx? What's being shared?"   |
| `"REST API"`       | "What resources? Auth required? Rate limiting?" |
| (no input)         | "What feature? What problem? Which component?"  |

**Max 3 clarification rounds.** Then generate with stated assumptions.

**Always offer to proceed with defaults:**

```
I can generate with reasonable defaults if you prefer, or answer these quick questions for a more tailored spec.
```

---

## Content Generation

### From Backlog

| Section             | Source                       | Guidelines                      |
| ------------------- | ---------------------------- | ------------------------------- |
| **Title**           | Backlog scope line           | Clear, concise name             |
| **Weight**          | Backlog attribute table      | LIGHTWEIGHT / STANDARD / FORMAL |
| **Description**     | Scope + "Covers" summary     | 2-3 sentence overview           |
| **Problem**         | Inferred from scope          | What pain point? Who affected?  |
| **Solution**        | Reference docs (ARCH)        | High-level approach             |
| **Requirements**    | "Covers" bullets expanded    | Numbered, testable criteria     |
| **Non-Goals**       | "Does NOT cover" bullets     | Explicit boundaries             |
| **Dependencies**    | "Depends On" field           | Spec IDs with status            |
| **Technical Notes** | Reference docs + code blocks | Architecture constraints        |
| **Open Questions**  | Gaps in backlog              | Decisions still needed          |

### From Custom Description

| Section            | Guidelines                                               |
| ------------------ | -------------------------------------------------------- |
| **Description**    | Preserved from input or generated from clarifications    |
| **Problem**        | What pain point? Who is affected? 2-3 sentences.         |
| **Solution**       | High-level approach (not implementation). 2-3 sentences. |
| **Dependencies**   | Other spec IDs that must complete first.                 |
| **Open Questions** | Unresolved decisions (from clarification gaps).          |

---

## Spec Template

```markdown
# {spec-id}: {Title}

**Version**: 0.1  
**Status**: DRAFT  
**Weight**: {LIGHTWEIGHT|STANDARD|FORMAL}  
**Created**: {date}  
**Author**: {from git config}

## Summary

{2-3 sentence overview from backlog scope + covers}

## Problem

{What pain point does this solve? Who is affected?}

## Solution

{High-level approach from reference docs, not implementation details}

## Requirements

{Expanded from backlog "Covers" bullets}

1. **{Requirement name}**
   - {Details}
   - {Acceptance criteria}

2. **{Requirement name}**
   - {Details}
   - {Acceptance criteria}

## Non-Goals

{From backlog "Does NOT cover"}

- {Item explicitly out of scope}
- {Item explicitly out of scope}

## Dependencies

| Spec ID | Status     | Why Needed     |
| ------- | ---------- | -------------- |
| {id}    | {⬜/🟨/🟩} | {brief reason} |

## Technical Notes

{From reference docs - architecture constraints, patterns, decisions}

### {Subsection if needed}

{Details}

## Open Questions

1. {Unresolved decision}
2. {Gap identified during generation}

## References

- [Technical Architecture](../docs/altair-technical-architecture.md) — §{Section}
- [Domain Model](../docs/altair-domain-model.md) — §{Section}
```

---

## Examples

### Example 1: From Backlog (Standard Flow)

```
User: /spectrena.specify guidance-001-quest-crud

Claude: Found guidance-001-quest-crud in backlog:

  Scope: Quest entity operations with QBA board columns
  Weight: STANDARD
  Depends On: core-002, core-003
  References: REQ §1.1, DOM §Quest, UF §G-1 §G-2, GLOSS §QBA Board Columns

Checking dependencies...
  ⚠️ core-002-schema-migrations: ⬜ Not started
  ⚠️ core-003-backend-skeleton: ⬜ Not started

Dependencies incomplete. Options:
1. Proceed anyway (dependency noted in spec)
2. Start with core-002 first

User: Proceed anyway

Claude: Loading reference docs...
Creating spec...

$ spectrena new -c guidance "Quest entity operations with QBA board columns"
✓ Created specs/guidance-001-quest-crud/
✓ Created branch spec/guidance-001-quest-crud

[generates full spec with requirements from "Covers" list,
 technical details from DOM and ARCH, UI patterns from UF]

✓ Spec ready: specs/guidance-001-quest-crud/spec.md
  Branch: spec/guidance-001-quest-crud

Updating backlog status: ⬜ → 🟨
```

### Example 2: From Backlog (No Issues)

```
User: /spectrena.specify core-001-project-setup

Claude: Found core-001-project-setup in backlog:

  Scope: Monorepo structure, build system, tooling
  Weight: STANDARD
  Depends On: (none)
  References: ARCH §Project Structure

No dependencies - ready to create.

$ spectrena new -c core "Monorepo structure, build system, tooling"
✓ Created specs/core-001-project-setup/
✓ Created branch spec/core-001-project-setup

[generates full spec]

✓ Spec ready: specs/core-001-project-setup/spec.md
  Branch: spec/core-001-project-setup

Updating backlog status: ⬜ → 🟨
```

### Example 3: Custom Spec (Not in Backlog)

```
User: /spectrena.specify "Email notifications" -c platform

Claude: I don't see "email notifications" in the backlog. Creating custom spec.

Quick questions:
1. What triggers notifications? (quest due, capture pending, etc.)
2. Email provider? (SES, SendGrid, self-hosted?)
3. User preferences for frequency?

Or I can generate with sensible defaults.

User: Quest due dates and weekly harvest reminder, SES, user configurable

Claude: Creating spec...

$ spectrena new -c platform "Email notifications for quest reminders"
✓ Created specs/platform-050-email-notifications/
✓ Created branch spec/platform-050-email-notifications

[generates full spec]

✓ Spec ready: specs/platform-050-email-notifications/spec.md
  Branch: spec/platform-050-email-notifications
```

### Example 4: Fill Existing Spec

```
User: /spectrena.specify

Claude: Reading specs/core-010-auth-local/spec.md...

Found partial spec. Checking backlog for core-010-auth-local...
✓ Found in backlog with "Covers" list and references.

Current spec has empty sections:
- Requirements (will fill from backlog "Covers")
- Technical Notes (will fill from ARCH references)

Filling empty sections...

✓ Updated: specs/core-010-auth-local/spec.md
```

### Example 5: Interactive (No Context)

```
User: /spectrena.specify

Claude: No spec found in current directory.

Do you have a spec ID from the backlog (e.g., core-001-project-setup)?
Or are you creating something new?

User: knowledge-003-wiki-links

Claude: Found knowledge-003-wiki-links in backlog:

  Scope: Bidirectional wiki-style linking
  Weight: STANDARD
  ...

[proceeds with Mode 1 flow]
```

---

## Updating Backlog Status

After creating a spec:

1. **Find the spec entry** in `docs/altair-spec-backlog.md`
2. **Update status** in the attribute table:
   - `⬜` → `🟨` (in progress)
3. **Optionally add note**: `<!-- Spec created: YYYY-MM-DD -->`

Status progression:

- `⬜` Not started
- `🟨` In progress (spec being written)
- `🟩` Complete (spec approved, ready for implementation)
- `🚫` Blocked
