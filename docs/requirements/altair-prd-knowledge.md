# Altair Product Requirements Document
## Knowledge: Personal Knowledge Management

| Field | Value |
|-------|-------|
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-01-08 |
| **Parent Document** | `altair-prd-core.md` |
| **Dependencies** | Guidance (quest linking), Tracking (item references) |

---

## 1. Purpose

This document defines the product requirements for Knowledge, the personal knowledge management (PKM) application in the Altair ecosystem. Knowledge provides Obsidian-like note-taking with bidirectional linking, enhanced by automatic relationship discovery and cross-app integration.

For system-level architecture, design principles, and cross-app integration, see `altair-prd-core.md`.

---

## 2. Problem Statement

ADHD users face specific challenges with knowledge management:

- **Capture friction**: Ideas arrive fast and disappear faster; any friction in capture means lost thoughts.
- **Organization paralysis**: Deciding where a note belongs requires executive function that's already taxed.
- **Forgotten connections**: Notes created weeks ago become invisible even when highly relevant to current work.
- **Format switching**: Moving between text, voice, video, and image capture requires too many context switches.
- **Rigid hierarchies**: Folder-based systems force premature categorization and hide cross-topic relationships.
- **Disconnected tools**: Notes, tasks, and inventory live in separate apps with no awareness of each other.

Knowledge addresses these through zero-friction capture, automatic relationship discovery, flat structure with emergent organization, and deep integration with Guidance and Tracking.

---

## 3. User Stories

### 3.1 Capture

> **As an ADHD user**, I want to capture thoughts instantly in any format so that I never lose an idea due to friction.

> **As an ADHD user**, I want voice capture with transcription so that I can externalize thoughts while my hands are busy.

> **As an ADHD user**, I want quick video capture so that I can record context-rich notes when text isn't enough.

### 3.2 Organization

> **As an ADHD user**, I want the system to discover connections between my notes so that I don't have to remember to link them manually.

> **As an ADHD user**, I want to see all notes related to my current work so that relevant context surfaces automatically.

> **As an ADHD user**, I want a graph view of my knowledge so that I can explore connections visually.

### 3.3 Integration

> **As an ADHD user**, I want to extract quests from notes so that actionable items don't get buried in documentation.

> **As an ADHD user**, I want to see which notes relate to my current quest so that research is always at hand.

> **As an ADHD user**, I want notes to detect mentioned inventory items so that project materials are linked automatically.

### 3.4 Obsidian Parity

> **As an Obsidian user**, I want bidirectional links and backlinks so that my existing workflows transfer.

> **As an Obsidian user**, I want full Markdown support including tables, code blocks, and LaTeX so that I can write technical documentation.

> **As an Obsidian user**, I want a canvas/whiteboard mode so that I can think spatially.

---

## 4. Core Concepts

### 4.1 Note Model

Notes are the atomic unit of Knowledge:

| Property | Description |
|----------|-------------|
| **Title** | Note name (used for WikiLinks) |
| **Body** | Markdown content |
| **Created** | Timestamp of creation |
| **Modified** | Timestamp of last edit |
| **Tags** | Hierarchical tag list |
| **Aliases** | Alternative names for linking |
| **Attachments** | Linked media (images, audio, video) |
| **Backlinks** | Notes that link to this note |
| **Outlinks** | Notes this note links to |

### 4.2 Relationship Types

Connections between notes (and cross-app entities):

| Type | Discovery Method |
|------|------------------|
| **Explicit link** | User creates [[WikiLink]] |
| **Semantic similarity** | Cosine similarity > 0.7 threshold |
| **Shared tags** | Notes with common tags |
| **Fuzzy match** | Title/alias matching ("JS" ≈ "JavaScript") |
| **Entity extraction** | Shared people, places, technologies |
| **Cross-app** | Note ↔ Quest, Note ↔ Item |

### 4.3 Daily Notes

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

#### Version History

**Requirements:**
- FR-K-017: Automatic version snapshots on save
- FR-K-018: View previous versions
- FR-K-019: Restore previous version
- FR-K-020: Diff view between versions
- FR-K-021: Version retention configurable (default: 30 days)

---

### 5.2 Markdown Editor

Full-featured Markdown editor with Obsidian compatibility.

#### Editing Modes

