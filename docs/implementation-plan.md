# Altair Implementation Plan

## Overview

This document contains feature specifications for Altair. Each feature is described in 4-6 sentences
suitable for Spec-Driven Development (SDD). Implement features in order—later phases depend on
earlier ones.

Reference the `docs/` folder for detailed requirements, domain model, and architecture decisions.

---

## Phase 1: Project Foundation

### 1.1 Kotlin Multiplatform Project Scaffold

Create a new Kotlin Multiplatform project with Compose Multiplatform using the JetBrains wizard.
Configure targets for desktop (JVM), Android, and iOS. Set up Gradle with version catalogs for
dependency management. Add core dependencies: Koin 4.x (DI), Decompose 3.x (navigation), Arrow 2.x
(error handling), kotlinx-serialization, kotlinx-datetime. Create a basic "hello world" screen that
renders on all three platforms. Verify by running `./gradlew :composeApp:run` (desktop),
`./gradlew :composeApp:installDebug` (Android), and building via Xcode (iOS).

### 1.2 Altair Design System Foundation

Create the Altair design system module with design tokens: colors (dark-first palette), typography
(Inter font family), spacing scale, and border radii. Implement `AltairTheme` composable that
provides tokens via CompositionLocal. Add Compose Unstyled dependency and create initial styled
components: `AltairButton`, `AltairTextField`, `AltairCard`. Verify by rendering components in a
preview screen showing all variants.

### 1.3 Desktop SurrealDB Integration

Add surrealdb.java dependency for desktop target. Initialize SurrealDB embedded on app startup using
SurrealKV storage engine, storing data in the platform-appropriate app data directory. Create a
simple repository that writes and reads a test record to verify the connection works. The database
namespace should be "altair" and the database name "main". Verify by creating an entity and
confirming it persists across app restart.

### 1.4 Mobile SQLite Integration

Add SQLDelight dependency for mobile targets. Define initial schema matching desktop entities
(quest, note, item tables with basic fields). Generate type-safe query classes. Create a simple
repository that writes and reads a test record. Verify by running on Android emulator and iOS
simulator, confirming data persistence.

### 1.5 Application Shell and Navigation

Create the root Decompose component (`RootComponent`) with `StackNavigation` for main navigation.
Implement child components for each module: `GuidanceComponent`, `KnowledgeComponent`,
`TrackingComponent`. Create the main application layout with bottom navigation (mobile) and sidebar
navigation (desktop) showing three modules. Set up Koin modules for DI and inject dependencies into
components. Verify by clicking each nav item and confirming the content area updates appropriately
per platform, with proper back handling on all platforms.

---

## Phase 2: Server Foundation

### 2.1 Ktor Server Scaffold

Create a Ktor server module with basic HTTP endpoints and kotlinx-rpc configuration. Set up Docker
build for the server image. Configure SurrealDB client connection for server-side database access.
Create a health check endpoint at `/health`. Verify by running `docker compose up` and hitting the
health endpoint.

### 2.2 kotlinx-rpc Service Definitions

Define RPC service interfaces in a shared module: `SyncService`, `AiService`, `AuthService`.
Implement stub implementations on the server. Configure RPC client in the client app module. Verify
by making a test RPC call from desktop app to local server and receiving a response.

### 2.3 Authentication Service

Implement `AuthService` with login, token refresh, and logout operations. Use JWT tokens with
configurable expiration. Store user credentials hashed with Argon2 in SurrealDB. Create client-side
token storage using platform-secure storage (Keychain/Keystore). Verify by logging in, making
authenticated requests, and logging out.

### 2.4 Database Schema and Migrations

Implement a migration system that tracks applied migrations in a `_migration` table and runs pending
migrations on startup. Create initial migrations for core tables: `epic`, `quest`, `checkpoint`,
`energy_budget`, `note`, `note_link`, `folder`, `tag`, `item`, `custom_field`, `location`,
`container`. All entities use ULID string IDs and include `created_at`, `updated_at`, and optional
`deleted_at` fields. Run migrations on both desktop SurrealDB and server SurrealDB.

### 2.5 Sync Engine Foundation

Implement basic sync protocol: pull (client requests changes since version N), push (client sends
local changes). Track `sync_version` per entity. Implement optimistic locking with version
conflicts. Create `SyncService` RPC implementation. Verify by creating an entity on desktop, syncing
to server, and seeing it appear after pull on mobile.

