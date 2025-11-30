# Code Style & Conventions

## Terminology (STRICT)

Always use project terminology from `docs/glossary.md`:

| ✅ Use | ❌ Don't Use |
|--------|-------------|
| Quest | Task, Todo, Ticket |
| Campaign | Project, Epic |
| Note | Document, Page |
| Item | Product, Asset |
| Capture | Inbox item, Draft |
| Archive | Delete |
| Energy levels: Tiny, Small, Medium, Large, Huge | Low/Medium/High |

## File Naming

| Type | Convention | Example |
|------|------------|---------|
| Rust modules | snake_case | `quest_commands.rs` |
| Svelte components | PascalCase | `QuestCard.svelte` |
| TypeScript utils | camelCase | `formatDate.ts` |
| Specs | `component-NNN-slug` | `guidance-001-quest-crud` |
| Migrations | `NNN_description.surql` | `001_initial_schema.surql` |

## Rust Style

- Use `Result<T, ApiError>` for fallible operations
- Prefer `async` for I/O operations
- Use `#[instrument]` from `tracing` for observability
- Run `cargo fmt` and `cargo clippy` before committing
- No `unwrap()` in production paths - handle errors properly

### Tauri Command Pattern

```rust
#[tauri::command]
async fn operation_name(
    state: State<'_, AppState>,
    param: Type,
) -> Result<Response, ApiError> {
    // Implementation
}
```

## TypeScript/Svelte Style

- Use generated types from `packages/bindings/`
- Prefer `invoke()` over manual type definitions
- Use Tailwind for styling (utility classes only)
- Run `pnpm lint` before committing

## SurrealQL Style

- Use `SCHEMAFULL` tables (explicit schema)
- Add `ASSERT` constraints for validation
- Include `CHANGEFEED 7d` on all synced tables
- Use graph edges for relationships (`->edge->`)

### Database Schema Pattern

```surql
DEFINE TABLE entity SCHEMAFULL CHANGEFEED 7d;
DEFINE FIELD owner ON entity TYPE record<user>;
DEFINE FIELD status ON entity TYPE string DEFAULT 'active';
DEFINE FIELD created_at ON entity TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON entity TYPE datetime VALUE time::now();
DEFINE FIELD device_id ON entity TYPE string;
```

## Architectural Constraints (STRICT)

Do NOT violate without updating `docs/decision-log.md`:

1. Desktop uses Tauri IPC, NOT REST
2. All tables have CHANGEFEED for sync
3. Soft delete only (status: archived), NO hard deletes
4. Local ONNX embeddings, no cloud dependency for search
5. Plugin architecture for auth/AI (use traits)
