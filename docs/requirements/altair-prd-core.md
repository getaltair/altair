# Altair Product Requirements Document

## Core PRD: System Overview & Shared Infrastructure

| Field                 | Value                                                                                                    |
| --------------------- | -------------------------------------------------------------------------------------------------------- |
| **Version**           | 2.0                                                                                                      |
| **Status**            | Draft                                                                                                    |
| **Last Updated**      | 2026-01-13                                                                                               |
| **Related Documents** | `altair-prd-guidance.md`, `altair-prd-knowledge.md`, `altair-prd-tracking.md`, `altair-design-system.md` |

---

## 1. Purpose

This document defines the product requirements for Altair, an open-source life management ecosystem that unifies task execution, knowledge capture, and inventory tracking into a single coherent system. Altair is designed for anyone managing a complex life, with thoughtful defaults that work especially well for neurodivergent users.

Rather than forcing users to maintain separate apps for separate concerns, Altair recognizes that life is interconnected—a home renovation project involves tasks, research, and materials simultaneously. Context flows naturally between domains, and the system adapts to your capacity rather than demanding consistency.

Core design principles include enforced focus (WIP=1 by default, adjustable), energy-aware filtering, and shame-free recovery from incomplete work. These features benefit everyone, but are particularly valuable for users with ADHD or executive function challenges.

For application-specific requirements, see the companion PRDs for Guidance, Knowledge, and Tracking.

---

## 2. Problem Statement: Universal Challenges

Existing productivity tools fail users in predictable ways:

- **Fragmented ecosystems**: Tasks live in one app, notes in another, inventory in a third. Users must manually maintain connections and constantly context-switch between tools.
- **Project/task false dichotomy**: Tools are either heavyweight project management (overwhelming) or simple task lists (insufficient for complex life). Nothing serves the middle ground of "I have stuff going on that's more than a checklist but I'm not running a business."
- **No capacity awareness**: Systems assume consistent daily output. Bad days, low energy, and variable schedules aren't accommodated—you either keep up or fall behind.
- **Cloud-dependent**: Many tools require constant connectivity and store sensitive data on third-party servers, creating privacy concerns and offline failures.
- **Configuration overhead**: Powerful tools require extensive setup and ongoing maintenance. Simple tools lack flexibility. Few find the right balance of "works out of the box" and "adapts to my needs."

---

## 3. Problem Statement: ADHD & Executive Function Challenges

In addition to universal challenges, individuals with ADHD face specific obstacles:

- **Overwhelm by design**: Most task managers encourage capturing everything, creating backlogs that trigger anxiety and avoidance.
- **Willpower-dependent**: Systems assume users can self-regulate task switching, prioritization, and focus—exactly the executive functions ADHD impairs.
- **Shame-inducing**: Incomplete tasks accumulate visibly, reinforcing negative self-perception. "Streaks" punish bad days rather than celebrating good ones.
- **Time blindness**: Few tools address the ADHD experience of time—difficulty estimating duration, losing track of time, or hyperfocusing past deadlines.

Altair addresses both universal and ADHD-specific challenges through:

- **Unified context**: Tasks, knowledge, and inventory share data via Initiatives
- **Externalized executive function**: System-enforced constraints (WIP=1 default, energy filtering) so users don't rely on willpower
- **Variable capacity support**: Energy system adapts to daily fluctuations
- **Shame-free recovery**: No failure states; incomplete work is pausable without judgment
- **Privacy-focused architecture**: Self-hosted server keeps data on your infrastructure

---

## 4. Target Users

### Primary Audience

- **Anyone managing a complex life** with multiple ongoing projects, areas of responsibility, and recurring routines
- **Makers and DIY enthusiasts** managing projects, parts inventory, and documentation
- **Knowledge workers** building personal knowledge bases with interconnected notes
- **Individuals with ADHD or executive function challenges** seeking tools that work _with_ their neurology

### Secondary Audience

- **Households and other groups** coordinating shared projects and inventory
- **Anyone** who has struggled with traditional productivity systems and wants a gentler, more flexible approach
- **People who've tried GTD, Notion, Obsidian, etc.** and found them either too rigid or too unstructured

### User Assumptions