---

## Phase 3: Core Infrastructure

### 3.1 Event Bus Implementation (Desktop)

Create an in-process event bus using Kotlin coroutines `SharedFlow` that modules can publish to and
subscribe from. Define event types for each module category: `GuidanceEvent`, `KnowledgeEvent`,
`TrackingEvent`, `SystemEvent`. Subscribers filter by event type. Include bounded buffer for
backpressure. Verify by publishing a test event and confirming a subscriber receives it.

### 3.2 Error Handling Framework

Define domain error types using sealed interfaces per module: `QuestError`, `NoteError`,
`ItemError`, `SyncError`. Use Arrow's `Either<Error, T>` for all repository and use case methods.
Implement `Raise` DSL for composing operations. Add Arrow Optics with `@optics` annotation for
nested state updates in ViewModels. Create UI error mapping to convert domain errors to
user-friendly messages. Verify by triggering validation errors and confirming appropriate
snackbar/toast display with actionable messages.

### 3.3 Server AI Service: Embeddings

Implement embedding generation on server using ort (Kotlin ONNX Runtime) with bundled
all-MiniLM-L6-v2 model. Create `AiService.embed()` RPC endpoint accepting text list and returning
vectors. Load model lazily on first request. Verify by embedding test strings and confirming
384-dimension vectors returned.

### 3.4 Server AI Service: Transcription

Add transcription to server AI service using whisper.cpp bindings with bundled whisper-small model.
Create `AiService.transcribe()` RPC endpoint accepting audio bytes and format. Handle common audio
formats (WAV, MP3, M4A). Verify by transcribing a short audio clip and confirming accurate text
output.

### 3.5 Server AI Service: Completion Proxy

Implement completion routing to user-configured providers: Ollama (local), Anthropic, OpenAI,
OpenRouter. Create `AiService.complete()` RPC endpoint returning `Flow<String>` for token streaming.
Store provider configuration in server settings. Verify by configuring Ollama and making a
completion request.

### 3.6 Testing Infrastructure

Set up commonTest with Mokkery 3.x for multiplatform mocking and Turbine for Flow testing. Create
fake implementations for core repositories (`FakeQuestRepository`, `FakeNoteRepository`,
`FakeItemRepository`) as the primary testing pattern. Add kotlinx-coroutines-test for coroutine
testing. Configure Koin test modules for dependency injection in tests. Verify by writing a sample
use case test that uses fakes and confirming tests pass on all platforms.

---

## Phase 4: Guidance Module

### 4.1 Quest CRUD Operations

Implement repository methods for creating, reading, updating, and deleting Quests. Quests have title
(required, max 200 chars), description (optional), energy_cost (1-5), and status (backlog, active,
completed, abandoned). All repository methods return `Either<QuestError, T>` for typed error
handling. Enforce WIP=1 rule: return `WipLimitExceeded` error when starting a Quest if another is
already active. Create `QuestListComponent` (Decompose) exposing quest state via `StateFlow`. Verify
by creating a Quest via the UI and confirming it persists across app restart.

### 4.2 Quest List and Detail Views

Create `QuestListContent` composable showing backlog and completed Quests grouped by status. Each
Quest card displays title, energy cost (as dots/icons), and Epic name if linked. Create
`QuestDetailComponent` with child navigation from list. Clicking a Quest navigates to detail screen
with full description and edit capability. Include FAB to create new Quests. Verify by creating
multiple Quests with different statuses and confirming proper grouping and navigation.

### 4.3 Active Quest and Focus Mode

Show the active Quest prominently at the top of the Guidance view with a "Complete" button and
energy cost indicator. Implement Focus Mode triggered by a button that hides navigation and shows
only the active Quest with its Checkpoints. Include a visible timer showing elapsed time. Verify by
starting a Quest, entering Focus Mode, and confirming distraction-free experience.

### 4.4 Checkpoints

Add Checkpoint support to Quests—ordered sub-steps with title and completed status. Checkpoints can
be added, reordered (drag-and-drop), and checked off from Quest detail. Display Checkpoint progress
as fraction (e.g., "3/5") on Quest cards. Completing all Checkpoints does NOT auto-complete the
Quest. Verify by adding Checkpoints, reordering, and checking them off.

