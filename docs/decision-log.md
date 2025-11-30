# Altair Decision Log

> **Architectural Decision Records (ADRs)** — Why we made the choices we made

---

## Quick Reference

| ADR                                        | Decision                              | Date       |
| ------------------------------------------ | ------------------------------------- | ---------- |
| [ADR-001](#adr-001-surrealdb-over-sqlite)  | SurrealDB over SQLite                 | 2024-11-29 |
| [ADR-002](#adr-002-tauri-over-electron)    | Tauri over Electron                   | 2024-11-29 |
| [ADR-003](#adr-003-rust-backend)           | Rust for backend                      | 2024-11-29 |
| [ADR-004](#adr-004-s3-compatible-storage)  | S3-compatible object storage          | 2024-11-29 |
| [ADR-005](#adr-005-local-embeddings)       | Local ONNX embeddings as core         | 2024-11-29 |
| [ADR-006](#adr-006-plugin-architecture)    | Plugin architecture for Auth and AI   | 2024-11-29 |
| [ADR-007](#adr-007-last-write-wins-sync)   | Last-Write-Wins sync strategy         | 2024-11-29 |
| [ADR-008](#adr-008-tauri-ipc-over-rest)    | Tauri IPC over REST for desktop       | 2024-11-29 |
| [ADR-009](#adr-009-inbox-pattern-capture)  | Inbox pattern for Quick Capture       | 2024-11-29 |
| [ADR-010](#adr-010-soft-delete)            | Soft delete with configurable cascade | 2024-11-29 |
| [ADR-011](#adr-011-shared-location-domain) | Shared location domain across apps    | 2024-11-29 |
| [ADR-012](#adr-012-global-tags)            | Global tags with optional namespace   | 2024-11-29 |

---

## ADR-001: SurrealDB over SQLite

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need embedded database with sync capability for
offline-first apps

### Decision

Use SurrealDB 2.x as the primary database.

### Options Considered

| Option                 | Pros                                                        | Cons                                                  |
| ---------------------- | ----------------------------------------------------------- | ----------------------------------------------------- |
| **SQLite + cr-sqlite** | Battle-tested, CRDT sync                                    | No native graph, no vector search, complex sync setup |
| **SurrealDB**          | Native graph, vector search, change feeds, embedded + cloud | Less mature, smaller ecosystem                        |
| **PouchDB/CouchDB**    | Proven sync                                                 | JavaScript-only, no graph queries                     |

### Rationale

- **Native graph queries** — Quest→Note→Item relationships without
  JOIN tables
- **Built-in vector search** — HNSW indexes for semantic search,
  no external service
- **Change feeds** — Built-in change tracking enables simple sync
  without CRDT complexity
- **Same database everywhere** — Embedded (desktop/mobile) and
  server (cloud) modes
- **SurrealQL** — Powerful query language with graph traversal

### Consequences

- ✅ Simpler data model (graph edges vs junction tables)
- ✅ Semantic search works out of box
- ✅ Sync implementation is straightforward
- ⚠️ Less mature than SQLite — may hit edge cases
- ⚠️ Smaller community for troubleshooting
- 🔄 Mitigation: SQLite fallback path exists if needed

---

## ADR-002: Tauri over Electron

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need cross-platform desktop framework with mobile
support

### Decision

Use Tauri 2.0 with Svelte for all frontend apps.

### Options Considered

| Option       | Bundle Size | RAM Usage | Mobile        |
| ------------ | ----------- | --------- | ------------- |
| **Electron** | ~100MB+     | 200-300MB | No            |
| **Tauri**    | ~10MB       | ~50MB     | Yes (Android) |
| **Flutter**  | ~15MB       | ~80MB     | Yes           |

### Rationale

- **10x smaller bundles** — ~10MB vs 100MB+
- **4x less RAM** — ~50MB vs 200-300MB
- **Native performance** — Uses system webview
- **Android support** — Tauri 2.0 supports Android (same codebase)
- **Rust backend** — Aligns with SurrealDB driver language

### Consequences

- ✅ Lightweight apps, fast startup
- ✅ Single codebase for desktop + mobile
- ✅ Rust backend for type safety
- ⚠️ Tauri mobile is newer (production since Oct 2024)
- ⚠️ System webview differences across platforms
- 🔄 Mitigation: Native fallback for critical mobile features
  if needed

---

## ADR-003: Rust Backend

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need backend language for local service handling DB,
sync, embeddings

### Decision

Use Rust with Axum for the local backend service.

### Options Considered

| Option              | Pros                                      | Cons                           |
| ------------------- | ----------------------------------------- | ------------------------------ |
| **Rust**            | Same as Tauri/SurrealDB, no GC, type-safe | Steeper learning curve         |
| **TypeScript/Node** | Familiar, fast iteration                  | GC pauses, different ecosystem |
| **Go**              | Fast, simple                              | Different language from Tauri  |

### Rationale

- **Language consistency** — Same as Tauri sidecar and
  SurrealDB driver
- **No GC pauses** — Important for timer accuracy (Pomodoro, etc.)
- **Type safety** — Catches errors at compile time
- **Performance** — Native speed for embeddings and sync
- **tauri-specta** — Auto-generates TypeScript types from Rust

### Consequences

- ✅ No context switching between languages
- ✅ Predictable performance
- ✅ Strong type safety across boundaries
- ⚠️ Slower iteration than scripting languages
- ⚠️ Steeper learning curve for contributors

---

## ADR-004: S3-Compatible Storage

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need object storage for photos, audio, video, documents

### Decision

Use S3-compatible API with Minio locally, any S3 provider for
cloud.

### Options Considered

| Option                             | Pros                   | Cons              |
| ---------------------------------- | ---------------------- | ----------------- |
| **Local filesystem**               | Simple                 | No sync, no cloud |
| **S3-compatible (Minio)**          | Standard API, portable | Extra service     |
| **Proprietary (Firebase Storage)** | Managed                | Vendor lock-in    |

### Rationale

- **Standard API** — Same code works with Minio, Backblaze B2,
  Cloudflare R2, AWS S3
- **Portable** — Can switch providers by changing config
- **Presigned URLs** — Direct upload without proxying through backend
- **Minio embedded** — Runs locally for offline-first

### Consequences

- ✅ No vendor lock-in
- ✅ Easy to switch providers (Backblaze B2 is ~$6/TB/mo)
- ✅ Direct uploads reduce backend load
- ⚠️ Extra service to run locally
- 🔄 Mitigation: Minio is lightweight (~50MB)

---

## ADR-005: Local Embeddings

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need vector embeddings for semantic search

### Decision

Local ONNX embeddings (all-MiniLM-L6-v2) as core infrastructure,
always enabled.

### Options Considered

| Option                 | Latency   | Cost           | Privacy            |
| ---------------------- | --------- | -------------- | ------------------ |
| **Cloud API (OpenAI)** | 200-500ms | ~$0.0001/embed | Data leaves device |
| **Local ONNX**         | 50-100ms  | Free           | Fully local        |
| **Optional/hybrid**    | Varies    | Varies         | Configurable       |

### Rationale

- **Minimal overhead** — ~25MB model, ~50-100ms latency
- **No privacy concerns** — Data never leaves device
- **Zero cost** — No API calls, no rate limits
- **Small storage** — ~1.5KB per note embedding
- **Works offline** — Semantic search always available

### Consequences

- ✅ Semantic search works out of box
- ✅ No configuration required
- ✅ Privacy by default
- ⚠️ Desktop generates embeddings, mobile receives via sync
  (battery constraint)
- ⚠️ Model quality may be lower than cloud options
- 🔄 Mitigation: Can add cloud provider option later for
  power users

---

## ADR-006: Plugin Architecture

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need extensibility for auth methods and AI providers

### Decision

Use trait-based plugin architecture for both Auth and AI providers.

### Options Considered

| Option                     | Pros                    | Cons                              |
| -------------------------- | ----------------------- | --------------------------------- |
| **Hardcoded providers**    | Simple                  | Can't extend without code changes |
| **Plugin architecture**    | Extensible, user choice | More complex                      |
| **External plugin system** | Maximum flexibility     | Overkill, security concerns       |

### Rationale

- **Extensibility** — Add new providers without core code changes
- **User choice** — Users pick their preferred auth and AI
- **Future-proof** — New providers (Groq, Gemini) easy to add
- **Clean separation** — Providers are self-contained

### Consequences

- ✅ Easy to add new providers
- ✅ Users can use their preferred services
- ✅ Clean abstraction boundaries
- ⚠️ More upfront design work
- ⚠️ Plugin security requires audit
- 🔄 Mitigation: Built-in providers are trusted; user plugins
  future consideration

---

## ADR-007: Last-Write-Wins Sync

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need sync strategy for multi-device support

### Decision

Use Last-Write-Wins (LWW) with SurrealDB change feeds.

### Options Considered

| Option                | Complexity | Conflict Handling     |
| --------------------- | ---------- | --------------------- |
| **CRDTs**             | High       | Automatic merge       |
| **Last-Write-Wins**   | Low        | Latest timestamp wins |
| **Manual resolution** | Medium     | User decides          |

### Rationale

- **Single-user focus** — Conflicts are rare with one user
- **Simplicity** — LWW is straightforward to implement
- **Change feeds proven** — SurrealDB change feeds are reliable
- **Offline queue** — Handles disconnection gracefully

### Consequences

- ✅ Simple implementation
- ✅ Predictable behavior
- ✅ Works with SurrealDB change feeds
- ⚠️ Concurrent edits on same record → last one wins
- ⚠️ No field-level merge (whole record replaced)
- 🔄 Mitigation: Offline queue tracks pending changes;
  conflicts are rare in single-user

---

## ADR-008: Tauri IPC over REST

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need communication between desktop UI and local backend

### Decision

Use Tauri Commands (IPC) for desktop, REST + WebSocket for
mobile/cloud.

### Options Considered

| Option              | Overhead             | Type Safety                    |
| ------------------- | -------------------- | ------------------------------ |
| **REST everywhere** | HTTP serialization   | Manual types                   |
| **Tauri IPC**       | Direct function call | `tauri-specta` generates types |
| **gRPC**            | Binary protocol      | Proto files                    |

### Rationale

- **No HTTP overhead** — Direct IPC is faster than localhost HTTP
- **Type safety** — `tauri-specta` generates TypeScript from Rust
- **No port management** — IPC doesn't need network ports
- **Same logic** — Commands wrap same business logic as
  REST endpoints

### Consequences

- ✅ Faster desktop communication
- ✅ Type-safe across Rust/TypeScript boundary
- ✅ Simpler deployment (no port conflicts)
- ⚠️ Two communication patterns to maintain (IPC + REST)
- 🔄 Mitigation: Both call same underlying service layer

---

## ADR-009: Inbox Pattern for Quick Capture

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need zero-friction capture without disrupting flow

### Decision

Capture to inbox with deferred classification,
AI-assisted suggestions.

### Options Considered

| Option                 | Friction at Capture | Friction at Classification |
| ---------------------- | ------------------- | -------------------------- |
| **Inline destination** | High (choose now)   | None                       |
| **Inbox pattern**      | Zero (capture only) | Low (batch review)         |
| **AI auto-route**      | Zero                | Zero but less control      |

### Rationale

- **Zero decisions at capture** — One tap, no destination selection
- **Deferred classification** — Decide when you have executive
  function
- **Batch processing** — More efficient than per-capture decisions
- **AI assists** — Suggests destination, user confirms with one click
- **ADHD-friendly** — Reduces decision fatigue

### Consequences

- ✅ Capture is truly "quick"
- ✅ User maintains control over classification
- ✅ AI reduces friction without removing agency
- ⚠️ Inbox can accumulate (30-day auto-archive mitigates)
- ⚠️ Requires review step

---

## ADR-010: Soft Delete

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need deletion strategy that prevents data loss

### Decision

Soft delete everywhere with user-configurable cascade
behavior.

### Options Considered

| Option          | Recovery      | Complexity |
| --------------- | ------------- | ---------- |
| **Hard delete** | None          | Simple     |
| **Soft delete** | Full recovery | Medium     |
| **Versioning**  | Full history  | High       |

### Rationale

- **Recovery** — "Archived" items can be restored
- **User control** — Configurable cascade
  (archive children vs delete)
- **Sync-friendly** — Tombstones sync properly
- **Audit trail** — Know what was deleted and when

### Consequences

- ✅ No accidental data loss
- ✅ Users can recover mistakes
- ✅ Sync handles deletions cleanly
- ⚠️ Storage grows (archived items retained)
- 🔄 Mitigation: "Empty Archive" for permanent deletion

---

## ADR-011: Shared Location Domain

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Locations used by Inventory (required) and potentially
Knowledge/Guidance

### Decision

Single shared `location` table used across all apps with
privacy settings.

### Options Considered

| Option                | Pros             | Cons                            |
| --------------------- | ---------------- | ------------------------------- |
| **Per-app locations** | Clear separation | Duplication, no cross-reference |
| **Shared locations**  | Reuse, cross-app | Privacy concerns                |

### Rationale

- **Reuse** — Same location hierarchy for items and notes
- **Privacy-first** — Auto-tagging is opt-in per user
- **Precision control** — User chooses city/neighborhood/exact
- **Cross-app value** — "Notes from Houston office" is useful

### Consequences

- ✅ Consistent location model
- ✅ Cross-app queries possible
- ✅ User controls privacy
- ⚠️ More complex permission model
- 🔄 Mitigation: Default is off; user explicitly enables

---

## ADR-012: Global Tags

**Status:** Accepted  
**Date:** 2024-11-29  
**Context:** Need tagging system across all apps

### Decision

Global tags with optional namespace prefix.

### Options Considered

| Option                 | Cross-App Filter | Namespace       |
| ---------------------- | ---------------- | --------------- |
| **Per-domain tags**    | No               | Implicit        |
| **Global tags**        | Yes              | None            |
| **Global + namespace** | Yes              | Optional prefix |

### Rationale

- **Cross-app filtering** — Find all `#urgent` across apps
- **Simplicity** — Tags are just strings
- **Optional namespacing** — `#guidance/sprint-1` when needed
- **Auto-complete** — Suggests existing tags

### Consequences

- ✅ Simple model
- ✅ Cross-app search works
- ✅ Namespace available when needed
- ⚠️ Potential collision (same tag, different meanings)
- 🔄 Mitigation: Namespace prefix when disambiguation needed

---

## Template

```text
## ADR-XXX: [Title]

**Status:** Proposed | Accepted | Deprecated | Superseded
**Date:** YYYY-MM-DD
**Context:** [Brief description of the problem]

### Decision

[What we decided]

### Options Considered

| Option | Pros | Cons |
|--------|------|------|
| **Option A** | ... | ... |
| **Option B** | ... | ... |

### Rationale

- [Why this option was chosen]
- [Key factors in the decision]

### Consequences

- ✅ [Positive consequence]
- ⚠️ [Risk or concern]
- 🔄 [Mitigation for concerns]
```
