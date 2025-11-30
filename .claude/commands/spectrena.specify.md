# /spectrena.specify

Create or generate content for a specification from the spec backlog.

## Usage

```
# Create spec from backlog entry (preferred)
/spectrena.specify core-001-project-setup

# Create spec with custom description (not in backlog)
/spectrena.specify "Brief description" -c COMPONENT

# Generate content for existing spec (after spectrena new)
/spectrena.specify
```

## Arguments

| Argument        | Required | Description                                           |
| --------------- | -------- | ----------------------------------------------------- |
| spec-id         | No       | Spec ID from backlog (e.g., `core-001-project-setup`) |
| description     | No       | Brief title/description (if not using backlog)        |
| --component, -c | No       | Component prefix (only needed if not using backlog)   |

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

1. **Find the spec section** — Search for `### {spec-id}` heading
2. **Extract the scope line** — First line after heading describes the scope
3. **Parse the attribute table** — Extract Weight, Status, Depends On, References
4. **Extract "Covers" list** — Bullet points under "Covers:"
5. **Extract "Does NOT cover" list** — Bullet points (if present)
6. **Note any code blocks** — May contain schema hints, file structures, etc.

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

### Extracting Information

From the backlog entry, extract:

| Field        | Source           | Maps To                |
| ------------ | ---------------- | ---------------------- |
| Title        | Scope line       | Spec title             |
| Weight       | Attribute table  | Spec weight            |
| Dependencies | "Depends On" row | Dependencies section   |
| References   | "References" row | Reference docs to read |
| Scope items  | "Covers" bullets | Requirements/scope     |
| Out of scope | "Does NOT cover" | Non-goals              |

## Behavior

### Mode 1: From Backlog (Preferred)

```
/spectrena.specify core-001-project-setup
```

1. **Parse backlog** — Find and extract spec entry
2. **Read reference docs** — Load docs listed in References
3. **Check dependencies** — Warn if dependent specs incomplete
4. **Generate spec** — Use backlog scope + reference docs
5. **Create files** — `specs/{spec-id}/spec.md`
6. **Update backlog** — Change status from ⬜ to 🟨

### Mode 2: Custom Description (Not in Backlog)

```
/spectrena.specify "Custom feature" -c PLATFORM
```

1. Validate component prefix
2. If brief (< 20 words), ask 2-3 clarifying questions
3. Generate spec ID (e.g., `platform-099-custom-feature`)
4. Generate full spec content
5. Save to `specs/{spec-id}/spec.md`

### Mode 3: Fill Existing Spec

```
/spectrena.specify
```

1. Read existing `spec.md` in current directory
2. Extract description from `## Description`
3. If brief, ask clarifying questions
4. Generate content for empty sections
5. Update `spec.md` in place

## Reference Doc Loading

When the backlog specifies references like `ARCH §Project Structure, DOM §Quest`:

1. **Map abbreviations to files:**
   - `REQ` → `altair-requirements-v2.md`
   - `ARCH` → `altair-technical-architecture.md`
   - `DOM` → `altair-domain-model.md`
   - `UF` → `altair-user-flows.md`
   - `DS` → `altair-design-system.md`
   - `ADR` → `altair-decision-log.md`
   - `GLOSS` → `altair-glossary.md`

2. **Extract relevant sections** — If `§Section Name` specified, focus on that section

3. **Use for context** — Reference docs inform the spec content

## Content Generation

Generate these sections based on backlog + references:

| Section             | Source              | Guidelines                        |
| ------------------- | ------------------- | --------------------------------- |
| **Title**           | Backlog scope       | Clear, concise name               |
| **Weight**          | Backlog attribute   | LIGHTWEIGHT / STANDARD / FORMAL   |
| **Problem**         | Inferred from scope | What pain point? Who affected?    |
| **Solution**        | Reference docs      | High-level approach               |
| **Requirements**    | "Covers" bullets    | Numbered list, testable           |
| **Non-Goals**       | "Does NOT cover"    | Explicit boundaries               |
| **Dependencies**    | "Depends On"        | Spec IDs that must complete first |
| **Technical Notes** | Reference docs      | Architecture constraints          |
| **Open Questions**  | Gaps in backlog     | Decisions still needed            |

