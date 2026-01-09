# Altair Product Requirements Document
## Core PRD: System Overview & Shared Infrastructure

| Field | Value |
|-------|-------|
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-01-08 |
| **Related Documents** | `altair-prd-guidance.md`, `altair-prd-knowledge.md`, `altair-prd-tracking.md`, `altair-design-system.md` |

---

## 1. Purpose

This document defines the product requirements for Altair, an open-source productivity ecosystem designed specifically for individuals with ADHD. It serves as the authoritative reference for system-level architecture, shared infrastructure, design principles, and cross-application integration.

For application-specific requirements, see the companion PRDs for Guidance, Knowledge, and Tracking.

---

## 2. Problem Statement

Existing productivity tools fail individuals with ADHD in predictable ways:

- **Overwhelm by design**: Most task managers encourage capturing everything, creating backlogs that trigger anxiety and avoidance.
- **Willpower-dependent**: Systems assume users can self-regulate task switching, prioritization, and focus—exactly the executive functions ADHD impairs.
- **Shame-inducing**: Incomplete tasks accumulate visibly, reinforcing negative self-perception. "Streaks" punish bad days rather than celebrating good ones.
- **Time-blind**: Few tools address the ADHD experience of time—difficulty estimating duration, losing track of time, or hyperfocusing past deadlines.
- **Fragmented ecosystems**: Knowledge, tasks, and inventory live in separate apps with no meaningful integration, increasing cognitive load.
- **Cloud-dependent**: Many tools require constant connectivity and store sensitive data on third-party servers, creating privacy concerns and offline failures.

Altair addresses these problems through externalized executive function, enforced constraints (like WIP=1), variable capacity support, and a privacy-focused self-hosted architecture that keeps your data on your own infrastructure.

---

## 3. Target Users

### Primary Audience

- **Individuals with ADHD** seeking productivity tools that work *with* their neurology rather than against it
- **Makers and DIY enthusiasts** managing projects, parts inventory, and documentation
- **Researchers and knowledge workers** building personal knowledge bases with interconnected notes

### Secondary Audience

- **Households** coordinating shared projects and inventory
- **Anyone** who has struggled with traditional productivity systems and wants a gentler, more flexible approach

### User Assumptions

- Basic computer literacy
- Minimum 4GB RAM and 1GB available storage
- Preference for privacy and local data control
- Willingness to learn new productivity patterns
- Understanding that ADHD affects executive function (even if not formally diagnosed)

---

## 4. Goals

### System Goals

1. **Externalize executive function** — The system enforces constraints (WIP limits, energy filtering, progressive disclosure) so users don't rely on willpower.
2. **Unify productivity domains** — Tasks, knowledge, and inventory share data and surface relevant connections automatically.
3. **Respect variable capacity** — Daily energy fluctuates; the system adapts rather than demanding consistency.
4. **Eliminate shame** — No failure states. Incomplete work is pausable, archivable, and recoverable without judgment.
5. **Prioritize privacy** — Self-hosted architecture with data stored on your own infrastructure.
6. **Enable portability** — Open-source (AGPL v3+) with standard data formats and no vendor lock-in.

### Non-Goals (v1)

- Real-time multi-user collaboration
- Cloud-only features (everything must work offline)
- Enterprise/team features
- Plugin marketplace (architecture included, marketplace deferred)
- Apple platform support (unless community-contributed)
- Web application
- Built-in health/medical data collection (integrate via plugins instead)

---

## 5. System Architecture

### Core Applications

| Application | Purpose | Key Differentiator |
|-------------|---------|-------------------|
| **Guidance** | Quest-Based Agile task management | WIP=1 enforcement, energy-based filtering |
| **Knowledge** | Personal knowledge management | Bidirectional linking, auto-relationship discovery |
| **Tracking** | Inventory and asset management | Photo-first capture, BoM intelligence |

### Architectural Principles

- **Privacy-focused**: Self-hosted server keeps all data on user's infrastructure
- **Offline-capable**: Desktop works fully offline; mobile requires server for AI features
- **Desktop-primary**: Windows and Linux are primary platforms; macOS nice-to-have
- **Shared data layer**: Hybrid database (SurrealDB desktop/server, SQLite mobile)
- **Plugin-extensible**: Core functionality augmented by sandboxed plugins
- **Cross-platform sync**: Real-time sync via self-hosted server

