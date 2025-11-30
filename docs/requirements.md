# Altair Requirements

**Version**: 1.0  
**Status**: APPROVED
**Created**: 2025-11-29
**Author**: Robert Hamilton

## System Specification

## System Overview

Altair is an open-source ADHD-focused productivity ecosystem consisting of three
interconnected applications available on both desktop and mobile platforms with
shared data synchronization. The system emphasizes local-first architecture, the
"Calm Focus" design system (see altair-design-system.md), and cross-platform data
integration. Desktop applications serve as the primary development platform with
mobile applications providing near-complete feature parity optimized for touch
interfaces and mobile-specific capabilities.

### Core Applications

1. **Guidance** - Quest-Based Agile task management (Desktop & Mobile)
2. **Knowledge** - Personal knowledge management with AI features (Desktop & Mobile)
3. **Tracking** - Inventory and asset management for makers (Desktop & Mobile)

### Target Audience

- Individuals with ADHD
- Makers and DIY enthusiasts
- Researchers and knowledge workers
- Households managing projects and inventory

---

## 1. GUIDANCE - Task Management Application

### 1.1 Quest-Based Agile (QBA) Board

#### Six-Column Kanban System

1. **Idea Greenhouse** - Unrefined ideas and thoughts
2. **Quest Log** - Refined, actionable quests
3. **This Cycle's Quest** - Current cycle focus (max 1)
4. **Next Up** - Priority queue (max 5)
5. **In Progress** - Active work (WIP limit = 1)
6. **Harvested** - Completed quests

#### Core Features {#guidance-1-1-core-features}

- Drag-and-drop between columns
- Strict WIP=1 enforcement with visual warnings
- Quest → Epic relationships
- Energy-based filtering
- Quick capture with global activation
- Bulk operations
- Archive management

### 1.2 Focus/Zen Mode

Focus Mode provides a distraction-free interface for the single quest in the
"In Progress" column of the QBA board.

#### UI Components

- Full-screen focused view on single quest
- Visual timer with time remaining (e.g., "25m remaining")
- Progress bar showing time elapsed
- Large "MARK COMPLETE" button
- "On Deck" preview at bottom (shows Next Up queue)
- Energy status indicator (top right)
- Level indicator (top left)

#### Functionality

- Progressive disclosure of quest steps (subtasks)
- Check off quest steps as completed
- Automatic timer start when entering focus mode
- Pomodoro timer integration (user configurable cycles)
- Auto-advance to next quest option
- Keyboard shortcuts for completion and exit
- Save progress on exit

### 1.3 Energy Management System

A subjective self-assessment system for matching tasks to daily capacity - not medical
or health tracking.

#### Daily Energy Check-ins

- Morning energy assessment (1-5 scale)
- Optional notes about how you're feeling
- Pattern recognition over time
- Energy-based quest filtering
- Spoon theory integration

#### Energy Levels

1. **Tiny** (⚡) - Minimal tasks only
2. **Small** (⚡⚡) - Light work
3. **Medium** (⚡⚡⚡) - Standard tasks
4. **Large** (⚡⚡⚡⚡) - Complex work
5. **Huge** (⚡⚡⚡⚡⚡) - Major projects

### 1.4 Weekly Harvest Ritual

#### Components

- Sunday evening reminder (customizable)
- Review completed quests
- Celebrate achievements
- Archive old quests
- Plan next cycle
- Reflect on patterns
- Adjust energy expectations

### 1.5 Gamification System

#### Elements

- XP for quest completion
- Levels with perks
- Achievement badges
- Streak tracking
- Weekly/monthly challenges
- Personal progress dashboard
- Customizable rewards

### 1.6 Quest Dependency Graphs

#### Visualization Features

- DAG (Directed Acyclic Graph) visualization
- Relationship types:
  - Blocks/blocked by
  - Related to
  - Parent/child
  - Follows/precedes
- Critical path highlighting
- Progress visualization (completed/in-progress/blocked)

#### Layout Options

- Top-to-bottom tree
- Left-to-right timeline
- Force-directed network
- Gantt-style view

---

## 2. KNOWLEDGE - Personal Knowledge Management

### 2.1 Note Management

#### Core Features {#knowledge-2-1-core-features}

- Daily notes as default entry point
- Markdown editor with live preview
- Bidirectional linking [[note]]
- Tag system with hierarchy
- Note templates
- Version history
- Full-text search

