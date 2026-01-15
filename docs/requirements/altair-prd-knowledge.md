# Altair Product Requirements Document

## Knowledge: Personal Knowledge Management

| Field               | Value                                                |
| ------------------- | ---------------------------------------------------- |
| **Version**         | 2.0                                                  |
| **Status**          | Draft                                                |
| **Last Updated**    | 2026-01-14                                           |
| **Parent Document** | `altair-prd-core.md`                                 |
| **Integrates With** | Guidance (quest linking), Tracking (item references) |

---

## 1. Purpose

This document defines the product requirements for Knowledge, the personal knowledge management (PKM) application in the Altair ecosystem. Knowledge provides Obsidian-like note-taking with bidirectional linking, enhanced by automatic relationship discovery and cross-app integration.

For system-level architecture, design principles, Initiatives, and Universal Inbox, see `altair-prd-core.md`.

---

## 2. Problem Statement

Knowledge management tools create specific challenges:

- **Capture friction**: Ideas arrive fast and disappear faster; any friction in capture means lost thoughts.
- **Organization paralysis**: Deciding where a note belongs requires executive function that may already be taxed.
- **Forgotten connections**: Notes created weeks ago become invisible even when highly relevant to current work.
- **Format switching**: Moving between text, voice, video, and image capture requires too many context switches.
- **Rigid hierarchies**: Folder-based systems force premature categorization and hide cross-topic relationships.
- **Disconnected tools**: Notes, tasks, and inventory live in separate apps with no awareness of each other.

Knowledge addresses these through:
- **Zero-friction capture**: Multiple capture modes via Universal Inbox or direct note creation
- **Automatic relationship discovery**: Semantic similarity, backlinks, and entity recognition
- **Flat structure with emergent organization**: Tags and links over rigid folders
- **Deep integration**: Notes connect to Quests, Items, and Initiatives

---

## 3. User Stories

### 3.1 Capture

> **As a user**, I want to capture thoughts instantly in any format so that I never lose an idea due to friction.

> **As a user**, I want voice capture with transcription so that I can externalize thoughts while my hands are busy.

> **As a user**, I want quick video capture so that I can record context-rich notes when text isn't enough.

### 3.2 Organization

> **As a user**, I want the system to discover connections between my notes so that I don't have to remember to link them manually.

> **As a user**, I want to see all notes related to my current work so that relevant context surfaces automatically.

> **As a user**, I want a graph view of my knowledge so that I can explore connections visually.

### 3.3 Integration

> **As a user**, I want to extract quests from notes so that actionable items don't get buried in documentation.

> **As a user**, I want to see which notes relate to my current quest so that research is always at hand.

> **As a user**, I want notes to detect mentioned inventory items so that project materials are linked automatically.

### 3.4 Obsidian Parity

> **As an Obsidian user**, I want bidirectional links and backlinks so that my existing workflows transfer.

> **As an Obsidian user**, I want full Markdown support including tables, code blocks, and LaTeX so that I can write technical documentation.

> **As an Obsidian user**, I want a canvas/whiteboard mode so that I can think spatially.

---

## 4. Core Concepts

### 4.1 Note Model

Notes are the atomic unit of Knowledge:

| Property        | Description                         |
| --------------- | ----------------------------------- |
| **Title**       | Note name (used for WikiLinks)      |
| **Body**        | Markdown content                    |
| **Created**     | Timestamp of creation               |
| **Modified**    | Timestamp of last edit              |
| **Tags**        | Hierarchical tag list               |
| **Aliases**     | Alternative names for linking       |
| **Attachments** | Linked media (images, audio, video) |
| **Backlinks**   | Notes that link to this note        |
| **Outlinks**    | Notes this note links to            |

### 4.2 Relationship Types

Connections between notes (and cross-app entities):

| Type                    | Discovery Method                           |
| ----------------------- | ------------------------------------------ |
| **Explicit link**       | User creates [[WikiLink]]                  |
| **Semantic similarity** | Cosine similarity > 0.7 threshold          |
| **Shared tags**         | Notes with common tags                     |
| **Fuzzy match**         | Title/alias matching ("JS" ≈ "JavaScript") |
| **Entity extraction**   | Shared people, places, technologies        |
| **Cross-app**           | Note ↔ Quest, Note ↔ Item                  |
| **Initiative context**  | Notes linked to same Initiative            |

### 4.3 Relationship to Initiatives

Notes can belong to an **Initiative** (defined in core PRD):

- Notes can link to one or more Initiatives
- Initiative Card in Knowledge shows related notes
- Filtering by Initiative shows all related notes

