<!--
SYNC IMPACT REPORT
==================
Version Change: 0.0.0 → 1.0.0 (MAJOR - initial constitution)
Modified Principles: N/A (new)
Added Sections: Core Principles (7), ADHD Design Requirements, Technology Constraints, Governance
Removed Sections: None
Templates Requiring Updates:
  - .spectrena/templates/plan-template.md: ✅ No updates needed (generic template)
  - .spectrena/templates/spec-template.md: ✅ No updates needed (generic template)
  - .spectrena/templates/tasks-template.md: ✅ No updates needed (generic template)
Follow-up TODOs: None
-->

# Altair Constitution

> **Governing principles for the ADHD-focused productivity ecosystem**

## Core Principles

### I. ADHD-First Design (NON-NEGOTIABLE)

Every feature MUST externalize executive function, not demand it. This is the foundational
principle that shapes all design decisions.

**Rules:**
- Reduce cognitive load — minimize decisions, provide clear paths
- External structure — system-enforced limits (WIP=1), not willpower
- Variable capacity — adapt to daily energy levels via Energy Management
- Forgiveness mechanisms — no shame for incomplete tasks, streaks have grace periods
- Immediate feedback — visual and auditory confirmations
- Progressive disclosure — show only what's needed
- Time blindness support — visual timers, reminders, progress indicators

**Rationale:** Users with ADHD need the system to be a supportive partner, not a demanding
boss. The UI must reduce friction, not add it.

### II. Local-First Architecture

All core functionality MUST work offline. Cloud sync is optional enhancement, not requirement.

**Rules:**
- Embedded database (SurrealDB) runs locally on every device
- Local S3-compatible storage (Minio) for media
- Local ONNX embeddings (~25MB) for semantic search — no cloud dependency
- Sync is additive — features work without it
- User data stays on device unless user explicitly enables cloud backup
- Privacy by default — embeddings never leave device

**Rationale:** Per ADR-001, ADR-004, ADR-005. Users must trust the system with their
data. Offline-first ensures the app is useful anywhere, anytime.

### III. Quest-Based Agile (QBA) Board

The six-column Kanban with strict WIP=1 is the heart of Guidance. Do not compromise it.

**Rules:**
- Six columns: Idea Greenhouse → Quest Log → This Cycle (max 1) → Next Up (max 5) →
  In Progress (max 1) → Harvested
- WIP=1 strictly enforced — only one quest can be "In Progress" at any time
- Energy levels required on every quest (Tiny/Small/Medium/Large/Huge)
- Focus mode provides distraction-free single-task view
- Weekly Harvest ritual for reflection without judgment

**Rationale:** Per requirements.md Section 1.1. ADHD brains struggle with task switching.
Enforcing WIP=1 at the system level removes the burden of self-regulation.

### IV. Soft Delete Everywhere

Never hard delete user data. All deletions MUST be recoverable.

**Rules:**
- Use `status: archived` for deletion, never DROP/DELETE
- Archived items visible in Archive view
- Cascade behavior is user-configurable (archive children or move to parent)
- "Empty Archive" for permanent deletion (explicit user action)
- All tables include `CHANGEFEED 7d` for sync support
- Tombstones sync to other devices

**Rationale:** Per ADR-010. ADHD users may impulsively delete and regret it. The system
must protect against permanent data loss.

### V. Graph Relationships Over Junction Tables

Use SurrealDB graph edges for entity relationships, not SQL-style junction tables.

**Rules:**
- Campaign →contains→ Quest, Quest →references→ Note, Note →links_to→ Note
- Quest →requires→ Item (Bill of Materials)
- Item →stored_in→ Location
- has_attachment edge connects any entity to Attachment
- All edges are SCHEMAFULL with CHANGEFEED 7d
- References are soft links — deleting source does not delete target

**Rationale:** Per ADR-001 and domain-model.md. SurrealDB's native graph queries
simplify the data model and enable powerful traversals without JOIN complexity.

### VI. Type Safety Across Boundaries

Rust types MUST generate TypeScript types. No manual type synchronization.