### 2.2 Mind Mapping

#### Node Types

- Note nodes (with title preview)
- Quest nodes (from Guidance)
- Item nodes (from Tracking)
- Topic/tag nodes

#### Features {#knowledge-2-2-features}

- Interactive node-based visualization
- Drag-and-drop node positioning
- Zoom/pan navigation
- Soft rounded card design with focus glow
- Color coding by type
- Expandable node details
- Cluster detection
- Force-directed layout
- Manual layout persistence

### 2.3 Obsidian Feature Parity

#### Markdown Editor

- Live preview
- Split view (edit/preview)
- Syntax highlighting
- Tables, checkboxes, code blocks
- LaTeX math support
- Mermaid diagrams

#### Linking System

- [[WikiLinks]] support
- Backlink panel
- Unlinked mentions detection
- Link autocomplete
- Alias support [[note|Display Name]]

#### Graph View

- Interactive relationship graph
- Filter by tags, date, type
- Local graph (current note connections)
- Global graph (all connections)
- Canvas/whiteboard mode

### 2.4 Multi-Modal Quick Capture

#### Capture Modes

- **Text**: Quick note field with auto-save
- **Voice**: AI-powered transcription
- **Video**: Record and store
  - Max 2 minute clips
  - Compressed storage
  - Thumbnail generation

#### UI Design

- Floating action button
- Mode icons: 📝 (text), 🎤 (voice), 📹 (video)
- Unified capture dialog
- Auto-routing to appropriate app

### 2.5 Semantic Search & AI Features

#### Search Capabilities

- Vector embeddings for semantic similarity
- Hybrid search (keyword + semantic)
- Cross-app search
- Search filters and facets

#### AI Features

- Quest extraction from notes
- Auto-summarization
- Knowledge graph generation
- Smart templates
- Content suggestions
- Related note recommendations

### 2.6 Auto-Discovery of Relationships

#### Discovery Mechanisms

- Semantic similarity (cosine similarity > 0.7)
- Fuzzy title matching
- Smart aliasing:
  - "web dev" ≈ "web development"
  - "RPi" ≈ "Raspberry Pi"
  - "JS" ≈ "JavaScript"
- Entity recognition
- Shared keyword extraction

#### Relationship Types

- Note ↔ Note
- Quest ↔ Quest
- Note ↔ Quest
- All ↔ Items

---

## 3. TRACKING - Inventory Management

### 3.1 Item Management

#### Core Features

- Item CRUD operations
- Location tracking
- Quantity management
- Photo attachments
- QR/barcode generation
- Custom fields
- Categories and tags

### 3.2 Inventory Intelligence

#### Auto-Discovery

- Real-time text analysis
- Pattern matching for item mentions
- Context-aware suggestions
- Non-intrusive popup showing e.g.,:

  ```text
  ----------
  | Found: |
  ----------
  | 2xPi 1 |
  | 1xPi 5 |
  ----------
  ```

#### Reservation System

- Item status: Available/Reserved/In-use
- Reserved for: [Quest/Note reference]
- Release on task completion
- Conflict resolution

### 3.3 Bill of Materials (BoM)

#### Parsing

- Structured BoM detection in notes
- Recipe/project material extraction
- Quantity parsing
- Unit conversion

#### Features {#tracking-3-3-features}

- Match to existing inventory
- Generate shopping lists
- Track material usage
- Cost estimation
- Supplier links

### 3.4 Maintenance Tracking

#### Features {#tracking-3-4-features}

- Warranty tracking
- Maintenance schedules
- Service history
- Documentation storage
- Reminder notifications
- Cost tracking

---

## 4. Cross-App Integration

### 4.1 Shared Data Access

#### Integration Points

- Quest ↔ Note linking
- Quest → Inventory requirements
- Note → Quest extraction
- Inventory → Project materials
- Universal search across apps
- Shared tag taxonomy

### 4.2 BoM Intelligence Pipeline

#### Basic Integration

- Parse structured BoMs from notes
- Match items to inventory
- Generate shopping lists
- Link materials to quests

#### Advanced Features

- Recipe → quest breakdown
- Automatic quest creation
- Material requirement aggregation
- Project cost estimation

### 4.3 Proactive Suggestions

#### Features {#cross-app-4-3-features}

- Context-aware recommendations
- Duplicate detection
- Missing material alerts
- Related content suggestions
- Quest timing optimization