### Technology Constraints

- Desktop/Mobile: Kotlin Multiplatform with Compose Multiplatform
- Server: Ktor with kotlinx-rpc, Docker Compose deployment
- Backend: FastAPI + PostgreSQL (local)
- AI: Local models (desktop) with cloud fallback (mobile)

---

## 6. ADHD Design Principles

These principles govern all UI/UX decisions across the ecosystem. They are non-negotiable.

### 6.1 Reduce Cognitive Load

- Minimal decisions required at any step
- Clear, single paths to common actions
- Hide complexity until explicitly needed
- No feature should require remembering where it lives

### 6.2 Externalize Structure

- System-enforced limits replace willpower (WIP=1, energy filtering)
- Constraints are features, not restrictions
- The UI is a supportive partner, not a demanding boss

### 6.3 Support Variable Capacity

- Energy check-ins acknowledge daily fluctuation
- Zero-energy days are valid; rest is not failure
- Tasks filter to match current capacity
- No shame for "unproductive" days

### 6.4 Build Forgiveness Mechanisms

- No task is ever truly deleted (archive with undo)
- "Phoenix Rising" recovery for abandoned boards
- Language is neutral, never judgmental
- Streaks reward progress, don't punish gaps

### 6.5 Provide Immediate Feedback

- Visual and auditory confirmation of actions
- Progress visible at a glance
- Celebrate completions (without being annoying)

### 6.6 Use Progressive Disclosure

- Show only what's needed now
- Details expand on demand
- Advanced features hidden until relevant
- Onboarding reveals complexity gradually

### 6.7 Combat Time Blindness

- Visual timers with clear remaining time
- Progress bars for elapsed time
- Proactive reminders (user-configurable)
- Harvest rituals provide natural checkpoints

---

## 7. Visual Design System

Full specifications in `altair-design-system.md`. Summary of core elements:

### Design Philosophy

- **Calm Focus**: Soft, low-stimulation aesthetics that reduce anxiety
- **Focus-first**: Default to single-task views; complexity is opt-in
- **Gentle hierarchy**: Visual weight guides attention without shouting

### Visual Language

| Element | Specification |
|---------|---------------|
| **Palette** | Low-contrast backgrounds, readable text, accent colors for actions |
| **Typography** | Large, accessible sizes; open letterforms; generous line height |
| **Corners** | Consistent soft rounding (no sharp edges) |
| **Depth** | Subtle shadows; no harsh borders |
| **Focus indicators** | Clear highlighting for active elements |

### Component Patterns

- **Quick Capture**: Global activation, instant focus, single-action save
- **Focus Dashboard**: Single active task view with hidden complexity
- **Energy Filter**: Dynamic filtering based on current capacity
- **Standard containers**: Consistent card-based content
- **Action buttons**: Primary (blue), completion (green)
- **Timer visualizations**: Circular or bar-based progress

---

## 8. Cross-Application Integration

### 8.1 Shared Data Model

All applications access a unified local database with the following entity relationships:

```
Quest (Guidance) ←→ Note (Knowledge)
Quest (Guidance) → Item requirements (Tracking)
Note (Knowledge) ←→ Item references (Tracking)
All entities ←→ Tags (shared taxonomy)
```

### 8.2 Auto-Discovery of Relationships

The system automatically surfaces connections between entities:

| Mechanism | Description |
|-----------|-------------|
| **Semantic similarity** | Cosine similarity > 0.7 suggests relationship |
| **Fuzzy title matching** | "web dev" ≈ "web development" |
| **Smart aliasing** | "RPi" ≈ "Raspberry Pi", "JS" ≈ "JavaScript" |
| **Entity recognition** | Extract people, places, technologies from text |
| **Shared keywords** | Common terms suggest relationship |

### 8.3 Integration Points

| From | To | Integration |
|------|-----|-------------|
| Knowledge note | Guidance quest | Extract actionable items as quests |
| Guidance quest | Knowledge note | Link research/documentation to tasks |
| Knowledge note | Tracking item | Detect mentioned components/materials |
| Tracking item | Guidance quest | Reserve items for quest requirements |
| Tracking BoM | Guidance quest | Generate shopping list quests |

### 8.4 Universal Search

Single search interface queries all applications:

- Full-text search across notes, quests, items
- Semantic search using vector embeddings
- Filter by application, type, tag, date
- Results ranked by relevance and recency

