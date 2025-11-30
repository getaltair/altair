# Altair Glossary

**Version**: 2.0  
**Status**: APPROVED  
**Created**: 2025-11-29  
**Author**: Robert Hamilton

> **Ubiquitous language** — Consistent terminology across all documentation and code

---

## Core Concepts

| Term | Definition | NOT |
|------|------------|-----|
| **Quest** | A task with energy cost and optional time estimate | "Task", "Todo", "Ticket" |
| **Campaign** | A container grouping related quests | "Project", "Epic", "Sprint" |
| **Note** | A markdown document in the knowledge base | "Document", "Page", "File" |
| **Daily Note** | Auto-created note for each day, default entry point | "Journal", "Log" |
| **Item** | A physical object tracked in inventory | "Product", "Asset", "Thing" |
| **Capture** | Raw input awaiting classification | "Inbox item", "Draft", "Quick note" |
| **Location** | A place where items are stored (hierarchical) | "Place", "Container", "Area" |
| **Folder** | Optional organization container for notes | "Directory", "Category", "Collection" |
| **Reservation** | Item quantity allocated to a specific quest | "Allocation", "Hold" |

---

## QBA Board Columns

| Column | Purpose | Limit |
|--------|---------|-------|
| **Idea Greenhouse** | Unrefined ideas and thoughts | Unlimited |
| **Quest Log** | Refined, actionable quests | Unlimited |
| **This Cycle** | Current cycle/sprint focus | Max 1 |
| **Next Up** | Priority queue for upcoming work | Max 5 |
| **In Progress** | Active work (WIP=1 enforced) | **Strictly 1** |
| **Harvested** | Completed quests | Unlimited |

---

## Quest-Based Agile (QBA)

| Term | Definition |
|------|------------|
| **Energy Cost** | Cognitive/physical effort: `tiny`, `small`, `medium`, `large`, `huge` |
| **Energy Check-in** | Daily self-assessment of current energy level |
| **Daily Planning** | Selecting quests based on current energy level |
| **Focus Mode** | Distraction-free interface for single In Progress quest |
| **Focus Session** | Timed work session on a quest (Pomodoro-style) |
| **Weekly Harvest** | End-of-week ritual: review, celebrate, archive, plan |
| **WIP=1** | Work-In-Progress limit of one quest at a time |

### Energy Levels

| Level | Icon | Description |
|-------|------|-------------|
| **Tiny** | ⚡ | Minimal cognitive load |
| **Small** | ⚡⚡ | Light work |
| **Medium** | ⚡⚡⚡ | Standard tasks |
| **Large** | ⚡⚡⚡⚡ | Complex work |
| **Huge** | ⚡⚡⚡⚡⚡ | Major projects |

---

## Relationships

| Term | Definition | Direction |
|------|------------|-----------|
| **contains** | Parent-child ownership | Campaign → Quest, Folder → Note |
| **references** | Quest links to related note | Quest → Note |
| **links_to** | Wiki-style note connection | Note ↔ Note (bidirectional) |
| **requires** | Quest needs physical item | Quest → Item |
| **documents** | Note describes an item | Note → Item |
| **stored_in** | Item's physical location | Item → Location |
| **reserved_for** | Item reserved for quest | Reservation → Quest |
| **blocks** | Quest dependency | Quest → Quest |
| **has_attachment** | Media file linked to entity | Any → Attachment |
| **tagged** | Tag applied to entity | Any → Tag |

---

## Gamification

| Term | Definition |
|------|------------|
| **XP** | Experience points earned from completing actions |
| **Level** | Player progression tier based on total XP |
| **Achievement** | Unlockable badge for specific accomplishments |
| **Streak** | Consecutive days of specific activity |
| **Grace Period** | Buffer time before streak breaks (default 24h) |

### XP Awards

| Action | XP |
|--------|-----|
| Complete Tiny quest | 10 |
| Complete Small quest | 25 |
| Complete Medium quest | 50 |
| Complete Large quest | 100 |
| Complete Huge quest | 200 |
| Daily energy check-in | 5 |
| Weekly harvest | 50 |
| Complete focus session | 15 |

---

## Knowledge Features

| Term | Definition |
|------|------------|
| **Wiki-link** | `[[Note Title]]` syntax creating bidirectional link |
| **Backlinks** | Panel showing notes that link to current note |
| **Unlinked Mentions** | References to note title without explicit link |
| **Mind Map** | Visual graph of note relationships |
| **Graph View** | Interactive visualization of connections |
| **Local Graph** | Connections for single note |
| **Global Graph** | All notes and relationships |
| **Semantic Search** | Vector-based similarity search |
| **Hybrid Search** | Combined keyword (BM25) and semantic search |
| **Auto-Discovery** | Automatic suggestion of related content |

---

## Tracking Features

| Term | Definition |
|------|------------|
| **Reservation** | Item quantity allocated to a quest |
| **BoM** | Bill of Materials — items needed for a project |
| **BoM Intelligence** | Auto-detection of item mentions in text |
| **Maintenance Schedule** | Recurring maintenance task for an item |
| **Item Status** | `available`, `reserved`, `in_use`, `depleted`, `archived` |