### 4.4 Plugin Architecture

#### Core Plugin System

- **Plugin API** - Defined interfaces for extensions
- **Sandboxed execution** - Plugins run in isolated context
- **Permission system** - Users control what plugins can access
- **Plugin types**:
  - Cloud sync providers
  - Import/export formats
  - AI model integrations
  - Custom capture methods
  - Workflow automations

#### Initial Plugin Categories

- **Cloud Storage** - Additional backup providers
- **AI Services** - Alternative AI providers
- **Import Sources** - Notion, Obsidian, Todoist, etc.
- **Export Formats** - PDF, Word, specialized formats
- **Automation** - IFTTT, Zapier, n8n, custom scripts
- **Health & Wellness** - Google Fit, Apple Health, Fitbit, sleep trackers
  - Import sleep quality data for energy correlation
  - Activity levels for quest planning
  - Optional medication reminders (handled by health app, not Altair)

#### Health Integration Philosophy

- **No built-in health tracking** - Avoids HIPAA/privacy concerns
- **Optional plugin connections** - Users choose to connect health apps
- **Read-only integration** - Pull data from dedicated health apps
- **User controls data flow** - Explicit permissions for each data type
- **Correlation, not diagnosis** - Data used only for pattern recognition

---

## 5. MOBILE APPLICATIONS

### 5.1 Overview

Mobile applications provide near-complete feature parity with desktop versions,
optimized for touch interfaces and mobile-specific capabilities. While desktop
remains the primary development platform, mobile apps are fully functional
standalone applications, not mere companions.

### 5.2 GUIDANCE Mobile

#### Full Features {#guidance-5-2-features}

- **Complete QBA Board** - All six columns with touch-optimized drag-and-drop
- **Focus/Zen Mode** - Full-screen focus with timer and progress tracking
- **Quest Management** - Complete CRUD operations for quests
- **Energy System** - Daily check-ins, energy-based filtering
- **Timers & Notifications**:
  - Pomodoro timer with background operation
  - Push notifications for timer completion
  - Daily energy check-in reminders
  - Weekly harvest reminders
  - Quest deadline alerts
- **Gamification** - XP tracking, achievements, streaks
- **Quick Capture** - Global shortcut, widget access, share target

#### Mobile Optimizations {#guidance-5-2-mobile}

- **Swipe gestures** - Move between columns, complete quests
- **Haptic feedback** - Task completion, timer milestones
- **Widget support** - Active quest, quick capture, energy status
- **Background sync** - Continuous desktop synchronization
- **Offline mode** - Full functionality without connection

### 5.3 KNOWLEDGE Mobile

#### Full Features {#knowledge-5-3-features}

- **Note Management** - Create, edit, delete with full markdown support
- **Mind Mapping** - Touch-optimized node manipulation with pinch-zoom
- **Obsidian Features**:
  - WikiLinks with touch-friendly autocomplete
  - Backlinks panel
  - Graph view with touch navigation
  - Canvas mode with finger drawing
- **Multi-Modal Capture**:
  - Camera integration for photo notes
  - Voice recording with transcription
  - Video capture (2-minute limit)
  - Drawing/sketching support
- **Search** - Full semantic and keyword search
- **AI Features** - Cloud-based AI (OpenAI, Claude, etc.) when available

#### Mobile Optimizations {#knowledge-5-3-mobile}

- **Markdown toolbar** - Quick formatting buttons above keyboard
- **Image handling** - Direct camera capture, photo library access
- **Voice input** - System dictation integration
- **Share extension** - Capture from any app
- **Document scanner** - Built-in OCR for physical documents

### 5.4 TRACKING Mobile

#### Full Features {#tracking-5-4-features}

- **Complete Inventory Management** - Full CRUD with photo-first interface
- **Barcode/QR Scanning** - Native camera integration
- **Location Services** - GPS tagging, indoor location mapping
- **BoM Processing** - View and check off materials
- **Maintenance Tracking** - Schedule and log maintenance
- **Shopping Lists** - Interactive lists with store mapping

#### Mobile-Specific Advantages

- **Batch photo mode** - Rapid multi-item capture
- **AR preview** - Visualize items in space (future)
- **NFC support** - Tag reading/writing
- **Bluetooth** - Connect to smart labels
- **Location-based reminders** - "When I get home" triggers

### 5.5 Cross-Platform Synchronization

#### Sync Strategy