### 8.5 Proactive Suggestions

The system surfaces relevant information without being asked:

- Duplicate detection across applications
- Missing material alerts (BoM vs. inventory)
- Related content suggestions
- Quest timing optimization based on dependencies

---

## 9. Plugin Architecture

### 9.1 Design Goals

- Extend functionality without modifying core
- User controls all permissions
- Plugins cannot compromise privacy principles
- Sandboxed execution prevents data leaks

### 9.2 Plugin Types

| Category | Examples |
|----------|----------|
| **Cloud sync** | Google Drive, Dropbox, Nextcloud, WebDAV |
| **AI services** | OpenAI, Claude, local model alternatives |
| **Import sources** | Notion, Obsidian, Todoist, Trello |
| **Export formats** | PDF, Word, specialized formats |
| **Automation** | IFTTT, Zapier, n8n, custom scripts |
| **Health integration** | Google Fit, Fitbit (read-only, correlation only) |

### 9.3 Permission Model

- Plugins request specific permissions at install
- Users approve/deny each permission
- Permissions revocable at any time
- Audit log of plugin data access

### 9.4 Health Integration Philosophy

Health data requires special handling:

- **No built-in health tracking** — Avoids HIPAA/privacy concerns
- **Read-only integration** — Pull data from dedicated health apps
- **Correlation, not diagnosis** — Data used only for pattern recognition (e.g., sleep quality → energy levels)
- **User controls data flow** — Explicit opt-in for each data type

---

## 10. Cross-Platform Synchronization

### 10.1 Sync Strategy

| Aspect | Approach |
|--------|----------|
| **Trigger** | Real-time when online, queued when offline |
| **Scope** | Between user's own devices only |
| **Conflict resolution** | Three-way merge with manual conflict UI |
| **Selective sync** | User-defined rules per data type |
| **Binary handling** | Efficient delta sync for images/video |

### 10.2 Cloud Backup Options

Backup is optional and user-controlled:

| Provider | Platform |
|----------|----------|
| iCloud | iOS/macOS |
| Google Drive | All platforms |
| Dropbox | All platforms |
| OneDrive | All platforms |
| Nextcloud | Self-hosted option |
| WebDAV | Generic protocol |

### 10.3 Backup Features

- **Selective backup**: Choose what to back up (full, settings only, specific apps)
- **Encryption**: End-to-end encryption before upload
- **Extensible**: Plugin architecture for additional providers

### 10.4 Offline Capabilities

- Full feature parity offline
- Complete local database mirror
- Action queue for pending sync
- Conflict detection and flagging
- Intelligent retry with exponential backoff

---

## 11. Mobile Platform Features

Application-specific mobile features are documented in each app's PRD. This section covers shared mobile infrastructure.

### 11.1 System Integration

| Feature | Capability |
|---------|------------|
| **Push notifications** | Quest reminders, timer completions, energy check-ins, harvest reminders, low-stock alerts, maintenance due dates |
| **Widgets** | Active quest, quick capture, energy status, daily stats, next up preview |
| **Shortcuts/Intents** | Voice commands, automation triggers, quick actions |
| **Biometric security** | Face ID, Touch ID, fingerprint, app lock |

### 11.2 Accessibility

- VoiceOver/TalkBack support
- Dynamic type support
- High contrast mode
- Reduce motion options
- Switch control compatibility

### 11.3 Platform-Specific Implementations

**Android:**
- Wear OS companion app
- Google Assistant integration
- Material You theming
- Work profile support
- Tablet optimization

**iOS (if community-contributed):**
- Apple Watch companion app
- Siri Shortcuts integration
- Handoff to Mac desktop

### 11.4 Mobile-Desktop Handoff

- **Continue on desktop**: Deep links to current context
- **Universal clipboard**: Copy on mobile, paste on desktop
- **Shared notifications**: Dismiss on one, cleared everywhere
- **Session handoff**: Resume timer/focus session across devices
- **File handoff**: Start capture on mobile, finish on desktop

### 11.5 Mobile Limitations

**Technical constraints:**
- Local AI models unavailable (use cloud alternatives)
- Sandboxed file system access
- Platform restrictions on background processing
- Memory constraints for large graphs
- Plugin system limited or unavailable