### 4.4 Relationship to Universal Inbox

**Universal Inbox** (defined in core PRD) can feed Notes:

- Inbox items are untyped until triaged
- Triage action "This is a Note" creates Note in Knowledge
- Knowledge also supports direct note creation (bypassing Inbox)

### 4.5 Daily Notes

Default entry point for capture:

- One note per day, auto-created
- Quick access from home screen
- Backlinks show all notes created/modified that day
- Optional daily template
- Serves as working memory / scratchpad

---

## 5. Functional Requirements

### 5.1 Note Management

#### Core CRUD

**Requirements:**

- FR-K-001: Create note with title (body optional initially)
- FR-K-002: Edit note with live Markdown preview
- FR-K-003: Delete note (soft delete, recoverable 30+ days)
- FR-K-004: Duplicate note
- FR-K-005: Merge notes (combine content, redirect links)
- FR-K-006: Split note (extract section to new note)

#### Daily Notes

**Requirements:**

- FR-K-007: Auto-create daily note on app open (configurable)
- FR-K-008: Quick access to today's daily note
- FR-K-009: Navigate between daily notes (previous/next)
- FR-K-010: Daily note template support
- FR-K-011: Daily notes show all notes created/modified that day

#### Organization

**Requirements:**

- FR-K-012: Tag notes with hierarchical tags (e.g., #project/altair)
- FR-K-013: Starred/favorited notes
- FR-K-014: Recent notes list
- FR-K-015: Note templates (user-defined)
- FR-K-016: Bulk tag/untag operations
- FR-K-017: Link note to Initiative (optional)

#### Version History

**Requirements:**

- FR-K-018: Automatic version snapshots on save
- FR-K-019: View previous versions
- FR-K-020: Restore previous version
- FR-K-021: Diff view between versions
- FR-K-022: Version retention configurable (default: 30 days)

---

### 5.2 Markdown Editor

Full-featured Markdown editor with Obsidian compatibility.

#### Editing Modes

**Requirements:**

- FR-K-023: Live preview (default) — see rendered output while typing
- FR-K-024: Split view — editor and preview side by side
- FR-K-025: Source mode — raw Markdown only
- FR-K-026: Reading mode — rendered output only

#### Syntax Support

**Base specification:** CommonMark with GitHub Flavored Markdown (GFM) extensions.

**Obsidian-specific extensions:**
- `[[WikiLinks]]` for internal linking
- `==highlighted==` for highlights
- Callout blocks (`> [!note]`, `> [!warning]`, etc.)

**Requirements:**

- FR-K-027: Full CommonMark + GFM syntax support
- FR-K-028: Fenced code blocks with syntax highlighting
- FR-K-029: Checkbox tasks (render as interactive checkboxes)
- FR-K-030: LaTeX math inline (`$...$`) and block (`$$...$$`)
- FR-K-031: Mermaid diagrams
- FR-K-032: Callout blocks (Obsidian-compatible)
- FR-K-033: WikiLinks (Obsidian-compatible)

#### Editor Features

**Requirements:**

- FR-K-034: Syntax highlighting
- FR-K-035: Auto-complete for WikiLinks
- FR-K-036: Auto-complete for tags
- FR-K-037: Toolbar for common formatting (mobile)
- FR-K-038: Keyboard shortcuts for formatting (desktop)
- FR-K-039: Find and replace
- FR-K-040: Word/character count
- FR-K-041: Focus mode (hide UI chrome)

---

### 5.3 Linking System

Obsidian-compatible bidirectional linking.

#### WikiLinks

**Requirements:**

- FR-K-042: `[[Note Title]]` creates link to note
- FR-K-043: `[[Note Title|Display Text]]` creates aliased link
- FR-K-044: `[[Note Title#Heading]]` links to specific heading
- FR-K-045: `[[Note Title#^block-id]]` links to specific block
- FR-K-046: Auto-complete while typing `[[`
- FR-K-047: Create note if linked note doesn't exist

#### Backlinks

**Requirements:**

- FR-K-048: Backlink panel shows all notes linking to current note
- FR-K-049: Backlinks show surrounding context (sentence/paragraph)
- FR-K-050: Click backlink to navigate to source
- FR-K-051: Unlinked mentions detection (title appears but not linked)
- FR-K-052: Convert unlinked mention to link with one click

#### Aliases

**Requirements:**

- FR-K-053: Define aliases in note frontmatter
- FR-K-054: Aliases included in link auto-complete
- FR-K-055: Links via alias resolve to canonical note

---

### 5.4 Graph View

Interactive visualization of note relationships.

#### Views

**Requirements:**

- FR-K-056: Global graph — all notes and connections
- FR-K-057: Local graph — current note and immediate connections
- FR-K-058: Filter by tag, date range, link type, Initiative
- FR-K-059: Search within graph

#### Visualization

**Requirements:**

- FR-K-060: Force-directed layout (default)
- FR-K-061: Nodes sized by connection count
- FR-K-062: Nodes colored by type (note, quest, item) or tag
- FR-K-063: Edge thickness by relationship strength
- FR-K-064: Zoom and pan navigation
- FR-K-065: Click node to open note
- FR-K-066: Hover preview of note content

#### Interaction

**Requirements:**

- FR-K-067: Drag nodes to reposition
- FR-K-068: Pin nodes to fixed positions
- FR-K-069: Manual layout persistence
- FR-K-070: Cluster detection and highlighting
- FR-K-071: Orphan notes highlighted (no connections)

---

### 5.5 Mind Mapping

Dedicated mind map interface beyond graph view.

#### Node Types

| Type       | Source         | Visual          |
| ---------- | -------------- | --------------- |
| Note node  | Knowledge note | Title preview   |
| Quest node | Guidance quest | Quest indicator |
| Item node  | Tracking item  | Item indicator  |
| Topic node | Tag or concept | Tag color       |

**Requirements:**

- FR-K-072: Create mind map from any starting note
- FR-K-073: Add child nodes (new notes or existing)
- FR-K-074: Link to Quests and Items from other apps
- FR-K-075: Color coding by node type
- FR-K-076: Expandable node details (show note excerpt)
- FR-K-077: Collapse/expand branches
- FR-K-078: Export mind map as image

---

### 5.6 Canvas/Whiteboard Mode

Freeform spatial canvas for visual thinking.

**Requirements:**

- FR-K-079: Infinite canvas with zoom/pan
- FR-K-080: Add notes as cards on canvas
- FR-K-081: Add images to canvas
- FR-K-082: Draw freeform lines/shapes
- FR-K-083: Add text labels
- FR-K-084: Group elements
- FR-K-085: Connect elements with arrows
- FR-K-086: Multiple canvases per vault
- FR-K-087: Canvas saved as note with special format

---

### 5.7 Multi-Modal Capture

Zero-friction capture in any format. Users can capture directly to Knowledge (creating Notes) or use Universal Inbox for later triage.

#### Capture Modes

| Mode  | Input            | Processing                    |
| ----- | ---------------- | ----------------------------- |
| Text  | Quick note field | Auto-save                     |
| Voice | Microphone       | AI transcription              |
| Video | Camera           | Compressed storage, thumbnail |
| Image | Camera/gallery   | OCR extraction (optional)     |

#### Text Capture

**Requirements:**

- FR-K-088: Quick capture accessible from Knowledge
- FR-K-089: Option to create new note or append to daily note
- FR-K-090: Option to specify target note
- FR-K-091: Capture from share sheet (mobile)

#### Voice Capture

**Requirements:**

- FR-K-092: One-tap voice recording
- FR-K-093: AI-powered transcription
- FR-K-094: Audio file attached to note
- FR-K-095: Transcription editable
- FR-K-096: Timestamps in transcription (optional)

#### Video Capture

**Requirements:**

- FR-K-097: Quick video recording (max 2 minutes)
- FR-K-098: Compressed storage for efficiency
- FR-K-099: Auto-generated thumbnail
- FR-K-100: Video embedded in note
- FR-K-101: Optional AI transcription of audio

#### Image Capture

**Requirements:**

- FR-K-102: Camera capture or gallery import
- FR-K-103: Document scanner mode (edge detection, perspective correction)
- FR-K-104: OCR text extraction from images
- FR-K-105: Image embedded in note

#### Capture Integration

**Requirements:**

- FR-K-106: Quick tag assignment during capture
- FR-K-107: Capture queued when offline
- FR-K-108: Link to Initiative during capture (optional)

---

### 5.8 Search

Comprehensive search across all notes.

#### Search Types

**Requirements:**

- FR-K-109: Full-text search with ranking
- FR-K-110: Semantic search (vector embeddings)
- FR-K-111: Hybrid search (keyword + semantic)
- FR-K-112: Tag search (#tag)
- FR-K-113: Path/title search
- FR-K-114: Cross-app search (include Quests, Items)
- FR-K-115: Filter by Initiative

#### Search Features

**Requirements:**

- FR-K-116: Instant results as you type
- FR-K-117: Result preview with context highlighting
- FR-K-118: Filter by date range
- FR-K-119: Filter by type (note, daily note, canvas)
- FR-K-120: Filter by tag
- FR-K-121: Sort by relevance, date modified, date created
- FR-K-122: Recent searches saved
- FR-K-123: Search operators (AND, OR, NOT, quotes for exact match)

---

### 5.9 Auto-Discovery of Relationships

Automatic surfacing of connections.

#### Discovery Mechanisms

| Mechanism           | Threshold/Rule                |
| ------------------- | ----------------------------- |
| Semantic similarity | Cosine similarity > 0.7       |
| Fuzzy title match   | Levenshtein distance < 3      |
| Smart aliasing      | "web dev" ≈ "web development" |
| Entity recognition  | Shared people, places, tech   |
| Shared keywords     | TF-IDF extracted terms        |
| Initiative context  | Notes in same Initiative      |

**Requirements:**

- FR-K-124: Background discovery process (non-blocking)
- FR-K-125: Discovered relationships shown as "suggested links"
- FR-K-126: One-click to confirm/create explicit link
- FR-K-127: Dismiss false positives (with learning)
- FR-K-128: Discovery confidence score visible
- FR-K-129: Cross-app discovery (Notes ↔ Quests ↔ Items)

---

### 5.10 AI Features

AI-powered assistance for knowledge work.

#### Features

| Feature                          | Description                           |
| -------------------------------- | ------------------------------------- |
| **Quest extraction**             | Identify actionable items in notes    |
| **Auto-summarization**           | Generate note summaries               |
| **Knowledge graph generation**   | Suggest tags and links                |
| **Smart templates**              | Generate note templates from examples |
| **Content suggestions**          | Recommend related content to add      |
| **Related note recommendations** | Surface relevant notes                |

**Requirements:**

- FR-K-130: Default: AI features require explicit invocation
- FR-K-131: Setting to enable proactive AI suggestions (default: off)
- FR-K-132: Extract Quests: button in note toolbar
- FR-K-133: Summarize: button in note toolbar
- FR-K-134: Suggest links: button in note toolbar
- FR-K-135: AI works with server AI or configured provider
- FR-K-136: Graceful degradation when AI unavailable
- FR-K-137: User can accept/reject/edit all AI suggestions

---

## 6. Mobile Features

Mobile is a **daily driver** for Knowledge capture, not just a companion. All core features are available with touch optimization.

### 6.1 Full Feature Parity

All features available on mobile:

- Complete note management with Markdown editor
- Mind mapping with pinch-zoom and touch manipulation
- Graph view with touch navigation
- Canvas mode with finger drawing
- Multi-modal capture (camera, voice, video)
- Full search (keyword and semantic)
- AI features via server

### 6.2 Touch Optimizations

**Requirements:**

- FR-K-138: Markdown toolbar above keyboard for quick formatting
- FR-K-139: Touch-friendly WikiLink auto-complete
- FR-K-140: Swipe gestures for navigation (back, forward)
- FR-K-141: Pinch-zoom on graph and canvas
- FR-K-142: Long-press for context menus
- FR-K-143: Pull-to-refresh for sync

### 6.3 Mobile Capture Advantages

**Requirements:**

- FR-K-144: Direct camera capture integrated in editor
- FR-K-145: Photo library access
- FR-K-146: System dictation integration
- FR-K-147: Share extension (capture from any app)
- FR-K-148: Document scanner with OCR
- FR-K-149: Drawing/sketching input

### 6.4 Notifications

**Requirements:**

- FR-K-150: Optional daily note reminder
- FR-K-151: Suggested link notifications (configurable frequency)

### 6.5 Widgets

**Requirements:**

- FR-K-152: Recent notes widget
- FR-K-153: Daily note widget (shows today's note)

---

## 7. Non-Functional Requirements

### 7.1 Performance

| Metric                   | Target                         |
| ------------------------ | ------------------------------ |
| Note creation            | < 200ms                        |
| Note render (Markdown)   | < 500ms                        |
| Search results           | < 1 second                     |
| Graph render (100 nodes) | < 1 second                     |
| Semantic search          | < 1 second                     |
| Auto-discovery           | Background, non-blocking       |
| Voice transcription      | < 3 seconds for 30-second clip |

### 7.2 Data Integrity

- Note content autosaved every 5 seconds during editing
- Version history retained minimum 30 days
- Attachments never orphaned (tracked references)
- Export always includes all linked attachments
- Sync conflicts surfaced to user (never silent data loss)

### 7.3 Accessibility

- Full keyboard navigation (desktop)
- Screen reader support for all views
- Alt text for embedded images
- High contrast mode
- Dynamic type support (mobile)
- Reduce motion option for graph animations

### 7.4 Import/Export

**Requirements:**

- FR-K-154: Import Obsidian vault (folder structure, links, attachments)
- FR-K-155: Import Markdown files
- FR-K-156: Export single note as Markdown
- FR-K-157: Export entire vault as folder structure
- FR-K-158: Export with or without attachments
- FR-K-159: Export graph as SVG/PNG

---

## 8. Integration Points

### 8.1 Universal Inbox Integration

| From Universal Inbox | To Knowledge                     |
| -------------------- | -------------------------------- |
| Triage "This is a Note" | Creates Note in Knowledge     |
| Link to Initiative   | Note inherits Initiative context |

### 8.2 Initiative Integration

| From Knowledge | To Initiative                    |
| -------------- | -------------------------------- |
| Note           | Can belong to one or more Initiatives |
| Progress       | Note count shown in Initiative Card |

| From Initiative | To Knowledge                     |
| --------------- | -------------------------------- |
| Initiative Card | Shows related note count         |
| Focus setting   | Filters notes to Initiative      |

### 8.3 Guidance Integration

| From Knowledge | To Guidance                        |
| -------------- | ---------------------------------- |
| Note           | Extract actionable items as Quests |
| BoM in note    | Generate shopping Quests           |

| From Guidance    | To Knowledge                     |
| ---------------- | -------------------------------- |
| Quest            | Link to research notes           |
| Epic             | Link to project documentation    |
| Quest completion | Option to create reflection note |
| Evening wrap-up  | Save reflection to Knowledge     |

### 8.4 Tracking Integration

| From Knowledge        | To Tracking               |
| --------------------- | ------------------------- |
| Note mentioning items | Auto-detect and link      |
| BoM in note           | Parse and match inventory |

| From Tracking    | To Knowledge                |
| ---------------- | --------------------------- |
| Item             | Link to documentation notes |
| Item with manual | Manual as attached note     |

### 8.5 Universal Features

- Universal search includes notes
- Shared tag taxonomy
- Notes visible in cross-app graph

---

## 9. Feature Priority

### Critical (Must Have)

1. Note CRUD with Markdown editor
2. WikiLinks with backlinks
3. Daily notes
4. Full-text search
5. Note ↔ Quest linking (core Altair differentiator)
6. Graph view (local + global)
7. Mobile: Touch-optimized editor
8. Mobile: Offline mode

### High Priority

1. Auto-relationship discovery
2. Semantic search
3. Voice capture with transcription
4. Mind mapping
5. Version history
6. Note ↔ Initiative linking
7. Vim editing mode (keybindings)
8. Mobile: Camera capture
9. Mobile: Share extension

### Medium Priority

1. Canvas/whiteboard mode
2. Video capture
3. AI features (summarization, extraction)
4. Obsidian import
5. Document scanner with OCR
6. Advanced graph filtering
7. Proactive AI suggestions (opt-in)

---

## 10. Resolved Design Decisions

| Question | Resolution |
|----------|------------|
| Semantic model | Server-side embeddings via ONNX Runtime (all-MiniLM-L6-v2); desktop can run locally |
| Obsidian compatibility | Core features (WikiLinks, backlinks, graph); plugin API not in v1 |
| Conflict resolution | Deferred resolution strategy (see core PRD §15.4) |
| AI transcription | Server-side via whisper.cpp; mobile sends audio to server |
| Capture routing | Direct note creation OR via Universal Inbox triage |
| Vim mode | High priority; may slip to post-v1 if implementation complexity warrants |

---

## Appendix: Keyboard-Accessible Actions

The following actions should be accessible via configurable keyboard shortcuts on desktop:

| Action                | Description                        |
| --------------------- | ---------------------------------- |
| New note              | Create new note                    |
| Save                  | Save current note                  |
| Search                | Open search                        |
| Toggle preview        | Switch between edit/preview modes  |
| Toggle graph          | Show/hide graph view               |
| Insert link           | Insert WikiLink                    |
| Bold                  | Apply bold formatting              |
| Italic                | Apply italic formatting            |
| Heading 1-6           | Apply heading level                |
| Checkbox              | Insert/toggle checkbox             |
| Go to daily note      | Open today's daily note            |
| Back                  | Navigate to previous note          |
| Forward               | Navigate to next note              |
| Toggle focus mode     | Hide/show UI chrome                |
| Link to Initiative    | Add Initiative link to note        |