## Clarification Guidelines

**Only ask if backlog entry is incomplete.** The backlog should provide enough context.

If clarification needed, ask focused questions:

```
I found core-001-project-setup in the backlog. The scope covers monorepo setup, but I have a quick question:

1. The backlog mentions Turborepo - should I also document alternative evaluation, or assume Turborepo is decided?

Or I can proceed assuming Turborepo is final (per ADR in decision log).
```

**Max 2 clarification rounds** when using backlog. Then generate with stated assumptions.

**Always offer to proceed with defaults** — don't block on answers.

## Examples

### From Backlog (Standard Flow)

```
User: /spectrena.specify guidance-001-quest-crud

Claude: Found guidance-001-quest-crud in backlog:
- Scope: Quest entity operations with QBA board columns
- Weight: STANDARD
- Depends On: core-002, core-003
- References: REQ §1.1, DOM §Quest, UF §G-1 §G-2, GLOSS §QBA Board Columns

Loading reference docs...

Creating specs/guidance-001-quest-crud/spec.md...

[generates full spec with requirements from backlog "Covers" list,
 technical details from DOM and ARCH, UI patterns from UF]
```

### From Backlog with Dependency Warning

```
User: /spectrena.specify guidance-004-focus-mode

Claude: Found guidance-004-focus-mode in backlog.

⚠️ Dependency check:
- guidance-001-quest-crud: ⬜ Not started

This spec depends on guidance-001. Options:
1. Proceed anyway (will note dependency in spec)
2. Start with guidance-001 first

User: Proceed anyway

Claude: Creating specs/guidance-004-focus-mode/spec.md...
[generates spec with dependency clearly noted]
```

### Custom Spec (Not in Backlog)

```
User: /spectrena.specify "Email notifications" -c PLATFORM

Claude: I don't see "email notifications" in the backlog. I'll create a new spec.

Quick questions:
1. What triggers notifications? (quest due, capture pending, etc.)
2. Email provider preference? (SendGrid, SES, self-hosted?)
3. User preferences for notification frequency?

Or I can generate with sensible defaults.

User: Quest due dates and weekly harvest reminder, SES, user configurable

Claude: Creating specs/platform-050-email-notifications/spec.md...
[generates full spec]
```

### Fill Existing Spec

```
User: /spectrena.specify

Claude: Reading specs/core-010-auth-local/spec.md...

Found partial spec with:
- Title: Local Authentication
- Weight: STANDARD
- Empty sections: Requirements, Technical Notes

Checking backlog for core-010-auth-local...
Found entry with "Covers" list and references.

Filling empty sections from backlog context...
[updates spec.md in place]
```

## Spec Template

When generating, use this structure:

```markdown
# {spec-id}: {Title}

**Version**: 0.1  
**Status**: DRAFT  
**Weight**: {LIGHTWEIGHT|STANDARD|FORMAL}  
**Created**: {date}  
**Author**: {from git config or prompt}

## Summary

{2-3 sentence overview}

## Problem

{What pain point does this solve? Who is affected?}

## Solution

{High-level approach, not implementation details}

## Requirements

{Numbered list from backlog "Covers" section, expanded with details}

1. **{Requirement name}**
   - {Details}
   - {Acceptance criteria}

## Non-Goals

{From backlog "Does NOT cover" section}

- {Item explicitly out of scope}

## Dependencies

{From backlog "Depends On"}

| Spec ID | Status   | Why Needed     |
| ------- | -------- | -------------- |
| {id}    | {status} | {brief reason} |

## Technical Notes

{From reference docs - architecture constraints, patterns to follow}

## Open Questions

{Unresolved decisions, gaps identified during generation}

1. {Question}
2. {Question}

## References

{Links to reference docs used}

- [{Doc name}](./path) — §{Section}
```

## Updating Backlog Status

After creating a spec, update the backlog:

1. Change status from `⬜` to `🟨` (in progress)
2. Add "Spec created: {date}" note if desired

When spec is approved, status changes to `🟩`.