**Intentional simplifications:**
- Bulk operations simplified for touch
- Subset of advanced settings
- Common export formats only
- Keyboard shortcuts limited to external keyboards

---

## 12. Performance Requirements

### 12.1 Desktop Targets

| Metric | Target |
|--------|--------|
| App launch | < 2 seconds |
| Quest creation | < 500ms |
| Semantic search | < 1 second |
| Mind map node render | < 100ms |
| Relationship discovery | > 90% accuracy |
| Inventory detection | < 500ms |

### 12.2 Mobile Targets

| Metric | Target |
|--------|--------|
| App launch (cold) | < 3 seconds |
| App launch (warm) | < 1 second |
| Camera capture ready | < 2 seconds |
| Sync latency (online) | < 5 seconds |
| Search (10k items) | < 2 seconds |
| Graph render (100 nodes) | < 1 second |
| Battery impact (typical use) | < 5% daily |
| Base app size | < 100 MB |
| Offline cache | Configurable 100 MB – 2 GB |

### 12.3 UX Targets

| Metric | Target |
|--------|--------|
| Quest completion time | -30% with focus mode |
| Auto-discovered relationships | 5+ links per note |
| Graph navigation | < 2 clicks to any node |
| Capture speed | < 3 seconds to start |
| Focus mode task completion | > 80% |

---

## 13. Constraints & Assumptions

### 13.1 Technical Constraints

- Desktop-first development (Windows, Linux required; macOS nice-to-have)
- Mobile quick capture focus (Android required; iOS nice-to-have)
- Privacy-focused self-hosted architecture
- Single-user focus (no real-time collaboration in v1)
- Open-source license: AGPL v3 or later

### 13.2 Assumptions

- Users have basic computer literacy
- Users have minimum 4GB RAM, 1GB storage
- Users prefer privacy and local data control
- Users are willing to learn new productivity patterns
- ADHD users benefit from externalized executive function

### 13.3 Explicit Non-Scope (v1)

| Item | Rationale |
|------|-----------|
| Web application | Desktop/mobile focus; future consideration |
| Real-time collaboration | Single-user v1; adds significant complexity |
| Third-party cloud storage | Data stays on user's server |
| Enterprise/team features | Out of scope for v1 |
| Plugin marketplace | Architecture ready; marketplace deferred |
| Advanced mobile AI | Use server AI; local models infeasible on mobile |
| All cloud providers | Core providers only; rest via plugins |
| Health data collection | HIPAA concerns; integrate via plugins |

---

## 14. Feature Priority (System-Level)

### Critical (Must Have)

1. Privacy-focused self-hosted architecture
2. Cross-app data integration
3. Auto-relationship discovery
4. Offline mode with sync (desktop)
5. Push notifications (mobile)

### High Priority

1. Semantic search across apps
2. Plugin architecture (core API)
3. Cloud backup (core providers)
4. Mobile-desktop handoff
5. Universal quick capture

### Medium Priority

1. Plugin architecture (additional providers)
2. Advanced sync conflict resolution
3. Watch/Wear OS companion apps
4. Proactive suggestions

---

## 15. Open Questions

1. **Sync conflict UI**: What's the optimal UX for presenting merge conflicts to non-technical users?
2. **Plugin sandboxing**: What's the right balance between plugin capability and security isolation?
3. **AI model selection**: Which local models provide acceptable quality on modest hardware?
4. **iOS contribution model**: How do we structure the project to enable community iOS contribution?

---

## Appendix A: Document Cross-References

| Document | Scope |
|----------|-------|
| `altair-prd-guidance.md` | Guidance application requirements |
| `altair-prd-knowledge.md` | Knowledge application requirements |
| `altair-prd-tracking.md` | Tracking application requirements |
| `altair-design-system.md` | Calm Focus visual design system |

---

## Appendix B: Glossary

| Term | Definition |
|------|------------|
| **Quest** | A discrete, completable task in Guidance |
| **Epic** | A large initiative containing multiple Quests |
| **Harvest** | Weekly reflection and planning ritual |
| **WIP** | Work In Progress; default limit is 1 active Quest |
| **Energy** | Subjective daily capacity rating (1-5 scale) |
| **BoM** | Bill of Materials; list of required items for a project |
| **Self-hosted** | Architecture where data stays on user-controlled infrastructure |
| **Phoenix Rising** | Recovery process for abandoned/overwhelming boards |
