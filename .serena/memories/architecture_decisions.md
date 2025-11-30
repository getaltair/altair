# Key Architecture Decisions

See `docs/decision-log.md` for full rationale on each decision.

## 1. SurrealDB over SQLite

**Decision:** Use SurrealDB 2.x embedded + cloud

**Why:**
- Native graph queries for relationships
- Built-in vector search for semantic search
- Change feeds enable sync
- Embedded mode for offline, cloud for sync

## 2. Tauri IPC over REST

**Decision:** Desktop apps use Tauri commands, not HTTP

**Why:**
- Type-safe via tauri-specta
- No HTTP overhead for local calls
- Automatic TypeScript generation
- Mobile uses REST (separate backend)

## 3. Local Embeddings Always On

**Decision:** ~25MB ONNX model (all-MiniLM-L6-v2) ships with app

**Why:**
- Semantic search works offline
- No cloud dependency
- Privacy preserved
- 384-dimension vectors

## 4. Last-Write-Wins Sync

**Decision:** Simple conflict resolution, single-user focus

**Why:**
- Single user = rare conflicts
- Simpler than CRDTs
- Change feeds track all changes
- User can resolve rare conflicts manually

## 5. Plugin Architecture

**Decision:** Auth and AI providers are trait-based plugins

**Why:**
- No vendor lock-in
- User choice (local auth vs OAuth vs OIDC)
- User choice (Ollama vs OpenAI vs Anthropic)
- New providers added without core changes

## 6. Soft Delete Everywhere

**Decision:** `status: archived`, never hard delete

**Why:**
- ADHD users act impulsively
- Recovery always possible
- Sync needs tombstones
- "Empty Archive" for permanent delete

## Domain Model

```
Campaign →contains→ Quest →references→ Note
                         →requires→ Item
Note →links_to→ Note (wiki-links, bidirectional)
Note →documents→ Item
Item →stored_in→ Location
```

**Key Rules:**
- All entities have `owner` field (record-level auth)
- All tables have `CHANGEFEED 7d` (sync support)
- References are soft links (deleting source doesn't delete target)
