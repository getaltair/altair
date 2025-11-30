# Altair Glossary

> **Ubiquitous language** — Consistent terminology across all documentation and code

---

## Core Concepts

| Term         | Definition                                    | NOT                                   |
| ------------ | --------------------------------------------- | ------------------------------------- |
| **Quest**    | Task with energy cost, optional time estimate | "Task", "Todo", "Ticket"              |
| **Campaign** | A container grouping related quests           | "Project", "Epic", "Sprint"           |
| **Note**     | A markdown document in the knowledge base     | "Document", "Page", "File"            |
| **Item**     | A physical object tracked in inventory        | "Product", "Asset", "Thing"           |
| **Capture**  | Raw input awaiting classification             | "Inbox item", "Draft", "Quick note"   |
| **Location** | Place where items are stored (hierarchical)   | "Place", "Container", "Area"          |
| **Folder**   | Optional organization container for notes     | "Directory", "Category", "Collection" |

---

## Quest-Based Agile (QBA)

| Term               | Definition                                                     |
| ------------------ | -------------------------------------------------------------- |
| **Energy Cost**    | Cognitive/physical effort: `low`, `medium`, `high`, `variable` |
| **Daily Planning** | Selecting quests based on current energy level                 |
| **Backlog**        | Quests not yet committed to                                    |
| **Active**         | Quests committed to for current period                         |

---

## Relationships

| Term               | Definition                  | Direction                       |
| ------------------ | --------------------------- | ------------------------------- |
| **contains**       | Parent-child ownership      | Campaign → Quest, Folder → Note |
| **references**     | Quest links to related note | Quest → Note                    |
| **links_to**       | Wiki-style note connection  | Note ↔ Note (bidirectional)     |
| **requires**       | Quest needs physical item   | Quest → Item                    |
| **documents**      | Note describes an item      | Note → Item                     |
| **stored_in**      | Item's physical location    | Item → Location                 |
| **has_attachment** | Media file linked to entity | Any → Attachment                |

---

## Entity States

| Term          | Applies To      | Meaning                   |
| ------------- | --------------- | ------------------------- |
| **active**    | Campaign, Quest | Currently in use          |
| **backlog**   | Quest           | Not yet started           |
| **completed** | Campaign, Quest | Finished successfully     |
| **archived**  | All             | Soft-deleted, recoverable |
| **pending**   | Capture         | Awaiting classification   |
| **processed** | Capture         | Routed to destination     |
| **discarded** | Capture         | User chose not to keep    |

---

## Apps

| Term              | Definition                                           |
| ----------------- | ---------------------------------------------------- |
| **Guidance**      | Quest and campaign management app (QBA methodology)  |
| **Knowledge**     | Personal knowledge management app (wiki-style notes) |
| **Tracking**      | Inventory management app (items and locations)       |
| **Quick Capture** | Zero-friction input capture (routes to other apps)   |

---

## Technical Terms

| Term              | Definition                                                           |
| ----------------- | -------------------------------------------------------------------- |
| **Embedding**     | Vector representation of note content for semantic search            |
| **Wiki-link**     | `[[Note Title]]` syntax creating bidirectional link                  |
| **Change feed**   | SurrealDB feature tracking record changes for sync                   |
| **Presigned URL** | Time-limited S3 URL for direct upload                                |
| **LWW**           | Last-Write-Wins conflict resolution                                  |
| **Hybrid search** | Combined keyword (BM25) and semantic (vector) search                 |
| **Tauri Command** | IPC function exposed by Rust backend, called from frontend           |
| **tauri-specta**  | Library generating TypeScript types from Rust command signatures     |
| **IPC**           | Inter-Process Communication (how frontend talks to backend in Tauri) |

---

## UI Terms

| Term          | Definition                                                |
| ------------- | --------------------------------------------------------- |
| **Omnibar**   | Global search input accessible from anywhere              |
| **Badge**     | Notification count indicator (e.g., "3 pending captures") |
| **Quick Add** | Minimal form for fast entity creation                     |
| **Backlinks** | Panel showing notes that link to current note             |

---

## Naming Conventions

### Code

| Context              | Convention | Example                          |
| -------------------- | ---------- | -------------------------------- |
| Tables               | snake_case | `quest`, `campaign`, `wiki_link` |
| Fields               | snake_case | `energy_cost`, `created_at`      |
| Rust structs         | PascalCase | `Quest`, `Campaign`              |
| Rust functions       | snake_case | `create_quest()`, `sync_notes()` |
| TypeScript types     | PascalCase | `Quest`, `Campaign`              |
| TypeScript functions | camelCase  | `createQuest()`, `syncNotes()`   |

### Files

| Type       | Convention              | Example                    |
| ---------- | ----------------------- | -------------------------- |
| Migrations | `NNN_description.surql` | `001_initial_schema.surql` |
| Components | PascalCase              | `QuestCard.svelte`         |
| Utilities  | camelCase               | `formatDate.ts`            |

---

## Anti-Patterns (Terms to Avoid)

| ❌ Don't Use         | ✅ Use Instead    | Reason                                       |
| -------------------- | ----------------- | -------------------------------------------- |
| Task                 | Quest             | Quest implies energy cost, adventure framing |
| Project              | Campaign          | Campaign implies scope, contained quests     |
| Todo                 | Quest             | Quest has richer metadata                    |
| Document             | Note              | Note is the specific PKM entity              |
| Inbox                | Capture (pending) | Inbox conflates location and state           |
| Category (for notes) | Folder or Tag     | Category implies single assignment           |
| Delete               | Archive           | Soft delete by default                       |