- **Real-time sync** when online between user's own devices
- **Conflict resolution** - Three-way merge with conflict UI
- **Selective sync** - User-defined sync rules
- **Binary handling** - Efficient image/video sync
- **Incremental updates** - Delta synchronization

#### Cloud Backup Options (User Choice)

- **Optional cloud backup** - Users choose whether to enable
- **Multiple providers** - Support for various services:
  - iCloud (iOS/macOS)
  - Google Drive
  - Dropbox
  - OneDrive
  - Nextcloud (self-hosted option)
  - WebDAV (generic protocol)
- **Selective backup** - Users choose what to back up:
  - Full data backup
  - Settings and preferences only
  - Specific apps/categories
  - Exclude sensitive data
- **Encryption option** - End-to-end encryption before cloud upload
- **Plugin architecture** - Extensible system for adding new providers

#### Offline Capabilities

- **Full offline mode** - All features work without connection
- **Local database** - Complete data mirror
- **Queue management** - Actions queued for sync
- **Conflict detection** - Flag conflicts for review
- **Auto-retry** - Intelligent retry with backoff

### 5.6 Mobile-Specific Features

#### System Integration

- **Push Notifications**:
  - Quest reminders
  - Timer completions
  - Energy check-in prompts
  - Harvest reminders
  - Inventory low-stock alerts
  - Maintenance due dates
- **Widgets** (iOS/Android):
  - Active quest display
  - Quick capture button
  - Energy status
  - Daily stats
  - Next up preview
- **Shortcuts/Intents**:
  - Voice commands ("Add quest...")
  - Automation triggers
  - Quick actions
- **Biometric Security**:
  - Face/Touch ID (iOS)
  - Fingerprint (Android)
  - App lock for sensitive data

#### Accessibility

- **VoiceOver/TalkBack** support
- **Dynamic type** support
- **High contrast mode**
- **Reduce motion** options
- **Switch control** compatibility

### 5.7 Platform-Specific Implementations

#### iOS

- **Apple Watch** companion app:
  - Quick capture
  - Active quest view
  - Timer control
  - Energy check-in
- **Siri Shortcuts** integration
- **Cloud backup** (user choice - iCloud, Google Drive, Dropbox, etc.)
- **Handoff** to Mac desktop app

#### Android

- **Wear OS** companion app
- **Google Assistant** integration
- **Material You** theming
- **Work profile** support
- **Tablet optimization**
- **Cloud backup** (user choice - Google Drive, Dropbox, Nextcloud, etc.)

### 5.8 Mobile-Desktop Handoff

#### Seamless Transitions

- **Continue on desktop** - Deep links to current context
- **Universal clipboard** - Copy on mobile, paste on desktop
- **Shared notifications** - Dismiss on one, cleared everywhere
- **Session handoff** - Resume timer/focus session
- **File handoff** - Start on mobile, finish on desktop

### 5.9 Performance Targets (Mobile)

- **App launch** - Cold: <3s, Warm: <1s
- **Camera capture** - Photo ready: <2s
- **Sync latency** - Changes visible: <5s
- **Search results** - <2s for 10k items
- **Graph render** - 100 nodes: <1s
- **Battery impact** - <5% daily with typical use
- **Storage** - Base app: <100MB
- **Offline cache** - Configurable 100MB-2GB

### 5.10 Limitations vs Desktop

#### Technical Constraints

- **Local AI models** - Use cloud alternatives (OpenAI, Claude)
- **File system access** - Sandboxed storage only
- **Background processing** - Platform restrictions apply
- **Large datasets** - Memory constraints for huge graphs
- **Plugin system** - Limited or unavailable

#### Intentional Simplifications

- **Bulk operations** - Simplified for touch interface
- **Advanced settings** - Subset of desktop options
- **Export formats** - Common formats only
- **Keyboard shortcuts** - Limited to external keyboards

---

## 6. ADHD Design Principles

### Core Principles

1. **Reduce cognitive load** - Minimal decisions, clear paths
2. **External structure** - System-enforced limits, not willpower
3. **Variable capacity** - Adapt to daily energy levels
4. **Forgiveness mechanisms** - No shame for incomplete tasks
5. **Immediate feedback** - Visual and auditory confirmations
6. **Progressive disclosure** - Show only what's needed
7. **Time blindness support** - Timers, reminders, visual progress

### Implementation Guidelines

