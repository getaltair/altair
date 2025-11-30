# Suggested Commands

## Development (Once Implemented)

```bash
# Start backend + all apps
pnpm dev

# Start specific app
pnpm --filter guidance dev

# Run backend only
cd backend && cargo run
```

## Testing

```bash
# All tests
pnpm test

# Rust tests
cd backend && cargo test

# Specific package
pnpm --filter @altair/db test
```

## Code Quality

```bash
# Rust - REQUIRED before commits
cargo fmt
cargo clippy

# TypeScript/Svelte - REQUIRED before commits
pnpm lint
```

## Type Generation

```bash
# Regenerate TypeScript bindings from Rust
cargo run --bin generate-bindings

# Types output to packages/bindings/
```

## Database Operations

```bash
# Run migrations (usually automatic on backend start)
cargo run --bin migrate

# Connect to embedded DB via standalone server (debugging only)
surreal start --user root --pass root surrealkv:~/.local/share/altair/db
surreal sql --conn ws://localhost:8000 --ns altair --db main

# Reset database (development only)
rm -rf ~/.local/share/altair/db && cargo run --bin migrate
```

## Spectrena Workflow

```bash
# Create specification from feature description
/spectrena.specify

# Resolve ambiguities in spec
/spectrena.clarify

# Generate implementation plan from spec
/spectrena.plan

# Break plan into executable tasks
/spectrena.tasks

# Execute task with code generation
/spectrena.implement
```

## Git Commit Convention

```bash
type(scope): description

# Examples:
feat(guidance): add quest CRUD commands [guidance-001]
fix(sync): handle offline queue overflow
docs: update CLAUDE.md with new patterns
refactor(db): extract query builders
```

## System Utilities (Linux)

```bash
# File search (prefer fd over find)
fd <pattern>

# Content search (prefer rg over grep)
rg <pattern>

# Package management (Arch Linux)
yay -S <package>
```

## Ports Reference

| Service | Port |
|---------|------|
| Backend API | 3847 |
| SurrealDB | 8000 |
| Minio | 9000 |
| Minio Console | 9001 |