---

## Entity States

### Quest States (Columns)

| Column | Meaning |
|--------|---------|
| `idea_greenhouse` | Unrefined idea |
| `quest_log` | Refined, actionable |
| `this_cycle` | Current cycle focus |
| `next_up` | Priority queue |
| `in_progress` | Active work |
| `harvested` | Completed |
| `archived` | Soft-deleted |

### Other Entity States

| Term | Applies To | Meaning |
|------|------------|---------|
| **active** | Campaign, Folder | Currently in use |
| **completed** | Campaign | Finished successfully |
| **archived** | All entities | Soft-deleted, recoverable |
| **pending** | Capture | Awaiting classification |
| **processed** | Capture | Routed to destination |
| **discarded** | Capture | User chose not to keep |
| **available** | Item | Ready for use |
| **reserved** | Item | Allocated to quest |
| **in_use** | Item | Currently being used |
| **depleted** | Item | Quantity = 0 |

---

## Apps

| Term | Definition |
|------|------------|
| **Guidance** | Quest and campaign management app (QBA methodology) |
| **Knowledge** | Personal knowledge management app (wiki-style notes) |
| **Tracking** | Inventory management app (items and locations) |
| **Quick Capture** | Zero-friction input capture (routes to other apps) |

---

## Technical Terms

| Term | Definition |
|------|------------|
| **Embedding** | Vector representation of note content for semantic search |
| **Wiki-link** | `[[Note Title]]` syntax creating bidirectional link |
| **Change feed** | SurrealDB feature tracking record changes for sync |
| **Presigned URL** | Time-limited S3 URL for direct upload |
| **LWW** | Last-Write-Wins conflict resolution |
| **Hybrid search** | Combined keyword (BM25) and semantic (vector) search |
| **Tauri Command** | IPC function exposed by Rust backend, called from frontend |
| **tauri-specta** | Library generating TypeScript types from Rust command signatures |
| **IPC** | Inter-Process Communication (how frontend talks to backend in Tauri) |

---

## UI Terms

| Term | Definition |
|------|------------|
| **QBA Board** | 6-column Kanban board for quest management |
| **Focus Mode** | Full-screen distraction-free view for In Progress quest |
| **Zen Mode** | Alternative name for Focus Mode |
| **Visual Timer** | Progress bar showing elapsed/remaining time |
| **On Deck** | Preview of Next Up queue in Focus Mode |
| **Omnibar** | Global search input accessible from anywhere |
| **Badge** | Notification count indicator (e.g., "3 pending captures") |
| **Quick Add** | Minimal form for fast entity creation |
| **Backlinks Panel** | UI showing notes that link to current note |
| **Graph View** | Interactive visualization of note relationships |
| **Energy Filter** | UI control to filter quests by energy level |
| **Harvest View** | Weekly summary and reflection interface |

---

## Design System: Calm Focus

| Term | Definition |
|------|------------|
| **Calm Focus** | Altair's ADHD-optimized design language |
| **WIP=1** | Core principle: one task in progress at a time |
| **Progressive Disclosure** | Show only what's needed, hide complexity |
| **Frictionless Capture** | Zero-decision input capture |
| **Tangible Time** | Visual timers instead of just numbers |
| **Forgiving Aesthetics** | Soft, low-contrast, non-anxiety-inducing visuals |

---

## Naming Conventions

### Code

| Context | Convention | Example |
|---------|------------|---------|
| Tables | snake_case | `quest`, `campaign`, `wiki_link` |
| Fields | snake_case | `energy_cost`, `created_at` |
| Rust structs | PascalCase | `Quest`, `Campaign` |
| Rust functions | snake_case | `create_quest()`, `sync_notes()` |
| TypeScript types | PascalCase | `Quest`, `Campaign` |
| TypeScript functions | camelCase | `createQuest()`, `syncNotes()` |

### Files

| Type | Convention | Example |
|------|------------|---------|
| Migrations | `NNN_description.surql` | `001_initial_schema.surql` |
| Components | PascalCase | `QuestCard.svelte` |
| Utilities | camelCase | `formatDate.ts` |

---

## Anti-Patterns (Terms to Avoid)

| ❌ Don't Use | ✅ Use Instead | Reason |
|--------------|----------------|--------|
| Task | Quest | Quest implies energy cost, adventure framing |
| Project | Campaign | Campaign implies scope, contained quests |
| Todo | Quest | Quest has richer metadata |
| Sprint | Cycle | Cycle is less corporate, more personal |
| Document | Note | Note is the specific PKM entity |
| Inbox | Capture (pending) | Inbox conflates location and state |
| Category (for notes) | Folder or Tag | Category implies single assignment |
| Delete | Archive | Soft delete by default |
| Low/Medium/High energy | Tiny/Small/Medium/Large/Huge | 5-level scale, more nuanced |
| Backlog (for quests) | Quest Log or Idea Greenhouse | Columns have specific meanings |
| Timer | Focus Session | Focus Session includes more context |
| Points | XP | XP is the gamification currency |
