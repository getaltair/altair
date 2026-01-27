## Goal
Keep `docs/` (architecture/ADR/reference), plus top-level dev-facing docs (`README.md`, `CLAUDE.md`, `AGENTS.md`) accurate as Kotlin/Gradle code changes, without auto-committing to `main`.

## Current state (repo-specific)
- You already run CI for tests (`test.yml`) and static checks (`code-quality.yml`).
- You already have Factory Droid GitHub workflows for tagging and auto-review (`droid.yml`, `droid-review.yml`).
- There is **no** existing documentation maintenance workflow.
- The codebase is Kotlin Multiplatform (`composeApp/`, `shared/`, `server/`); docs are substantial under `docs/architecture/*`.

## Option A (recommended)
### 1) Scheduled Docs Maintenance workflow (creates PRs for review)
Add a new GitHub Actions workflow (e.g. `.github/workflows/docs-maintenance.yml`) that:
- Runs on `schedule` (weekly) and `workflow_dispatch`.
- Computes “recently changed” files (e.g. last 7 days) from git history.
- Runs `droid exec --auto low` with a prompt that:
  - updates only relevant markdown files in `docs/`, `README.md`, `CLAUDE.md`, `AGENTS.md`
  - preserves structure/style and avoids broad rewrites
  - writes a summary file (e.g. `docs-updates.md`) for reviewers
  - does **not** commit/push (workflow handles git)
- If changes exist, the workflow creates a branch + PR with the summary in the body.

**Why weekly:** reduces churn/noise and avoids frequent PRs; you can switch to daily if you want.

**Kotlin-focused file targeting:**
- Treat these as “doc-impacting” sources:
  - `shared/src/**/*.kt`, `composeApp/src/**/*.kt`, `server/src/**/*.kt`
  - `build.gradle.kts`, `settings.gradle.kts`, `gradle.properties`, `gradle/**`

**Prompt skeleton (tailored to this repo):**
- “Given these changed files, update `docs/architecture/*` and other relevant docs if (and only if) they are now inaccurate. Prefer minimal diffs. If uncertain, add a ‘Needs human review’ note in `docs-updates.md` instead of changing architecture/ADR.”

### 2) Lightweight local “documentation-sync” warnings (during Droid sessions)
Optionally add a `.factory/settings.json` PostToolUse hook (per Factory’s *documentation-sync* guide) that runs a small script to **warn** (not auto-edit) when Droid changes Kotlin/Gradle files that usually require doc touch-ups.
- Best practice alignment: “Automate but don’t auto-commit.”
- Scope: warn-only, fast (no Gradle builds), and only for paths like `shared/src/commonMain/**`, `server/src/main/**`, `docs/architecture/**`.

## Option B
Same as Option A, but trigger on every push to `main` with a `paths:` filter (post-merge docs PRs).
- Pros: docs stay fresher.
- Cons: more PR noise; can be expensive because your existing CI runs full JVM + Android test suites for doc-only PRs.

## Option C
Only add local `.factory` documentation-sync hooks (warn-only) and rely on humans to update docs.
- Pros: no bot PRs.
- Cons: docs drift will still accumulate.

## Rollout / safety checklist
1. Add `FACTORY_API_KEY` to GitHub repo secrets.
2. Start with **weekly** schedule + manual `workflow_dispatch` so you can trial-run.
3. Ensure the prompt explicitly restricts edits to documentation locations and asks for minimal diffs.
4. Keep PRs human-reviewed; never push directly to `main`.
5. (Optional, recommended if you go daily) Add `paths-ignore: ['docs/**', '*.md']` to heavy CI workflows to avoid running full Kotlin test suites for doc-only PRs.

## What I will implement after you approve
- Create the new docs-maintenance workflow (Option A or B).
- (If chosen) Add `.factory/settings.json` + a warn-only hook script for doc-sync hints during Droid sessions.
- Validate by dry-running the workflow logic locally where feasible and ensuring the YAML is consistent with your existing CI conventions.