### 4.5 Energy Budget System

Create `energy_budget` table storing daily budget and spent values keyed by date. Display energy
meter in Guidance header showing remaining energy for today. Completing a Quest deducts its
energy_cost from daily budget. Show warning (not block) if completing Quest would exceed budget.
Verify by completing Quests and watching energy meter decrease.

### 4.6 Epic Management

Implement Epic CRUD with title, description, and status (active, completed, archived). Quests can
optionally link to an Epic via `epic_id`. Show Epics in collapsible list with child Quests nested.
Epic status becomes "completed" when all its Quests complete. Verify by creating Epic, adding
Quests, completing them to see Epic complete.

---

## Phase 5: Knowledge Module

### 5.1 Note CRUD Operations

Implement repository methods for creating, reading, updating, and deleting Notes. Notes have title
(required, max 200 chars, unique within folder), content (Markdown), and optional folder_id.
Deleting is soft delete (sets `deleted_at`). Create ViewModel exposing note state. Verify by
creating a Note and confirming persistence across app restart.

### 5.2 Note Editor

Create Markdown editor for Note content using a Compose-compatible Markdown editor library. Support
basic formatting (headings, bold, italic, lists, code blocks) with keyboard shortcuts. Auto-save
content after debounced delay (500ms). Show "saved" indicator when persisted. Verify by editing,
waiting for auto-save, and refreshing to confirm persistence.

### 5.3 Folder Hierarchy

Implement Folder CRUD with name, parent_id (for nesting), and order fields. Display folders in tree
structure in Knowledge sidebar with expand/collapse. Notes appear under parent folder; root-level
notes at top. Support drag-and-drop to move Notes between Folders. Verify by creating nested
folders, adding Notes, reorganizing via drag-and-drop.

### 5.4 Wiki-Links and Backlinks

Parse Note content for wiki-link syntax `[[Note Title]]` on save and create `note_link` records.
Display clickable links in rendered Markdown navigating to linked Note. Show "Backlinks" section in
Note detail listing all Notes that link to this one. Broken links can create Note on click. Verify
by creating links between Notes and confirming bidirectional navigation.

### 5.5 Tags

Implement Tag CRUD with name (lowercase, unique) and optional color. Add tag input to Note editor
with typeahead for existing tags and inline creation. Display tags as colored pills on Note cards in
list view. Create tag filter in sidebar to show Notes with specific tag. Verify by tagging Notes and
filtering by tag.

### 5.6 Note Search (Full-Text)

Add full-text search index on Note content using SurrealDB search (desktop) or SQLite FTS5 (mobile).
Create search bar in Knowledge header filtering Notes by content match. Display results with
highlighted snippets. Target <100ms for up to 10,000 Notes. Verify by creating Notes with distinct
content and confirming search finds correct ones.

### 5.7 Note Search (Semantic) — Desktop Only

Request embeddings from server for Note content, storing vectors locally. Regenerate on content
change (background). Add "Find similar" button on Notes querying nearest neighbors by vector
similarity. Display results ranked by score. Verify by finding similar Notes to one about specific
topic.

### 5.8 Attachments

Support attaching files to Notes by drag-and-drop or file picker. Store files in app data directory
with content-hash filenames for deduplication. Display images inline; other files as download links.
Limit to 100MB. Verify by attaching image and PDF, confirming display/download.

---

## Phase 6: Tracking Module

### 6.1 Item CRUD Operations

Implement repository methods for creating, reading, updating, and deleting Items. Items have name
(required, max 200 chars), description, quantity (default 1), and optional location_id OR
container_id (exclusive). Create ViewModel exposing item state. Verify by creating Item and
confirming persistence across app restart.

### 6.2 Item List and Detail Views

Create Item list screen with grid/list view toggle. Each Item card shows name, quantity,
location/container, and primary image if set. Clicking opens detail view with full information and
edit capability. Include FAB to create new Items. Verify by creating multiple Items and switching
view modes.

### 6.3 Location Hierarchy

Implement Location CRUD with name, description, parent_id for nesting. Display Locations in tree
structure in Tracking sidebar. Items at Location appear in Location detail view. Support
drag-and-drop to move Items between Locations. Verify by creating nested Locations and moving Items.

### 6.4 Containers