**Requirements:**
- FR-K-022: Live preview (default) — see rendered output while typing
- FR-K-023: Split view — editor and preview side by side
- FR-K-024: Source mode — raw Markdown only
- FR-K-025: Reading mode — rendered output only

#### Syntax Support

| Feature | Syntax |
|---------|--------|
| Headings | `# H1` through `###### H6` |
| Bold/Italic | `**bold**`, `*italic*` |
| Links | `[text](url)`, `[[WikiLink]]` |
| Images | `![alt](path)` |
| Code | Inline `` `code` ``, fenced blocks |
| Tables | Pipe tables |
| Checkboxes | `- [ ]` and `- [x]` |
| Blockquotes | `> quote` |
| Horizontal rules | `---` |
| Footnotes | `[^1]` |
| Highlights | `==highlighted==` |
| Strikethrough | `~~deleted~~` |

**Requirements:**
- FR-K-026: All standard Markdown syntax supported
- FR-K-027: Tables with alignment
- FR-K-028: Fenced code blocks with syntax highlighting
- FR-K-029: Checkbox tasks (render as interactive checkboxes)
- FR-K-030: LaTeX math inline (`$...$`) and block (`$$...$$`)
- FR-K-031: Mermaid diagrams
- FR-K-032: Callout blocks (note, warning, tip, etc.)

#### Editor Features

**Requirements:**
- FR-K-033: Syntax highlighting
- FR-K-034: Auto-complete for WikiLinks
- FR-K-035: Auto-complete for tags
- FR-K-036: Toolbar for common formatting (mobile)
- FR-K-037: Keyboard shortcuts for formatting (desktop)
- FR-K-038: Find and replace
- FR-K-039: Word/character count
- FR-K-040: Focus mode (hide UI chrome)

---

### 5.3 Linking System

Obsidian-compatible bidirectional linking.

#### WikiLinks

**Requirements:**
- FR-K-041: `[[Note Title]]` creates link to note
- FR-K-042: `[[Note Title|Display Text]]` creates aliased link
- FR-K-043: `[[Note Title#Heading]]` links to specific heading
- FR-K-044: `[[Note Title#^block-id]]` links to specific block
- FR-K-045: Auto-complete while typing `[[`
- FR-K-046: Create note if linked note doesn't exist

#### Backlinks

**Requirements:**
- FR-K-047: Backlink panel shows all notes linking to current note
- FR-K-048: Backlinks show surrounding context (sentence/paragraph)
- FR-K-049: Click backlink to navigate to source
- FR-K-050: Unlinked mentions detection (title appears but not linked)
- FR-K-051: Convert unlinked mention to link with one click

#### Aliases

**Requirements:**
- FR-K-052: Define aliases in note frontmatter
- FR-K-053: Aliases included in link auto-complete
- FR-K-054: Links via alias resolve to canonical note

---

### 5.4 Graph View

Interactive visualization of note relationships.

#### Views

**Requirements:**
- FR-K-055: Global graph — all notes and connections
- FR-K-056: Local graph — current note and immediate connections
- FR-K-057: Filter by tag, date range, link type
- FR-K-058: Search within graph

#### Visualization

**Requirements:**
- FR-K-059: Force-directed layout (default)
- FR-K-060: Nodes sized by connection count
- FR-K-061: Nodes colored by type (note, quest, item) or tag
- FR-K-062: Edge thickness by relationship strength
- FR-K-063: Zoom and pan navigation
- FR-K-064: Click node to open note
- FR-K-065: Hover preview of note content

#### Interaction

**Requirements:**
- FR-K-066: Drag nodes to reposition
- FR-K-067: Pin nodes to fixed positions
- FR-K-068: Manual layout persistence
- FR-K-069: Cluster detection and highlighting
- FR-K-070: Orphan notes highlighted (no connections)

---

### 5.5 Mind Mapping

Dedicated mind map interface beyond graph view.

#### Node Types

| Type | Source | Visual |
|------|--------|--------|
| Note node | Knowledge note | Title preview |
| Quest node | Guidance quest | Quest indicator |
| Item node | Tracking item | Item indicator |
| Topic node | Tag or concept | Tag color |

