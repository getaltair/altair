---
description: Create or generate content for a specification
arguments:
  - name: spec-id
    description: Spec ID from backlog (e.g., core-001-project-setup)
    required: false
  - name: description
    description: Brief title/description for the spec (if not using backlog)
    required: false
  - name: component
    description: Component prefix (CORE, API, UI, etc.) - only needed if not using backlog
    required: false
    flag: -c,--component
---

# Specify

Create or generate content for a specification from the spec backlog or custom description.

## Usage

```
# From backlog (preferred)
/spectrena.specify core-001-project-setup

# Custom spec (not in backlog)
/spectrena.specify "Brief description" -c COMPONENT

# Fill existing or interactive
/spectrena.specify
```

## Input Expectations

**Backlog specs have full context.** Minimal clarification needed.

**Custom specs may need clarification.** If description lacks detail, ask 2-3 questions:

| Input                    | Action                                             |
| ------------------------ | -------------------------------------------------- |
| `core-001-project-setup` | Parse backlog, generate directly                   |
| `"User auth"`            | Ask: "OAuth? Username/password? What providers?"   |
| `"Monorepo setup"`       | Ask: "What tools? Melos? Nx? What's being shared?" |
| Detailed paragraph       | Generate directly                                  |

**Max 3 clarification rounds.** Then generate with stated assumptions.

---

## Spec Backlog Integration

### Locating the Backlog

The spec backlog is at `docs/spec-backlog.md`. It contains pre-defined specs with:

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

**Does NOT cover:**

- Actual app implementation
- Database schema
```

### Field Mapping

| Backlog Field    | Spec Section        | Notes                             |
| ---------------- | ------------------- | --------------------------------- |
| Spec ID          | Folder name, branch | `specs/{spec-id}/`                |
| Scope line       | Title, Description  | Primary description               |
| Weight           | Metadata            | LIGHTWEIGHT / STANDARD / FORMAL   |
| Depends On       | Dependencies        | Check their status in backlog     |
| References       | Docs to load        | Parse abbreviations               |
| "Covers" bullets | Requirements        | Expand into detailed requirements |
| "Does NOT cover" | Non-Goals           | Explicit boundaries               |

### Reference Doc Abbreviations

| Abbrev  | Document               | Path                             |
| ------- | ---------------------- | -------------------------------- |
| `REQ`   | Requirements           | `docs/requirements.md`           |
| `ARCH`  | Technical Architecture | `docs/technical-architecture.md` |
| `DOM`   | Domain Model           | `docs/domain-model.md`           |
| `UF`    | User Flows             | `docs/user-flows.md`             |
| `DS`    | Design System          | `docs/design-system.md`          |
| `ADR`   | Decision Log           | `docs/decision-log.md`           |
| `GLOSS` | Glossary               | `docs/glossary.md`               |

If `§Section Name` is specified, focus on that section when loading.

---

## Behavior

### Mode 1: From Backlog (Preferred)

```
/spectrena.specify core-001-project-setup
```

1. **Parse backlog** — Find and extract spec entry by ID
2. **Check dependencies** — Warn if "Depends On" specs have status ⬜
3. **Load reference docs** — Read docs listed in References field
4. **Git: Ensure clean working tree** (warn if uncommitted changes)
5. Create directory: `specs/{spec-id}/`
6. Copy template: `.spectrena/templates/spec-template.md` → `specs/{spec-id}/spec.md`
7. **Git: Create and checkout branch:**
   ```bash
   git checkout -b spec/{spec-id}
   ```
8. Generate full spec content using backlog scope + reference docs
9. Write content to `specs/{spec-id}/spec.md`
10. **Git: Stage and commit:**
    ```bash
    git add specs/{spec-id}/
    git commit -m "spec({spec-id}): Initialize specification"
    ```
11. **Update backlog status:** Change ⬜ to 🟨 in `docs/spec-backlog.md`

### Mode 2: Custom Description (Not in Backlog)

```
/spectrena.specify "Brief description" -c COMPONENT
```

1. Validate component (prompt if required but missing)
2. If description brief (< 20 words), ask 2-3 clarifying questions
3. Read `.spectrena/config.yml` for spec ID template
4. Find next spec number by scanning `specs/` directory
5. Generate spec ID: apply template (e.g., `CORE-001-user-auth`)
6. **Git: Ensure clean working tree** (warn if uncommitted changes)
7. Create directory: `specs/{SPEC-ID}/`
8. Copy template: `.spectrena/templates/spec-template.md` → `specs/{SPEC-ID}/spec.md`
9. **Git: Create and checkout branch:**
   ```bash
   git checkout -b spec/{SPEC-ID}
   ```
10. Generate full spec content based on description + clarifications
11. Write content to `specs/{SPEC-ID}/spec.md`
12. **Git: Stage and commit:**
    ```bash
    git add specs/{SPEC-ID}/
    git commit -m "spec({SPEC-ID}): Initialize specification"
    ```

### Mode 3: Without Arguments (Fill Existing or Interactive)

**If in spec directory or on spec branch:**

1. Detect current spec:
   - Check current git branch for `spec/{SPEC-ID}` pattern
   - Or find spec directory in current path
2. **Check backlog** — If spec ID exists in backlog, load that context
3. **Git: Ensure on correct branch:**
   ```bash
   git checkout spec/{SPEC-ID}
   ```
4. Read existing `spec.md`
5. If description brief and no backlog context, ask clarifying questions
6. Generate content for empty sections
7. Update `spec.md` in place
8. **Git: Commit changes:**
   ```bash
   git add specs/{SPEC-ID}/spec.md
   git commit -m "spec({SPEC-ID}): Expand specification content"
   ```

**If NOT in spec directory:**

1. Ask: "Do you have a spec ID from the backlog (e.g., `core-001-project-setup`), or creating something new?"
2. If backlog ID provided → Mode 1
3. If new → ask clarifying questions, then Mode 2

---

## Spec ID Generation

**For backlog specs:** Use the spec ID as-is (e.g., `core-001-project-setup`)

**For custom specs:** Read from `.spectrena/config.yml`:

```yaml
spec_id:
  template: '{component}-{NNN}-{slug}'
  padding: 3
  components:
    - CORE
    - API
    - UI
    - GUIDANCE
    - KNOWLEDGE
    - TRACKING
    - PLATFORM