Implement Container CRUD with name, description, location_id, parent_id (containers can nest).
Containers appear as special item type holding other Items. Moving Container moves all Items inside.
Display contents in collapsible list. Verify by creating nested Containers with Items and moving
parent.

### 6.5 Item Templates and Custom Fields

Implement ItemTemplate with name, description, icon, plus FieldDefinition for template fields.
Creating Item with template pre-populates CustomField entries. CustomFields support types: text,
number, date, boolean, url, enum. Display custom fields in Item detail with appropriate widgets.
Verify by creating template, making Item from it, editing fields.

### 6.6 Item Photo Capture

Add primary image field to Items with file upload and clipboard paste support. Display image
prominently in detail and as thumbnail in lists. Resize to max 1920px on import. Support JPEG, PNG,
WebP. Verify by adding image via drag-and-drop and clipboard paste.

### 6.7 Low Stock Alerts

Publish `TrackingEvent.QuantityZero` when Item quantity reaches zero. Display badge on zero-quantity
Items in list. Create "Low Stock" view filtering Items below threshold. Verify by decrementing Item
to zero and confirming indicator.

---

## Phase 7: Cross-Module Integration

### 7.1 Quest-Note References

Add `quest_note` relation linking Quests to Notes. In Quest detail, show linked Notes with picker to
add more. In Note detail, show linked Quests. Clicking navigates to detail view. Verify by linking
Note to Quest and navigating between.

### 7.2 Quest-Item References

Add `quest_item` relation linking Quests to Items. In Quest detail, show linked Items (tools needed)
with picker. In Item detail, show linked Quests. Verify by linking Item to Quest and navigating
between.

### 7.3 Note-Item Mentions

Parse Note content for Item mentions using `[[Item:Name]]` syntax. Create links from Notes to Items.
Display mentioned Items in Note detail. Show "Mentioned in" Notes in Item detail. Verify by
mentioning Item in Note and confirming bidirectional links.

### 7.4 Cross-Module Event Reactions

Subscribe to events across modules for suggestions. Quest completion prompts reflection Note. Item
quantity zero suggests restock Quest. Display suggestions as dismissible cards. Verify by completing
Quest and seeing reflection prompt.

---

## Phase 8: Settings and Polish

### 8.1 Settings Panel

Create Settings screen accessible from top bar with sections: General, AI, Data, Account. General:
theme (light/dark/system), default energy budget. AI: completion provider selection. Data: export,
import, clear. Account: server URL, logout. Verify by changing settings and confirming persistence.

### 8.2 Server AI Provider Configuration

Server settings page for AI provider configuration. Allow selection of Ollama, Anthropic, OpenAI,
OpenRouter for completion. Validate API keys on save. Show clear errors if provider unavailable.
Verify by configuring provider and using for completion.

### 8.3 Data Export

Implement export of all user data as JSON file containing all entities and relationships. Option to
export Notes as individual Markdown files in ZIP. Show progress for large datasets. Verify by
exporting, deleting database, confirming JSON contains all records.

### 8.4 Data Import

Implement import from JSON export format, merging or replacing based on user choice. Validate
structure before applying. Show preview of import counts by entity type. Verify by importing
previously exported JSON and confirming data restored.

### 8.5 Onboarding Flow

Create first-launch onboarding introducing three modules and core concepts. Step to set daily energy
budget default. Optionally create sample data to demonstrate features. Mark complete in settings so
it doesn't repeat. Verify by clearing app data and confirming onboarding appears.

### 8.6 Keyboard Shortcuts (Desktop)

Implement global keyboard shortcuts: Cmd/Ctrl+N for new, Cmd/Ctrl+K for quick search, Cmd/Ctrl+1/2/3
to switch modules. Show shortcut reference via Cmd/Ctrl+?. Verify by using shortcuts to navigate and
create entities.

### 8.7 Trash and Recovery

Create Trash view showing soft-deleted entities from all modules. Allow restore or permanent delete.
Auto-purge items older than 30 days. Confirm before permanent deletion. Verify by deleting, finding
in Trash, restoring.

---

## Implementation Notes

- Each feature should be implemented as a focused development session
- Reference `docs/architecture/` for technical patterns
- Reference `docs/requirements/` for detailed acceptance criteria
- Run tests after each feature to catch regressions
- Commit after each successful feature implementation
- Desktop features may be implemented before mobile equivalents
- Server features required before sync-dependent client features