**Requirements:**
- FR-K-071: Create mind map from any starting note
- FR-K-072: Add child nodes (new notes or existing)
- FR-K-073: Link to Quests and Items from other apps
- FR-K-074: Soft rounded card design with focus glow
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

### 5.7 Multi-Modal Quick Capture

Zero-friction capture in any format.

#### Capture Modes

| Mode | Input | Processing |
|------|-------|------------|
| Text | Quick note field | Auto-save |
| Voice | Microphone | AI transcription |
| Video | Camera | Compressed storage, thumbnail |
| Image | Camera/gallery | OCR extraction (optional) |

#### Text Capture

**Requirements:**
- FR-K-088: Floating action button for quick capture
- FR-K-089: Global keyboard shortcut (desktop)
- FR-K-090: Auto-save as new note or append to daily note
- FR-K-091: Option to specify target note
- FR-K-092: Capture from share sheet (mobile)

#### Voice Capture

**Requirements:**
- FR-K-093: One-tap voice recording
- FR-K-094: AI-powered transcription
- FR-K-095: Audio file attached to note
- FR-K-096: Transcription editable
- FR-K-097: Timestamps in transcription (optional)

#### Video Capture

**Requirements:**
- FR-K-098: Quick video recording (max 2 minutes)
- FR-K-099: Compressed storage for efficiency
- FR-K-100: Auto-generated thumbnail
- FR-K-101: Video embedded in note
- FR-K-102: Optional AI transcription of audio

#### Image Capture

**Requirements:**
- FR-K-103: Camera capture or gallery import
- FR-K-104: Document scanner mode (edge detection, perspective correction)
- FR-K-105: OCR text extraction from images
- FR-K-106: Image embedded in note

#### Universal Capture Features

**Requirements:**
- FR-K-107: Unified capture dialog (mode icons: 📝 🎤 📹 📷)
- FR-K-108: Auto-routing to appropriate note
- FR-K-109: Quick tag assignment during capture
- FR-K-110: Capture queued when offline

---

### 5.8 Search

Comprehensive search across all notes.

#### Search Types