- WIP=1 enforcement prevents task switching
- Visual timers combat time blindness
- Energy system respects variable capacity
- Harvest ritual provides reflection without judgment
- Gamification rewards progress, not perfection
- Auto-discovery reduces memory burden
- Focus mode eliminates distractions

---

## 7. Visual Design System

### "Calm Focus" Design Language

Full specifications available in `altair-design-system.md`. Core elements:

#### Design Philosophy

- **Externalize executive function** - UI as supportive partner, not demanding boss
- **Reduce cognitive load** - Frictionless paths to action
- **Focus-first** - Default to single-task view, hide complexity

#### Visual Language

- **Soft palette** - Low contrast backgrounds with readable text
- **Generous typography** - Large, accessible font sizes with open letterforms
- **Rounded corners** - Consistent soft edges for reduced visual harshness
- **Subtle depth** - Soft shadows without sharp borders
- **Focus indicators** - Clear visual highlighting for active elements

#### Key Patterns

- **Quick Capture** - Global activation, instant focus, single-action save
- **Focus Dashboard** - Single active task view with hidden complexity
- **Energy Filter** - Dynamic filtering based on current capacity
- **Progressive Disclosure** - Hidden secondary information revealed on demand

#### Component Patterns

- **Standard containers** - Consistent card-based content containers
- **Action buttons** - Primary actions (blue), completion actions (green)
- **Timer visualizations** - Visual progress representations for time
- **Capture interfaces** - Zero-friction input modals
- **Focus indicators** - Visual highlighting for active elements
- **Progressive disclosure elements** - Collapsible sections and expandable details

---

## 8. Success Criteria

### Performance Requirements

#### Desktop Performance

- App launch: <2s
- Quest creation: <500ms
- Semantic search: <1s
- Focus mode task completion: >80%
- Mind map node render: <100ms
- Relationship discovery: >90% accuracy
- Inventory detection: <500ms

#### Mobile Performance (See Section 5.9 for detailed targets)

- App launch: <3s cold, <1s warm
- Camera capture: <2s
- Sync latency: <5s when online
- Battery impact: <5% daily typical use

### User Experience Goals

- Quest completion time: -30% with focus mode
- Relationship discovery: 5+ auto-links per note
- Graph navigation: <2 clicks to any node
- Capture speed: <3 seconds to start
- Daily active use by week 4
- Zero critical bugs at launch

### Feature Priorities

#### Critical (Must Have)

1. QBA board with WIP=1 (Desktop & Mobile)
2. Mind mapping (Desktop & Mobile)
3. Auto-relationship discovery
4. Focus/Zen mode with timers (Desktop & Mobile)
5. Basic note management (Desktop & Mobile)
6. Item tracking with photo capture (Desktop & Mobile)
7. Push notifications (Mobile)
8. Offline mode with sync (Mobile)

#### High Priority

1. Energy management (Desktop & Mobile)
2. Multi-modal capture (Desktop & Mobile)
3. Quest dependency graphs (Desktop & Mobile)
4. Auto-inventory discovery
5. Obsidian feature parity
6. Semantic search (Desktop & Mobile)
7. BoM intelligence
8. Widgets & system integration (Mobile)
9. Watch/Wear OS apps (Mobile)

#### Medium Priority

1. Gamification
2. Weekly Harvest
3. Video capture
4. Canvas/whiteboard
5. Advanced graph filters
6. Maintenance tracking
7. Plugin architecture (extensibility)

---

## 9. Constraints and Assumptions

### Constraints

- Desktop-first development (Linux, Windows, macOS)
- Mobile near-feature-parity (iOS, Android)
- Local-first architecture (works fully offline, cloud backup optional)
- Single-user focus (no real-time collaboration in v1)
- Open-source (AGPL 3.0 license)

### Assumptions

- Users have basic computer literacy
- Users have at least 4GB RAM and 1GB storage available
- Users prefer privacy and local data control
- Users are willing to learn new productivity patterns
- ADHD users benefit from externalized executive function

### Out of Scope for v1

- Web application (future consideration)
- Real-time multi-user collaboration
- Cloud-only features (all features must work offline)
- Enterprise/team features
- Plugin marketplace (though plugin architecture is included)
- Advanced local AI models on mobile (use cloud AI instead)
- Built-in support for every cloud provider (core providers only, rest via plugins)
- Health data collection (HIPAA concerns - integrate with dedicated health apps
  via plugins instead)