- Basic computer literacy
- Minimum 4GB RAM and 1GB available storage
- Willingness to learn new patterns (but shouldn't need to configure extensively)
- Access to both mobile device and desktop/laptop (mobile for daily ops, desktop for deeper work)

---

## 5. Goals

### System Goals

1. **Externalize executive function** — The system enforces constraints (WIP limits, energy filtering, progressive disclosure) so users don't rely on willpower.
2. **Unify life domains** — Tasks, knowledge, and inventory share context through Initiatives. Relevant information surfaces automatically without manual linking.
3. **Respect variable capacity** — Daily energy fluctuates; the system adapts rather than demanding consistency.
4. **Eliminate shame** — No failure states. Incomplete work is pausable, archivable, and recoverable without judgment. Projects can become ongoing areas without stigma.
5. **Support the daily rhythm** — Mobile-first for morning rituals, capture, and completion. Desktop for reflection, planning, and deep work.
6. **Prioritize privacy** — Self-hosted architecture with data stored on your own infrastructure.
7. **Enable portability** — Open-source (AGPL v3+) with standard data formats and no vendor lock-in.

### Non-Goals (v1)

- Real-time multi-user collaboration
- Cloud-only features (everything must work offline on mobile and desktop)
- Enterprise/team features
- Plugin marketplace (architecture included, marketplace deferred)
- Full-featured web application (minimal view-only dashboard acceptable)
- Built-in health/medical data collection (integrate via plugins instead)
- Dependency graphs, Gantt charts, or traditional project management tooling

---

## 6. System Architecture

### Core Applications

| Application   | Purpose                        | Primary Platform | Key Differentiator                          |
| ------------- | ------------------------------ | ---------------- | ------------------------------------------- |
| **Guidance**  | Task execution & focus         | Mobile + Desktop | WIP=1 enforcement, energy-based filtering   |
| **Knowledge** | Information capture & linking  | Desktop + Mobile | Bidirectional linking, auto-discovery       |
| **Tracking**  | Physical inventory management  | Mobile + Desktop | Photo-first capture, location awareness     |

### Cross-Cutting Concepts

| Concept        | Scope        | Purpose                                                    |
| -------------- | ------------ | ---------------------------------------------------------- |
| **Initiative** | System-wide  | Groups related Quests, Notes, and Items into a unified context |
| **Routine**    | Guidance     | Recurring templates that spawn Quest instances on schedule |
| **Tags**       | System-wide  | Shared taxonomy across all three applications              |

### Architectural Principles

- **Privacy-focused**: Self-hosted server keeps all data on user's infrastructure
- **Offline-capable**: Both mobile and desktop work fully offline; sync when connected
- **Mobile-daily, desktop-deep**: Mobile is primary for daily rituals and capture; desktop for reflection and complex work
- **Shared data layer**: Hybrid database (SurrealDB desktop/server, SQLite mobile) with robust sync
- **Plugin-extensible**: Core functionality augmented by sandboxed plugins
- **Cross-platform sync**: Real-time sync via self-hosted server

### Platform Strategy

| Platform | Role | Key Use Cases |
| -------- | ---- | ------------- |
| **Mobile (Android, iOS)** | Daily driver | Morning energy check-in, routine completion, quick capture, Quest completion, Tracking at store |
| **Desktop (Windows, Linux, macOS)** | Deep work & planning | Weekly Harvest, Knowledge writing, bulk operations, complex views |
| **Server (Self-hosted Docker)** | Sync hub & AI services | Multi-device sync, embedding generation, transcription |
| **Web (minimal)** | View-only access | Check status from any browser; no editing required |

### Technology Constraints

- Desktop/Mobile: Kotlin Multiplatform with Compose Multiplatform
- Server: Ktor with kotlinx-rpc, Docker Compose deployment
- Auth: JWT tokens, Argon2 password hashing, multi-user with data isolation
- Storage: S3-compatible API for attachments (local filesystem default, Backblaze B2/AWS S3/MinIO optional)
- AI Services:
  - Embeddings: Server-side default; desktop can run locally via ONNX Runtime (all-MiniLM-L6-v2)
  - Transcription: Server-side via whisper.cpp
  - Completion: Routed to user-configured provider (Ollama, Anthropic, OpenAI, OpenRouter)

---

## 7. ADHD Design Principles

These principles govern all UI/UX decisions across the ecosystem. They are non-negotiable.

### 7.1 Reduce Cognitive Load

- Minimal decisions required at any step
- Clear, single paths to common actions
- Hide complexity until explicitly needed
- No feature should require remembering where it lives

### 7.2 Externalize Structure

- System-enforced limits replace willpower (WIP=1, energy filtering)
- Constraints are features, not restrictions
- The UI is a supportive partner, not a demanding boss

### 7.3 Support Variable Capacity

- Energy check-ins acknowledge daily fluctuation
- Zero-energy days are valid; rest is not failure
- Tasks filter to match current capacity
- No shame for "unproductive" days

### 7.4 Build Forgiveness Mechanisms

- No task is ever truly deleted (archive with undo)
- "Phoenix Rising" recovery for abandoned boards
- Projects can become ongoing areas without judgment
- Language is neutral, never judgmental
- Streaks reward progress, don't punish gaps

### 7.5 Provide Immediate Feedback

- Visual and auditory confirmation of actions
- Progress visible at a glance
- Celebrate completions (without being annoying)

### 7.6 Use Progressive Disclosure

- Show only what's needed now
- Details expand on demand
- Advanced features hidden until relevant
- Onboarding reveals complexity gradually

### 7.7 Combat Time Blindness

- Visual timers with clear remaining time
- Progress bars for elapsed time
- Proactive reminders (user-configurable)
- Weekly Harvest provides natural checkpoint

---

## 8. Visual Design System

Full specifications in `altair-design-system.md`. Summary of core elements:

### Design Philosophy

- **Calm Focus**: Soft, low-stimulation aesthetics that reduce anxiety
- **Focus-first**: Default to single-task views; complexity is opt-in
- **Gentle hierarchy**: Visual weight guides attention without shouting

### Visual Language

| Element              | Specification                                                      |
| -------------------- | ------------------------------------------------------------------ |
| **Palette**          | Low-contrast backgrounds, readable text, accent colors for actions |
| **Typography**       | Large, accessible sizes; open letterforms; generous line height    |
| **Corners**          | Consistent soft rounding (no sharp edges)                          |
| **Depth**            | Subtle shadows; no harsh borders                                   |
| **Focus indicators** | Clear highlighting for active elements                             |

### Component Patterns

- **Quick Capture**: Global activation, instant focus, single-action save
- **Focus Dashboard**: Single active task view with hidden complexity
- **Energy Filter**: Dynamic filtering based on current capacity
- **Initiative Card**: Pinned cross-app context showing related content
- **Standard containers**: Consistent card-based content
- **Action buttons**: Primary (blue), completion (green)
- **Timer visualizations**: Circular or bar-based progress

---

## 9. Initiatives

Initiatives are the cross-cutting organizational layer that connects work across Guidance, Knowledge, and Tracking. They represent "something you're working toward" without imposing heavyweight project management.

### 9.1 Initiative Model

| Property        | Description                                                |
| --------------- | ---------------------------------------------------------- |
| **Name**        | Initiative title (e.g., "Bathroom Renovation")             |
| **Description** | Optional longer description                                |
| **Parent**      | Optional parent Initiative (enables nesting areas → projects) |
| **Ongoing**     | Toggle: unchecked = project (has end), checked = area (maintained indefinitely) |
| **Target Date** | Optional target completion date (ignored if Ongoing)       |
| **Status**      | Active, Paused, Completed, Archived                        |
| **Links**       | Connected Epics, Notes, Items, Tags                        |
| **Created**     | Timestamp                                                  |
| **Updated**     | Timestamp                                                  |

**Nesting Rules:**
- Areas (Ongoing=true) can contain projects or other areas
- Projects (Ongoing=false) can contain sub-projects but not areas
- Maximum nesting depth: 3 levels (Area → Project → Sub-project)
- Example: "Home Improvement" (area) → "Bathroom Renovation" (project) → "Plumbing Phase" (sub-project)

### 9.2 Initiative Behaviors

**Creation:**
- Initiatives are optional—users can use Altair without ever creating one
- Created from any app or from a dedicated Initiatives view
- Can be created retroactively to group existing content

**Linking:**
- Epics (Guidance) can belong to an Initiative
- Notes (Knowledge) can be linked to an Initiative
- Items (Tracking) can be allocated to an Initiative
- Tags can be associated with an Initiative for auto-linking

**Status Transitions:**
- Active → Paused: "Not working on this now, but will return"
- Active → Completed: "This project is done" (only for non-ongoing)
- Active ↔ Ongoing toggle: Convert project to area or vice versa (no shame!)
- Any → Archived: "Don't show this anymore, but keep the data"

### 9.3 Initiative Card

A persistent, contextual card that appears across all three applications when an Initiative is active:

```
┌─────────────────────────────────────────────┐
│ 🏠 Bathroom Renovation                      │
│                                             │
│ Guidance: 3 Quests (1 active)    →          │
│ Knowledge: 12 Notes              →          │
│ Tracking: 47 Items (8 needed)    →          │
│                                             │
│ [Pause]  [Complete]                         │
└─────────────────────────────────────────────┘
```

**Behaviors:**
- Pinned in sidebar or header when Initiative is "focused"
- Shows contextual summary per app
- One-click navigation to filtered view of Initiative content
- Visible on both mobile and desktop
- Users can have multiple active Initiatives but optionally "focus" one

### 9.4 Initiative Dashboard

Dedicated view showing all Initiatives:

**Requirements:**
- FR-I-001: List all Initiatives grouped by status (Active, Paused, Completed, Archived)
- FR-I-002: Create new Initiative with name only (other fields optional)
- FR-I-003: Edit Initiative properties
- FR-I-004: Toggle Ongoing status
- FR-I-005: Archive Initiative (soft delete, recoverable)
- FR-I-006: View Initiative detail with all linked content across apps
- FR-I-007: Set one Initiative as "focused" (optional, affects Initiative Card display)
- FR-I-008: Quick actions to link existing content to Initiative
- FR-I-009: Search and filter Initiatives

---

## 10. Routines

Routines are recurring templates that spawn Quest instances on a schedule. They separate "the definition of what repeats" from "the instance I need to do today."

### 10.1 Routine Model

| Property        | Description                                                |
| --------------- | ---------------------------------------------------------- |
| **Name**        | Routine name (e.g., "Take out trash")                      |
| **Description** | Optional details                                           |
| **Schedule**    | Recurrence pattern (daily, weekly on X, monthly, custom)   |
| **Time of Day** | Optional specific time for reminders/notifications         |
| **Energy**      | Energy cost (1-5) for spawned Quests                       |
| **Initiative**  | Optional link to Initiative                                |
| **Active**      | Whether routine is currently generating instances          |
| **Next Due**    | Calculated next occurrence                                 |

### 10.2 Routine Behaviors

**Instance Generation:**
- System generates Quest instances based on schedule
- Instances appear in "Today" or "This Week" view in Guidance
- Only near-term instances are generated (not 52 weeks of trash day)
- Completed instances roll into Harvested like any Quest

**Schedule Options (v1):**
- Daily
- Weekly on specific day(s)
- Monthly on specific date or relative (e.g., "first Monday")
- Custom interval (every N days)
- No end date (runs until deactivated)

**Schedule Options (v1.1):**
- Weekdays only / weekends only
- Complex patterns (e.g., "every other Tuesday")
- Seasonal schedules

**Time of Day:**
- Optional specific time (e.g., "8:00 AM")
- Used for push notifications and desktop toasts
- If not set, appears in daily list without time-based reminder

**Flexibility:**
- Skip an instance without breaking the routine
- Complete early (instance disappears, next one generates on schedule)
- Snooze to later in day
- Pause routine entirely (no new instances until resumed)

### 10.3 Routine Requirements

- FR-R-001: Create Routine with name and schedule
- FR-R-002: Edit Routine properties
- FR-R-003: Deactivate Routine (stops generating, keeps history)
- FR-R-004: Delete Routine (soft delete, recoverable)
- FR-R-005: View all Routines in dedicated list
- FR-R-006: See upcoming instances across all Routines
- FR-R-007: Skip specific instance
- FR-R-008: Complete instance (marks done, next instance generates per schedule)
- FR-R-009: Link Routine to Initiative (optional)
- FR-R-010: Routine instances appear in Daily Check-in and Weekly Harvest
- FR-R-011: Set time of day for routine reminders
- FR-R-012: Push notification / desktop toast at specified time

---

## 11. Universal Inbox

The Universal Inbox is a system-level capture point that reduces friction by deferring the "what type is this?" decision until triage.

### 11.1 Design Philosophy

**Capture without decisions:**
- At capture time, users shouldn't need to decide if something is a Quest, Note, or Item
- Everything goes to Inbox first
- Classification happens during triage (later, when user has time)

**One inbox, three destinations:**
- Inbox items are untyped until triaged
- Triage converts an inbox item into a Quest, Note, or Item
- Can also link to Initiative during triage

### 11.2 Inbox Model

| Property        | Description                                           |
| --------------- | ----------------------------------------------------- |
| **Content**     | Text, voice recording, image, or link                 |
| **Created**     | Timestamp                                             |
| **Source**      | How captured (keyboard, voice, camera, share, widget) |
| **Attachments** | Optional media (photos, audio clips)                  |

### 11.3 Capture Methods

| Method | Platform | Description |
|--------|----------|-------------|
| **Quick capture field** | All | Text input always accessible |
| **Global keyboard shortcut** | Desktop | System-wide hotkey opens capture |
| **Voice capture** | All | One-tap voice recording with transcription |
| **Camera capture** | Mobile | Photo capture (useful for Items) |
| **Share target** | Mobile | Receive content from other apps |
| **Widget** | Mobile | Home screen quick capture |
| **Watch** | Wearable | Voice capture from wrist |

### 11.4 Triage Actions

When reviewing Inbox, user can:

| Action | Result |
|--------|--------|
| **"This is a Quest"** | Creates Quest in Guidance (goes to Guidance Inbox or Next Up) |
| **"This is a Note"** | Creates Note in Knowledge |
| **"This is an Item"** | Creates Item in Tracking |
| **Link to Initiative** | Associates with an Initiative during conversion |
| **Archive** | Removes from Inbox (recoverable) |
| **Delete** | Soft delete (recoverable 30 days) |

### 11.5 Mobile Home Screen

On mobile, the Universal Inbox is the **default home screen**, providing:

- Quick capture field (prominent)
- Inbox items list (untriaged count)
- Today summary (active Quest + due routines)
- Energy status
- Navigation to Guidance, Knowledge, Tracking

This positions capture as the primary mobile interaction, with triage and execution following naturally.

### 11.6 Desktop Access

On desktop, Inbox is accessible from:
- Global keyboard shortcut (capture)
- Sidebar/navigation in all three apps
- Dedicated Inbox view

### 11.7 Inbox Requirements

- FR-IN-001: Quick capture input always accessible (all platforms)
- FR-IN-002: Global keyboard shortcut for capture (desktop)
- FR-IN-003: Voice capture with transcription
- FR-IN-004: Camera capture (mobile)
- FR-IN-005: Share target integration (mobile)
- FR-IN-006: Inbox widget for home screen (mobile)
- FR-IN-007: Triage action: convert to Quest
- FR-IN-008: Triage action: convert to Note
- FR-IN-009: Triage action: convert to Item
- FR-IN-010: Link to Initiative during triage (optional)
- FR-IN-011: Archive with undo (minimum 30 days)
- FR-IN-012: Bulk triage operations
- FR-IN-013: Inbox count badge visible system-wide
- FR-IN-014: Mobile home screen shows Inbox + Today summary

### 11.8 Widget Strategy

| Widget | Purpose |
|--------|---------|
| **Quick Capture** | One-tap text, long-press voice, swipe for camera |
| **Inbox Count** | Badge showing untriaged items (tap to open) |
| **Today Summary** | Active Quest + due routines (compact) |
| **Initiative Card** | Focused Initiative with cross-app status |

---

## 12. Cross-Application Integration

### 12.1 Shared Data Model

All applications access a unified local database with the following entity relationships:

```
Universal Inbox (System-wide)
└── Triage → Quest | Note | Item

Initiative (System-wide)
├── Epic (Guidance) ←→ Note (Knowledge)
│   └── Quest (Guidance) → Item requirements (Tracking)
├── Note (Knowledge) ←→ Item references (Tracking)
└── Item (Tracking)

Routine (System-wide) → spawns Quest instances
All entities ←→ Tags (shared taxonomy)
```

### 12.2 Auto-Discovery of Relationships

The system automatically surfaces connections between entities:

| Mechanism                | Description                                    |
| ------------------------ | ---------------------------------------------- |
| **Semantic similarity**  | Cosine similarity > 0.7 suggests relationship  |
| **Fuzzy title matching** | "web dev" ≈ "web development"                  |
| **Smart aliasing**       | "RPi" ≈ "Raspberry Pi", "JS" ≈ "JavaScript"    |
| **Entity recognition**   | Extract people, places, technologies from text |
| **Shared keywords**      | Common terms suggest relationship              |
| **Initiative context**   | Content linked to same Initiative is related   |

### 12.3 Integration Points

| From           | To             | Integration                                     |
| -------------- | -------------- | ----------------------------------------------- |
| Knowledge note | Guidance quest | Extract actionable items as quests              |
| Guidance quest | Knowledge note | Link research/documentation to tasks            |
| Knowledge note | Tracking item  | Detect mentioned components/materials           |
| Tracking item  | Guidance quest | Reserve items for quest requirements            |
| Tracking BoM   | Guidance quest | Generate shopping list quests                   |
| Any content    | Initiative     | Group related work under unified context        |
| Initiative     | All apps       | Initiative Card shows cross-app summary         |

### 12.4 Universal Search

Single search interface queries all applications:

- Full-text search across notes, quests, items, initiatives
- Semantic search using vector embeddings
- Filter by application, type, tag, date, initiative
- Results ranked by relevance and recency

### 12.5 Proactive Suggestions

The system surfaces relevant information without being asked:

- Duplicate detection across applications
- Missing material alerts (BoM vs. inventory)
- Related content suggestions within Initiative context
- Routine reminders based on schedule

---

## 13. Plugin Architecture

### 13.1 Design Goals

- Extend functionality without modifying core
- User controls all permissions
- Plugins cannot compromise privacy principles
- Sandboxed execution prevents data leaks

### 13.2 Plugin Types (v1)

v1 supports data and integration plugins. UI plugins (custom views, screens) are planned for v1.1.

| Category               | Examples                                         |
| ---------------------- | ------------------------------------------------ |
| **AI services**        | Ollama, OpenAI, Anthropic, OpenRouter (example plugins) |
| **Cloud sync**         | Google Drive, Dropbox, Nextcloud, WebDAV         |
| **Import sources**     | Notion, Obsidian, Todoist, Trello                |
| **Export formats**     | PDF, Word, specialized formats                   |
| **Automation**         | IFTTT, Zapier, n8n, custom scripts               |
| **Health integration** | Google Fit, Fitbit (read-only, correlation only) |

### 13.3 Example Plugins

AI provider integrations serve as reference implementations demonstrating the plugin API:

- **Ollama Plugin**: Local LLM completion via Ollama API
- **OpenAI Plugin**: GPT completion and embeddings
- **Anthropic Plugin**: Claude completion

These examples show patterns for: API authentication, streaming responses, error handling, and user configuration.

### 13.4 Security Model

**Capability-Based Permissions:**
- Plugins declare required capabilities at registration
- Users approve/deny each capability at install
- Permissions revocable at any time
- Audit log of plugin data access

**Available Capabilities:**
| Capability | Description |
|------------|-------------|
| `read:quests` | Read Quest data |
| `write:quests` | Create/update Quests |
| `read:notes` | Read Note data |
| `write:notes` | Create/update Notes |
| `read:items` | Read Item data |
| `write:items` | Create/update Items |
| `network:<host>` | Contact specified host (e.g., `network:api.openai.com`) |
| `user:settings` | Read/write plugin-specific user settings |

**Sandboxing (v1):**
- Strict API boundary: plugins only access approved interfaces
- No reflection or runtime introspection of core code
- No direct filesystem access (all storage via Altair APIs)
- Network allowlist: plugins declare which hosts they contact
- Sufficient for data/integration plugins

**Sandboxing (v1.1 for UI plugins):**
- Process isolation or WebAssembly sandbox (TBD)
- Required for untrusted UI code execution

### 13.5 Health Integration Philosophy

Health data requires special handling:

- **No built-in health tracking** — Avoids HIPAA/privacy concerns
- **Read-only integration** — Pull data from dedicated health apps
- **Correlation, not diagnosis** — Data used only for pattern recognition (e.g., sleep quality → energy levels)
- **User controls data flow** — Explicit opt-in for each data type

### 13.6 Future: UI Plugins (v1.1)

UI plugins will enable:
- Custom views and screens
- New navigation entries
- Custom visualization components
- Example: Dependency graph view as optional plugin

---

## 14. Authentication & Multi-User

Altair's self-hosted server supports multiple users on a single instance. Each user has complete data isolation—no user can see another user's content. This enables households to share a single server without separate deployments.

### 14.1 User Model

| Property         | Description                                      |
| ---------------- | ------------------------------------------------ |
| **Username**     | Unique identifier for login                      |
| **Email**        | Optional, for password recovery                  |
| **Password**     | Hashed with Argon2                               |
| **Role**         | Admin or Member                                  |
| **Status**       | Active, Disabled, Deleted                        |
| **Storage Used** | Calculated from attachments/photos               |
| **Storage Quota**| Configurable limit (optional)                    |
| **Created**      | Timestamp                                        |
| **Last Login**   | Timestamp                                        |

### 14.2 Data Isolation

All user-generated content is scoped to a single user:

- Every entity (Initiative, Quest, Note, Item, Routine, Epic, etc.) has a `user_id` field
- All queries filter by authenticated user
- No cross-user data access in v1 (future collaboration will use explicit sharing)
- Admin users cannot view member data (only manage accounts)

### 14.3 Admin Model

**First Admin:**
- First user to register becomes admin, OR
- Admin credentials set via environment variable on first boot
- Server can start in "setup mode" requiring admin creation

**Admin Capabilities:**
- Create, disable, delete user accounts
- Reset user passwords
- Configure storage quotas
- View server-wide storage usage (not content)
- Configure AI providers and server settings
- Multiple admins allowed (no single point of control)

**Admin Limitations:**
- Cannot view user content
- Cannot impersonate users
- Cannot export other users' data

### 14.4 Authentication Flow

**Registration:**
- Invite-only by default (admin generates invite codes)
- Open registration can be enabled (not recommended)
- Email verification optional (configurable)

**Login:**
- Username/email + password
- Returns JWT token with user scope
- Token refresh mechanism for long sessions
- Optional: TOTP two-factor authentication (v1.1)
- Optional: OAuth/OIDC identity provider integration (v1.1)

**Device Authentication:**
- Mobile/desktop apps store token in secure storage (Keychain/Keystore)
- Token scoped to user
- Multiple devices per user supported
- Device list viewable in settings
- Revoke device access remotely

### 14.5 User Lifecycle

**Account Deletion:**
1. User or admin initiates deletion
2. Prompt to export all data first
3. Soft delete with configurable retention period (default: 30 days)
4. After retention: permanent purge of all user data
5. Storage reclaimed

**Account Disable:**
- Admin can disable account (normal login blocked)
- Data preserved
- Can be re-enabled
- Disabled users can still log in to export their data (limited session, export only)
- Admin can toggle "Allow disabled users to export" (default: enabled)

### 14.6 Storage Architecture

**Attachments (photos, files):**
- S3-compatible API for storage backend
- Default: local filesystem
- Configurable: Backblaze B2, AWS S3, MinIO, any S3-compatible service
- Per-user storage quotas (optional, admin-configurable)
- Quota enforcement on upload

**Storage Quota Features:**
- FR-A-001: Admin can set global default quota
- FR-A-002: Admin can set per-user quota override
- FR-A-003: User sees storage usage in settings
- FR-A-004: Warning at 80% quota
- FR-A-005: Calculate file size before upload; reject if would exceed quota with clear message showing required vs. available space

### 14.7 Future Collaboration Considerations

While v1 is single-user only, the data model is designed for future sharing:

- All entities have `user_id` (owner)
- Schema supports future `shared_with` field for explicit sharing
- Sync architecture supports eventual consistency (target: < 5 seconds)
- No real-time collaboration planned (Google Docs style)
- Future sharing would be explicit opt-in per Initiative or entity

---

## 15. Cross-Platform Synchronization

### 15.1 Sync Strategy

| Aspect                  | Approach                                         |
| ----------------------- | ------------------------------------------------ |
| **Trigger**             | Real-time when online, queued when offline       |
| **Scope**               | User's own devices only (scoped by auth token)   |
| **Conflict resolution** | Auto-merge when safe; deferred resolution for complex conflicts (see §14.4) |
| **Selective sync**      | User-defined rules per data type                 |
| **Binary handling**     | Efficient delta sync for images/video            |
| **Priority**            | Quests and Routines sync first (daily use)       |
| **Consistency target**  | < 5 seconds for cross-device visibility          |

### 15.2 Multi-Device Daily Use

Sync must support fluid movement between devices throughout the day:

| Scenario | Expectation |
| -------- | ----------- |
| Morning check-in on phone | Energy level syncs to desktop within seconds |
| Complete Quest on phone | Harvested on desktop immediately |
| Add item at store (phone) | Visible on desktop when user returns |
| Write note on desktop | Available on phone for reference |
| Weekly Harvest on desktop | All phone activity already present |

### 15.3 Offline Capabilities

- Full feature parity offline on both mobile and desktop
- Complete local database mirror
- Action queue for pending sync
- Conflict detection and flagging
- Intelligent retry with exponential backoff
- Clear indicator of sync status

### 15.4 Conflict Resolution Strategy

Conflicts are resolved based on data type, minimizing user decision burden:

**Auto-Resolved (No User Action):**
- Timestamps and metadata → most recent wins
- Additive changes (new items, new links) → merge both
- Non-conflicting field changes → merge both

**Simple Conflicts (Quick Resolution):**
- Single field differs (e.g., title, status, energy) → "Pick one" UI with clear labels
- Short text changes → side-by-side comparison, pick one

**Complex Conflicts (Deferred Resolution):**
- Long text (note body) with divergent edits → **keep both versions as conflict snapshots**
- Entity shows "conflict" indicator until resolved
- User views both versions and merges at leisure
- Most recent version shown by default until resolved
- No data loss; user controls final outcome

**Design Principle:** Don't force immediate resolution for complex conflicts. Flag them, preserve all versions, and let the user resolve when they have time.

### 15.5 Cloud Backup Options

Backup is optional and user-controlled (separate from device sync):

| Provider     | Platform           |
| ------------ | ------------------ |
| iCloud       | iOS/macOS          |
| Google Drive | All platforms      |
| Dropbox      | All platforms      |
| OneDrive     | All platforms      |
| Nextcloud    | Self-hosted option |
| WebDAV       | Generic protocol   |

### 15.6 Backup Features

- **Selective backup**: Choose what to back up (full, settings only, specific apps)
- **Encryption**: End-to-end encryption before upload
- **Extensible**: Plugin architecture for additional providers

---

## 16. Mobile Platform Features

Mobile is the **primary platform for daily operations**. This section covers shared mobile infrastructure.

### 16.1 Daily Driver Features

| Feature | Purpose |
| ------- | ------- |
| **Morning Check-in** | Energy assessment, today's Routines, pick first Quest |
| **Quick Capture** | Instant text, voice, photo capture to any app |
| **Quest Completion** | Complete Quests throughout the day |
| **Routine Tracking** | Check off recurring tasks |
| **Tracking at Store** | View shopping lists, mark items purchased |
| **Initiative Context** | See Initiative Card with cross-app status |

### 16.2 System Integration

| Feature                | Capability                                                                                                       |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------- |
| **Push notifications** | Quest reminders, timer completions, energy check-ins, Routine due, low-stock alerts, maintenance due dates |
| **Widgets**            | Active quest, quick capture, energy status, daily routines, today's summary |
| **Shortcuts/Intents**  | Voice commands, automation triggers, quick actions                                                               |
| **Biometric security** | Face ID, Touch ID, fingerprint, app lock                                                                         |

### 16.3 Accessibility

- VoiceOver/TalkBack support
- Dynamic type support
- High contrast mode
- Reduce motion options
- Switch control compatibility

### 16.4 Platform-Specific Implementations

**Android:**

- Wear OS companion app
- Google Assistant integration
- Material You theming
- Work profile support
- Tablet optimization

**iOS:**

- Apple Watch companion app
- Siri Shortcuts integration
- Handoff to Mac desktop

### 16.5 Mobile-Desktop Handoff

- **Continue on desktop**: Deep links to current context
- **Universal clipboard**: Copy on mobile, paste on desktop
- **Shared notifications**: Dismiss on one, cleared everywhere
- **Session handoff**: Resume timer/focus session across devices
- **File handoff**: Start capture on mobile, finish on desktop

### 16.6 Mobile Limitations

**Technical constraints:**

- Local AI models unavailable (use server AI)
- Sandboxed file system access
- Platform restrictions on background processing
- Memory constraints for large graphs

**Intentional simplifications:**

- Complex graph views simplified for touch
- Bulk operations simplified for touch
- Weekly Harvest optimized for desktop (but functional on mobile)

---

## 17. Desktop Platform Features

Desktop is the **primary platform for deep work and planning**.

### 17.1 Shared Features (Also on Mobile)

Desktop includes all mobile daily driver features:
- Energy check-in and tracking
- Quest creation and completion
- Routine tracking
- Quick capture (text, voice, photo via webcam)
- Initiative Card display
- Full Guidance, Knowledge, and Tracking functionality
- Push notifications (via system notifications)

### 17.2 Desktop-Specific Features

| Feature | Purpose |
| ------- | ------- |
| **Weekly Harvest** | Full reflection and planning ritual (optimized for desktop) |
| **Knowledge Writing** | Long-form note creation with graph exploration |
| **Bulk Operations** | Multi-select, batch tag, reorganize |
| **Complex Views** | Full graph view, canvas mode |
| **Initiative Management** | Create and organize Initiatives |
| **Local Embeddings** | Run embedding model locally (optional, reduces server load) |

### 17.3 Desktop Advantages

- Full keyboard navigation and shortcuts
- Large screen for graph visualization
- Multi-window support
- File system integration for exports
- Local embedding generation via ONNX Runtime
- Drag-and-drop file import

---

## 18. Web Dashboard

A lightweight web interface served by the self-hosted server. Provides view-only access to user data plus full admin functionality.

### 18.1 Scope

**User Features:**
- View Initiatives and their status
- View today's Quests and Routines
- View recent Notes (read-only)
- Quick capture (text only)
- Sync status
- Account settings and data export

**Admin Features:**
- User management (create, disable, delete, reset password)
- Invite code generation
- Storage quota configuration
- AI provider configuration
- Server health and status dashboard
- System settings

**Excluded:**
- Full editing capabilities for content
- Graph views
- Complex navigation
- Feature parity with native apps

### 18.2 Purpose

- Check status from any browser
- Quick reference when away from devices
- **Canonical admin interface** (all admin actions happen here)
- Server monitoring and configuration
- Share view with household member (future)

### 18.3 Requirements

**User Requirements:**
- FR-W-001: View Initiative list with status
- FR-W-002: View today's Quests and Routines
- FR-W-003: View recent Notes (read-only)
- FR-W-004: Quick text capture to Inbox
- FR-W-005: Show sync status
- FR-W-006: Account settings and data export

**Admin Requirements:**
- FR-W-007: User management (list, create, disable, delete)
- FR-W-008: Generate invite codes
- FR-W-009: Configure storage quotas (global and per-user)
- FR-W-010: Configure AI providers
- FR-W-011: Server health dashboard (CPU, memory, storage, active connections)
- FR-W-012: System settings management

**Technical Requirements:**
- FR-W-013: Responsive design (fully functional on mobile browsers for admin access)
- FR-W-014: Authentication required for all pages
- FR-W-015: Role-based access (admin features only visible to admins)

---

## 19. Performance Requirements

### 19.1 Mobile Targets (Daily Driver)

| Metric                       | Target                     |
| ---------------------------- | -------------------------- |
| App launch (cold)            | < 3 seconds                |
| App launch (warm)            | < 1 second                 |
| Energy check-in flow         | < 5 seconds total          |
| Quest completion             | < 500ms                    |
| Routine instance check-off   | < 300ms                    |
| Camera capture ready         | < 2 seconds                |
| Sync latency (online)        | < 5 seconds                |
| Search (10k items)           | < 2 seconds                |
| Battery impact (typical use) | < 5% daily                 |
| Base app size                | < 100 MB                   |
| Offline cache                | Configurable 100 MB – 2 GB |

### 19.2 Desktop Targets

| Metric                 | Target         |
| ---------------------- | -------------- |
| App launch             | < 2 seconds    |
| Quest creation         | < 500ms        |
| Semantic search        | < 1 second     |
| Graph render (500 nodes) | < 2 seconds  |
| Relationship discovery | > 90% accuracy |

### 19.3 Sync Targets

| Metric | Target |
| ------ | ------ |
| Quest sync (online) | < 2 seconds |
| Full sync (1000 entities) | < 30 seconds |
| Conflict detection | Immediate |
| Offline queue processing | < 5 seconds after reconnect |

---

## 20. Constraints & Assumptions

### 20.1 Technical Constraints

- Multi-platform native apps (Windows, Linux, macOS, Android, iOS)
- Mobile as daily driver, not just companion
- Privacy-focused self-hosted architecture
- Multi-user server with complete data isolation
- S3-compatible storage backend for attachments
- Single-user data model with future collaboration considerations
- Open-source license: AGPL v3 or later

### 20.2 Assumptions

- Users have basic computer literacy
- Users have minimum 4GB RAM, 1GB storage
- Users have access to both mobile device and desktop/laptop (for optimal experience)
- Users are willing to self-host a sync server (or use a trusted instance)

### 20.3 Explicit Non-Scope (v1)

| Item                        | Rationale                                        |
| --------------------------- | ------------------------------------------------ |
| Full-featured web application | Native apps prioritized; admin dashboard only  |
| Real-time collaboration     | Single-user v1; adds significant complexity      |
| Third-party cloud-only storage | Data stays on user's server                   |
| Enterprise/team features    | Out of scope for v1                              |
| Plugin marketplace          | Architecture ready; marketplace deferred         |
| UI plugins                  | v1 supports data/integration plugins only; UI plugins in v1.1 |
| Dependency graphs / Gantt   | PM tooling; could be added as UI plugin later    |
| Health data collection      | HIPAA concerns; integrate via plugins            |

---

## 21. Feature Priority (System-Level)

### Critical (Must Have)

1. Multi-user authentication with data isolation
2. Initiative system (cross-cutting context)
3. Routine system (recurring task templates)
4. Universal Inbox (system-level capture with triage)
5. Privacy-focused self-hosted architecture
6. Cross-app data integration via Initiatives
7. Robust multi-device sync
8. Offline mode (both mobile and desktop)
9. Push notifications (mobile)
10. Initiative Card (cross-app context display)
11. Web dashboard with admin UI
12. Mobile home screen with Inbox + Today summary

### High Priority

1. S3-compatible storage backend configuration
2. Auto-relationship discovery
3. Semantic search across apps
4. Mobile-desktop handoff
5. Watch/Wear OS companion apps (reduced feature set)
6. Plugin architecture (data/integration API)

### Medium Priority

1. Cloud backup (core providers)
2. Two-factor authentication
3. OAuth/OIDC identity provider support
4. Proactive suggestions
5. UI plugin architecture (v1.1 foundation)

---

## 22. Resolved Design Decisions

These questions were raised during design and have been resolved:

| Question | Resolution | Location |
|----------|------------|----------|
| Initiative hierarchy | Yes, Initiatives can nest (areas → projects → sub-projects), max 3 levels | §9.1 |
| Routine complexity | v1 supports basic patterns; weekdays/complex patterns in v1.1 | §10.2 |
| Sync conflict UI | Deferred resolution: keep both versions, user merges at leisure | §14.4 |
| Plugin sandboxing | Capability-based permissions + API boundary for v1; stronger isolation for UI plugins in v1.1 | §12.4 |
| User data retention | 30 days default for soft-deleted accounts | §13.5 |

## 23. Open Questions

No critical open questions remain for v1. The following are out of scope for this document:

- **AI model selection**: Deployment/configuration concern, not PRD
- **iOS development**: Project governance concern, not PRD
- **Hosted instances**: Out of scope; users self-host or use trusted third-party instances at their own discretion

---

## Appendix A: Document Cross-References

| Document                  | Scope                              |
| ------------------------- | ---------------------------------- |
| `altair-prd-guidance.md`  | Guidance application requirements  |
| `altair-prd-knowledge.md` | Knowledge application requirements |
| `altair-prd-tracking.md`  | Tracking application requirements  |
| `altair-design-system.md` | Calm Focus visual design system    |

---

## Appendix B: Glossary

| Term               | Definition                                                      |
| ------------------ | --------------------------------------------------------------- |
| **Inbox**          | System-level capture point; items are untyped until triaged     |
| **Initiative**     | Cross-cutting organizational unit linking Quests, Notes, Items  |
| **Quest**          | A discrete, completable task in Guidance                        |
| **Epic**           | A group of related Quests within an Initiative                  |
| **Routine**        | A recurring template that spawns Quest instances on schedule    |
| **Triage**         | Process of converting Inbox items into Quests, Notes, or Items  |
| **Harvest**        | Weekly reflection and planning ritual                           |
| **WIP**            | Work In Progress; default limit is 1 active Quest               |
| **Energy**         | Subjective daily capacity rating (1-5 scale)                    |
| **Ongoing**        | Initiative toggle: project (has end) vs. area (maintained indefinitely) |
| **BoM**            | Bill of Materials; list of required items for a project         |
| **Self-hosted**    | Architecture where data stays on user-controlled infrastructure |
| **Phoenix Rising** | Recovery process for abandoned/overwhelming boards              |

---

## Appendix C: Day in the Life

A typical day using Altair:

**7:00 AM (phone):** Wake up, open Altair. Home screen shows Inbox (2 items from yesterday) and Today summary. Energy check-in prompt: tap "3 - Medium."

**7:05 AM (phone):** Triage Inbox: "Research faucet brands" → Quest (link to Bathroom Renovation). "That cool Rust article" → Note. Done.

**7:10 AM (phone):** See today's Routines: "Morning meds ✓" "Review calendar ✓" "One Quest before email." Check them off.

**7:30 AM (phone):** Pick the faucet Quest from Next Up. Complete it.

**9:00 AM (desktop):** Deep work session. Open Knowledge, see the faucet research note linked to the Initiative. Add more notes.

**11:00 AM (desktop):** Thought strikes: "Need to call insurance about that claim." Quick capture via keyboard shortcut → Inbox. Back to work.

**12:30 PM (phone):** At hardware store. Open Tracking, filter by "Bathroom Renovation" Initiative. See items needed. Find faucet, realize caulk is also low. Mark both as purchased.

**3:00 PM (phone):** Waiting room. Voice capture: "Maybe we should paint the bathroom too." → Inbox for later triage.

**6:00 PM (desktop):** Triage remaining Inbox items. Link relevant notes to Initiatives.

**Sunday 7:00 PM (desktop):** Weekly Harvest. Review completed Quests (celebrate!). See "Bathroom Renovation" progress. Plan next week's Quests. Note: "Monday is trash day, Wednesday is landscaping."

**Monday 7:00 AM (phone):** Morning check-in shows Routine: "Take out trash." Check it off. Start the week.