**Requirements:**
- FR-K-111: Full-text search with ranking
- FR-K-112: Semantic search (vector embeddings)
- FR-K-113: Hybrid search (keyword + semantic)
- FR-K-114: Tag search (#tag)
- FR-K-115: Path/title search
- FR-K-116: Cross-app search (include Quests, Items)

#### Search Features

**Requirements:**
- FR-K-117: Instant results as you type
- FR-K-118: Result preview with context highlighting
- FR-K-119: Filter by date range
- FR-K-120: Filter by type (note, daily note, canvas)
- FR-K-121: Filter by tag
- FR-K-122: Sort by relevance, date modified, date created
- FR-K-123: Recent searches saved
- FR-K-124: Search operators (AND, OR, NOT, quotes for exact match)

---

### 5.9 Auto-Discovery of Relationships

Automatic surfacing of connections.

#### Discovery Mechanisms

| Mechanism | Threshold/Rule |
|-----------|---------------|
| Semantic similarity | Cosine similarity > 0.7 |
| Fuzzy title match | Levenshtein distance < 3 |
| Smart aliasing | "web dev" ≈ "web development" |
| Entity recognition | Shared people, places, tech |
| Shared keywords | TF-IDF extracted terms |

**Requirements:**
- FR-K-125: Background discovery process (non-blocking)
- FR-K-126: Discovered relationships shown as "suggested links"
- FR-K-127: One-click to confirm/create explicit link
- FR-K-128: Dismiss false positives (with learning)
- FR-K-129: Discovery confidence score visible
- FR-K-130: Cross-app discovery (Notes ↔ Quests ↔ Items)

---

### 5.10 AI Features

AI-powered assistance for knowledge work.

#### Features

| Feature | Description |
|---------|-------------|
| **Quest extraction** | Identify actionable items in notes |
| **Auto-summarization** | Generate note summaries |
| **Knowledge graph generation** | Suggest tags and links |
| **Smart templates** | Generate note templates from examples |
| **Content suggestions** | Recommend related content to add |
| **Related note recommendations** | Surface relevant notes |

**Requirements:**
- FR-K-131: AI features require explicit invocation (no background automation)
- FR-K-132: Extract Quests: button in note toolbar
- FR-K-133: Summarize: button in note toolbar
- FR-K-134: Suggest links: button in note toolbar
- FR-K-135: AI works with local models (desktop) or cloud (mobile)
- FR-K-136: Graceful degradation when AI unavailable
- FR-K-137: User can accept/reject/edit all AI suggestions

---

## 6. Mobile-Specific Features

### 6.1 Full Feature Parity

All desktop features available on mobile with touch optimization:

- Complete note management with Markdown editor
- Mind mapping with pinch-zoom and touch manipulation
- Graph view with touch navigation
- Canvas mode with finger drawing
- Multi-modal capture (camera, voice, video)
- Full search (keyword and semantic)
- AI features via cloud

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
- FR-K-152: Capture reminder based on time/location (optional)

### 6.5 Widgets

**Requirements:**
- FR-K-153: Quick capture widget (one-tap to daily note)
- FR-K-154: Recent notes widget
- FR-K-155: Daily note widget (shows today's note)

---

## 7. Non-Functional Requirements

### 7.1 Performance

| Metric | Target |
|--------|--------|
| Note creation | < 200ms |
| Note render (Markdown) | < 500ms |
| Search results | < 1 second |
| Graph render (100 nodes) | < 1 second |
| Semantic search | < 1 second |
| Auto-discovery | Background, non-blocking |
| Voice transcription | < 3 seconds for 30-second clip |

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
- FR-K-156: Import Obsidian vault (folder structure, links, attachments)
- FR-K-157: Import Markdown files
- FR-K-158: Export single note as Markdown
- FR-K-159: Export entire vault as folder structure
- FR-K-160: Export with or without attachments
- FR-K-161: Export graph as SVG/PNG

---

## 8. Integration Points

### 8.1 Guidance Integration

| From Knowledge | To Guidance |
|----------------|-------------|
| Note with actionable items | Extract as Quests (AI-assisted) |
| Note with BoM | Generate shopping Quests |
| Research note | Link to related Quest |

| From Guidance | To Knowledge |
|---------------|--------------|
| Quest | Link to research/documentation notes |
| Epic | Link to project overview note |
| Quest completion | Prompt to create reflection note |

### 8.2 Tracking Integration

| From Knowledge | To Tracking |
|----------------|-------------|
| Note mentioning items | Auto-detect and suggest links |
| BoM in note | Link to inventory items |

| From Tracking | To Knowledge |
|---------------|--------------|
| Item | Link to documentation notes |
| Item with manual | Attach manual as note |

### 8.3 Universal Features

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
5. Quick capture (text)
6. Graph view (local + global)
7. Mobile: Touch-optimized editor
8. Mobile: Offline mode

### High Priority

1. Auto-relationship discovery
2. Semantic search
3. Voice capture with transcription
4. Mind mapping
5. Version history
6. Quest ↔ Note linking
7. Mobile: Camera capture
8. Mobile: Share extension

### Medium Priority

1. Canvas/whiteboard mode
2. Video capture
3. AI features (summarization, extraction)
4. Obsidian import
5. Document scanner with OCR
6. Advanced graph filtering

---

## 10. Open Questions

1. **Semantic model**: Which embedding model provides best balance of quality and local performance?
2. **Obsidian compatibility**: Full plugin API compatibility, or core features only?
3. **Conflict resolution**: What's the optimal UX for merge conflicts in notes edited on multiple devices?
4. **AI transcription**: Local model viable on mobile, or cloud-only?

---

## Appendix: Keyboard Shortcuts (Desktop)

| Action | Shortcut |
|--------|----------|
| New note | Cmd/Ctrl + N |
| Quick capture | Cmd/Ctrl + Shift + N |
| Save | Cmd/Ctrl + S |
| Search | Cmd/Ctrl + / |
| Toggle preview | Cmd/Ctrl + E |
| Toggle graph | Cmd/Ctrl + G |
| Insert link | Cmd/Ctrl + K |
| Bold | Cmd/Ctrl + B |
| Italic | Cmd/Ctrl + I |
| Heading | Cmd/Ctrl + 1-6 |
| Checkbox | Cmd/Ctrl + Enter |
| Go to daily note | Cmd/Ctrl + D |
| Back (navigation) | Cmd/Ctrl + [ |
| Forward (navigation) | Cmd/Ctrl + ] |