```

Apply template:

- `{component}` → provided component (e.g., CORE)
- `{NNN}` → next number, zero-padded (e.g., 001)
- `{slug}` → slugified description (e.g., user-auth)

---

## Content Generation

### From Backlog

| Section             | Source                       |
| ------------------- | ---------------------------- |
| **Title**           | Backlog scope line           |
| **Weight**          | Backlog attribute table      |
| **Problem**         | Infer from scope             |
| **Solution**        | Reference docs (ARCH)        |
| **Requirements**    | Expand "Covers" bullets      |
| **Non-Goals**       | "Does NOT cover" bullets     |
| **Dependencies**    | "Depends On" with status     |
| **Technical Notes** | Reference docs + code blocks |
| **Open Questions**  | Gaps identified              |

### From Custom Description

| Section                 | Source                                  |
| ----------------------- | --------------------------------------- |
| **Title**               | From description                        |
| **Problem**             | Infer from description + clarifications |
| **Solution**            | Proposed approach                       |
| **Scope**               | What's in/out                           |
| **Dependencies**        | Other specs this depends on             |
| **Acceptance Criteria** | Testable requirements                   |

---

## Examples

### Example 1: From Backlog

```
User: /spectrena.specify core-001-project-setup

Claude: Found core-001-project-setup in backlog:

  Scope: Monorepo structure, build system, tooling
  Weight: STANDARD
  Depends On: (none)
  References: ARCH §Project Structure

Loading reference docs...
No dependencies - ready to create.

$ git checkout -b spec/core-001-project-setup
Switched to a new branch 'spec/core-001-project-setup'

[creates directory, generates spec from backlog context]

$ git add specs/core-001-project-setup/
$ git commit -m "spec(core-001-project-setup): Initialize specification"

✓ Created specs/core-001-project-setup/spec.md
✓ On branch spec/core-001-project-setup
✓ Updated backlog status: ⬜ → 🟨
```

### Example 2: From Backlog with Dependency Warning

```
User: /spectrena.specify guidance-001-quest-crud

Claude: Found guidance-001-quest-crud in backlog:

  Scope: Quest entity operations with QBA board columns
  Weight: STANDARD
  Depends On: core-002, core-003
  References: REQ §1.1, DOM §Quest, UF §G-1 §G-2

Checking dependencies...
  ⚠️ core-002-schema-migrations: ⬜ Not started
  ⚠️ core-003-backend-skeleton: ⬜ Not started

Dependencies incomplete. Options:
1. Proceed anyway (dependency noted in spec)
2. Start with core-002 first

User: Proceed anyway

Claude: $ git checkout -b spec/guidance-001-quest-crud
Switched to a new branch 'spec/guidance-001-quest-crud'

[generates spec with dependencies noted]

$ git add specs/guidance-001-quest-crud/
$ git commit -m "spec(guidance-001-quest-crud): Initialize specification"

✓ Created specs/guidance-001-quest-crud/spec.md
✓ On branch spec/guidance-001-quest-crud
✓ Updated backlog status: ⬜ → 🟨
```

### Example 3: Custom Spec (Not in Backlog)

```
User: /spectrena.specify "OAuth login" -c CORE

Claude: I don't see "OAuth login" in the backlog. Creating custom spec.

Quick questions:
1. Which OAuth providers? (Google, GitHub, etc.)
2. Need refresh tokens?
3. Any role-based access?

User: Google and GitHub, yes refresh tokens, no roles yet

Claude: Creating CORE-050-oauth-login...

$ git checkout -b spec/CORE-050-oauth-login
Switched to a new branch 'spec/CORE-050-oauth-login'

[creates directory, writes spec.md]

$ git add specs/CORE-050-oauth-login/
$ git commit -m "spec(CORE-050-oauth-login): Initialize specification"

✓ Created specs/CORE-050-oauth-login/spec.md
✓ On branch spec/CORE-050-oauth-login
```

### Example 4: Fill Existing Spec

```
User: /spectrena.specify

Claude: Detected branch: spec/core-010-auth-local
Reading specs/core-010-auth-local/spec.md...

Found partial spec. Checking backlog for core-010-auth-local...
✓ Found in backlog with "Covers" list and references.

Empty sections to fill:
- Requirements (from backlog "Covers")
- Technical Notes (from ARCH references)

Generating content...

$ git add specs/core-010-auth-local/spec.md
$ git commit -m "spec(core-010-auth-local): Expand specification content"

✓ Updated specs/core-010-auth-local/spec.md
```

---

## Updating Backlog Status

After creating a spec from backlog:

1. Open `docs/spec-backlog.md`
2. Find the spec entry's attribute table
3. Change `| **Status** | ⬜ |` to `| **Status** | 🟨 |`
4. Stage and commit with the spec changes

Status progression:

- `⬜` Not started
- `🟨` In progress (spec being written)
- `🟩` Complete (spec approved)
- `🚫` Blocked
