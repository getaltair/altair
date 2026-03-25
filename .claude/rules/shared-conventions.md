# Shared Conventions

These conventions apply to all code regardless of stack.

## Git
- Conventional commits: `type(scope): description`
- Types: feat, fix, refactor, test, docs, chore, build, ci
- Commit messages reference task numbers: `Tasks: S001, S002`
- Commit messages reference ADRs when applicable: `ADR-0003`
- One logical change per commit — don't mix unrelated changes
- Planning artifacts (Context/) committed separately from source code

## Shared Contracts
- All platforms use canonical identifiers from `packages/contracts/`
- See `docs/altair-shared-contracts-spec.md` for full contract strategy
- Never invent new shared identifiers inline — use generated bindings
- Registry JSON is the source of truth for entity types, relation types, sync streams

## Code Organization
- One responsibility per file — if a file does two unrelated things, split it
- Public API surface should be minimal — don't export internals
- Dead code gets deleted, not commented out (git has history)
- No secrets in source code — use environment variables or secret managers

## Documentation
- Public functions/methods get documentation comments
- Complex algorithms get inline comments explaining *why*, not *what*
- README.md at project root covers setup, build, and run instructions
- Architecture decisions go in Context/Decisions/ as ADRs

## Planning Workflow
- Features use 4-phase workflow: Spec → Tech → Steps → Implementation
- Small tasks use quick plans
- Every deviation from spec gets an ADR
- Milestone checkpoints verify alignment with Testable Assertions
- Test and documentation tasks are planned, not afterthoughts

## Error Messages
- Error messages should help user fix the problem
- Include what went wrong, what was expected, and what to try
- Never expose internal stack traces to end users
- Log full error context for debugging
