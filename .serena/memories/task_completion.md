# Task Completion Checklist

## Before Committing

### Rust Code

```bash
cargo fmt        # Format code
cargo clippy     # Lint and catch issues
cargo test       # Run tests
```

### TypeScript/Svelte Code

```bash
pnpm lint        # Lint all packages
pnpm test        # Run tests
```

### Type Synchronization

If Tauri commands changed:
```bash
cargo run --bin generate-bindings
```

## Code Review Checklist

- [ ] Matches spec/requirements?
- [ ] Follows project patterns (see style_conventions.md)?
- [ ] Tests included?
- [ ] Errors handled gracefully (no unwrap)?
- [ ] Types correct and complete?
- [ ] Security concerns addressed?
- [ ] Performance acceptable?

## Terminology Verification

Ensure all code and comments use correct terminology:
- Quest (not Task/Todo)
- Campaign (not Project/Epic)
- Note (not Document/Page)
- Archive (not Delete)

## Database Changes

If schema changed:
1. Create migration: `migrations/NNN_description.surql`
2. Ensure `CHANGEFEED 7d` on new tables
3. Update `packages/db/` query functions
4. Run `cargo run --bin migrate`

## Commit Message Format

```
type(scope): description

[spec-id] if implementing a spec

Examples:
feat(guidance): add quest CRUD commands [guidance-001]
fix(sync): handle offline queue overflow
docs: update glossary with new terms
refactor(db): extract query builders
```

## PR Requirements

1. Reference spec if applicable: "Implements guidance-001-quest-crud"
2. Include what/why/how sections
3. Document testing approach
4. Note any architectural decisions
