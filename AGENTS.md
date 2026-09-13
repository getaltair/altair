# Altair - AI Agents

This file is the canonical project context for OpenCode and other AGENTS.md-compatible tools.

Read `CONVENTIONS.md` before any GitHub or git operation.

<!-- BEGIN bigpowers:context-routing -->
## Context Routing

No subdirectory has an `AGENTS.md` file. Add a routing row when a subdirectory needs specific instructions.
<!-- END bigpowers:context-routing -->

<!-- BEGIN bigpowers:learned-preferences -->
## Learned User Preferences

- Use Conventional Commits.
- Never modify lockfiles by hand.

## Workspace Facts

- `docs/initial-build.md` is the accepted initial-build definition.
<!-- END bigpowers:learned-preferences -->

<!-- BEGIN bigpowers:project -->
## Project

Altair is a web-first personal management system connecting projects, tasks, knowledge, and resources through contextual recall.

Stack: Vue 3, TypeScript, Vite, Pinia, Rust, Axum, SQLx, PostgreSQL 18, pgvector, CodeMirror 6, Authentik, and Garage.

## Commands

| Action | Command |
|---|---|
| Run | `just dev` |
| Test | `just test` |
| Build | `just build` |
| Lint | `just lint` |
| Preflight | `just preflight` |
| CI | `gh pr checks` when a pull request is open |

Treat these `just` commands as the stable project interface. Add their recipes during application scaffolding.

## Test

Run `just test` after each behavior change.

## Lint

Run `just lint` after changing source or configuration.

## Build

Run `just build` before declaring implementation work complete.

## Architecture

Organize application code by `projects`, `tasks`, `notes`, `resources`, and `recall` domains.

Keep database, object storage, identity, and inference integrations in separate infrastructure modules.

The Vue client calls Axum. Axum owns PostgreSQL, Garage, Authentik, and OpenAI-compatible inference access.

## Conventions

- Read `docs/initial-build.md` before product, domain, or architecture decisions.
- Keep domain records distinct while sharing stable identities and explicit relationships.
- Keep authored Markdown canonical and independent of CodeMirror state.
- Keep saved state, editor drafts, and server-response caches distinct.
- Use explicit SQLx migrations for every schema change.
- Use server-owned sessions with Secure, HttpOnly cookies and appropriate CSRF protection.
- Preserve user data and drafts across failures, navigation, conflicts, and implementation replacement.
- Use Conventional Commits with `<type>(<scope>): <description>`.
- Manage dependencies and lockfiles through package-manager commands.

## Never

- NEVER dismiss reproducible gate failures as pre-existing or out of scope.
- NEVER proceed while Preflight or CI is red.
- NEVER modify a lockfile by hand.
- NEVER expose personal data remotely without authentication.
- NEVER send content to an undeclared inference provider or hidden cloud fallback.
- NEVER block authoritative saves on embedding inference.
- NEVER let suggestions mutate records, links, quantities, or statuses automatically.
- NEVER add excluded v0.1 systems without an accepted scope change.
- NEVER couple domain behavior to Vue, Axum, SQLx, or provider payload types.
- NEVER commit secrets, `.env`, `node_modules/`, `dist/`, or `target/`.

## Agent Rules

- MUST use bigpowers skills for structured planning, implementation, verification, and release work.
- MUST read `specs/`, `CONVENTIONS.md`, and relevant `docs/` before writing code.
- MUST write all planning and specification artifacts under `specs/`.
- MUST write the minimum code that satisfies accepted behavior.
- MUST run relevant tests after every change.
- MUST show verification evidence before declaring work complete.
- MUST ask one focused question instead of encoding an unsupported assumption.
<!-- END bigpowers:project -->