**Rules:**
- Use tauri-specta to generate TypeScript from Rust structs
- Desktop apps use Tauri IPC commands, not HTTP (per ADR-008)
- Mobile apps use REST + WebSocket to cloud backend
- All DTOs derive Serialize, Deserialize, and specta::Type
- Generated bindings live in packages/bindings/

**Rationale:** Per ADR-008 and technical-architecture.md. Manual type synchronization
leads to drift. Codegen eliminates entire classes of bugs.

### VII. Plugin Architecture for Extensibility

Auth providers and AI providers MUST use trait-based plugins. No hardcoding.

**Rules:**
- AuthProvider trait: authenticate, validate, refresh, logout
- AiProvider trait: complete, transcribe, generate_image (optional per provider)
- Built-in providers: local auth, OAuth (Google, GitHub), OIDC
- AI is OPTIONAL — core features work without any AI provider
- Rate limiting protects against API cost overruns
- API keys stored in OS keychain, never in config files

**Rationale:** Per ADR-006. Users have different preferences and constraints. Plugin
architecture lets them choose without forking the codebase.

## ADHD Design Requirements

These requirements apply to ALL user-facing features:

| Requirement | Implementation |
|------------|----------------|
| Zero-friction capture | Quick Capture: one tap, no destination choice |
| Deferred classification | Inbox pattern — decide later when you have spoons |
| Energy-based filtering | Show only tasks matching current energy level |
| Visual timers | Progress bars, not just countdown numbers |
| Celebration | XP, achievements, level-ups for completed work |
| No shame mechanics | Streaks have 24-hour grace period |
| Opt-out gamification | Users can disable XP/achievements entirely |
| Auto-discovery | System suggests relationships, user confirms |

## Technology Constraints

| Layer | Technology | Why |
|-------|------------|-----|
| Database | SurrealDB 2.x | Graph queries, vector search, change feeds (ADR-001) |
| Object Storage | S3-compatible | Portable, no vendor lock-in (ADR-004) |
| Desktop | Tauri 2.0 + Svelte | 10x smaller than Electron (ADR-002) |
| Mobile | Tauri 2.0 Android | Same codebase as desktop |
| Backend | Rust + Axum | No GC pauses, same language as Tauri (ADR-003) |
| Embeddings | Local ONNX | Always-on, no cloud dependency (ADR-005) |
| Sync | LWW + Change Feeds | Simple, single-user optimized (ADR-007) |

## Terminology

Use these terms consistently. See docs/glossary.md for full definitions.

| Use | Don't Use | Why |
|-----|-----------|-----|
| Quest | Task, Todo | Quest has energy cost, adventure framing |
| Campaign | Project, Epic | Campaign contains quests |
| Note | Document, Page | Note is the PKM entity |
| Item | Product, Asset | Item is the inventory entity |
| Capture | Inbox item | Capture is pending classification |
| Archive | Delete | Soft delete, recoverable |
| Harvest | Complete, Done | Weekly ritual framing |

## Governance

### Amendment Procedure

1. Propose change via PR with rationale
2. Update relevant ADR in docs/decision-log.md if architectural
3. Review against ADHD-First principle (Principle I)
4. If approved, increment constitution version
5. Update all dependent artifacts (templates, docs)

### Version Policy

- **MAJOR**: Backward-incompatible principle changes, principle removal
- **MINOR**: New principle added, significant guidance expansion
- **PATCH**: Clarifications, typo fixes, non-semantic refinements

### Compliance Verification

All PRs MUST verify:
- [ ] ADHD-First: Does this reduce cognitive load?
- [ ] Local-First: Does this work offline?
- [ ] Soft Delete: Is data recoverable?
- [ ] Type Safety: Are types generated, not manual?
- [ ] Terminology: Are terms used correctly per glossary?

### Reference Documents

| Document | Purpose |
|----------|---------|
| docs/requirements.md | Full feature specifications |
| docs/technical-architecture.md | Implementation details |
| docs/domain-model.md | Entity relationships |
| docs/decision-log.md | ADRs with rationale |
| docs/glossary.md | Consistent terminology |
| docs/design-system.md | Calm Focus visual language |
| CLAUDE.md | Runtime development guidance |

**Version**: 1.0.0 | **Ratified**: 2025-11-29 | **Last Amended**: 2025-11-29
