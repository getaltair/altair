# Altair Implementation Plan

## Overview

This document contains feature specifications for Altair. Each feature is described in 4-6 sentences suitable for
Spec-Driven Development (SDD). Implement features in order—later phases depend on earlier ones.

Reference the `docs/` folder for detailed requirements, domain model, and architecture decisions.

---

## Phase 1: Project Foundation

### 1.1 Tauri + Svelte Project Scaffold

Create a new Tauri 2 application with a Svelte 5 frontend using SvelteKit. Configure pnpm as the package manager and set
up Tailwind CSS 4 with the shadcn-svelte component library. The Rust backend should use the 2024 edition with a basic
"hello world" Tauri command that the frontend can invoke. Verify by running `pnpm tauri dev` and confirming the window
opens with a working IPC round-trip.

### 1.2 SurrealDB Embedded Integration

Add SurrealDB as an embedded database using the SurrealKV storage engine. Initialize the database on app startup in the
Tauri setup hook, storing data in the platform-appropriate app data directory. Create a simple Tauri command that writes
and reads a test record to verify the connection works. The database namespace should be "altair" and the database
name "main".

### 1.3 Application Shell and Navigation

Create the main application layout with a sidebar navigation showing three modules: Guidance, Knowledge, and Tracking.
Each module should have its own SvelteKit route group. Include a top bar with the app title and a placeholder settings
button. Use shadcn-svelte components for the navigation and layout structure. Verify by clicking each nav item and
confirming the URL and content area update.

---

## Phase 2: Core Infrastructure

### 2.1 Database Schema and Migrations

Implement a migration system that tracks applied migrations in a `_migration` table and runs pending migrations on
startup. Create initial migrations for the core tables: `epic`, `quest`, `checkpoint`, `energy_budget`, `note`,
`note_link`, `folder`, `tag`, `item`, `custom_field`, `location`, `container`, `item_template`, `field_definition`.
All entities use ULID string IDs and include `created_at`, `updated_at`, and optional `deleted_at` fields.
Verify by checking the database has all tables after fresh startup.

### 2.2 Event Bus Implementation

Create an in-process event bus using `tokio::sync::broadcast` that modules can publish to and subscribe from. Define
event types for each module category: `guidance:*`, `knowledge:*`, `tracking:*`, and `system:*`. Subscribers should
be able to filter by event prefix. Include a ring buffer that retains the last 1000 events for debugging. Verify by
publishing a test event and confirming a subscriber receives it.

### 2.3 Error Handling Framework

Define a unified error type (`AltairError`) using `thiserror` that covers database errors, validation errors, AI
provider errors, and not-found errors. Implement `Into<tauri::InvokeError>` so errors serialize properly to the
frontend. Create a frontend error handling utility that displays user-friendly toast messages for common error types.
Verify by triggering a not-found error and confirming the toast appears.

### 2.4 AI Service with Local Embedding

Implement the AI service with the `AiProvider` trait supporting `embed()`, `complete()`, and `transcribe()` methods. Add
the default local embedding provider using `ort` with a bundled `all-MiniLM-L6-v2` ONNX model that loads
lazily on first use. Create a Tauri command that accepts text and returns its embedding vector. Verify by embedding
a test string and confirming a 384-dimension vector is returned.

### 2.5 AI Service with Local Transcription

Add local transcription to the AI service using `whisper-rs` with a bundled `whisper-small` model that loads lazily on
first use. Create a Tauri command that accepts an audio file path and returns transcribed text. Handle common audio
formats (WAV, MP3, M4A). Verify by transcribing a short audio clip and confirming accurate text output.

---

## Phase 3: Guidance Module

### 3.1 Quest CRUD Operations

Implement Tauri commands for creating, reading, updating, and deleting Quests. Quests have title (required, max 200
chars), description (optional), energy_cost (1-5), and status (backlog, active, completed, abandoned). Enforce the
WIP=1 rule: reject starting a Quest if another is already active. Create a Svelte store that caches Quest data and
exposes reactive state. Verify by creating a Quest via the UI and confirming it persists across app restart.

### 3.2 Quest List and Detail Views

Create the Quest list page showing backlog and completed Quests grouped by status. Each Quest card should display title,
energy cost (as dots or icons), and Epic name if linked. Clicking a Quest opens a detail view with full description and
edit capability. Include a floating action button to create new Quests. Verify by creating multiple Quests with different
statuses and confirming proper grouping.

### 3.3 Active Quest and Focus Mode

Show the active Quest prominently at the top of the Guidance view with a "Complete" button and energy cost indicator.
Implement Focus Mode triggered by a button that hides the sidebar and shows only the active Quest with its Checkpoints.
Include a visible timer showing elapsed time on the Quest. Verify by starting a Quest, entering Focus Mode, and
confirming distractions are hidden.

### 3.4 Checkpoints

Add Checkpoint support to Quests—a list of sub-steps with title and completed status. Checkpoints can be added,
reordered (drag-and-drop), and checked off from the Quest detail view. Display Checkpoint progress as a fraction
(e.g., "3/5") on Quest cards. Completing all Checkpoints should NOT auto-complete the Quest. Verify by adding Checkpoints,
reordering them, and checking them off.

### 3.5 Energy Budget System

Create the `energy_budget` table storing daily budget and spent values keyed by date. Display an energy meter in the
Guidance header showing remaining energy for today. Completing a Quest deducts its energy_cost from the daily budget.
Show a warning (not a block) if completing a Quest would exceed the budget. Verify by completing Quests and watching
the energy meter decrease.

### 3.6 Epic Management

Implement Epic CRUD with title, description, and status (active, completed, archived). Quests can optionally link to an
Epic via `epic_id`. Show Epics in a collapsible list with their child Quests nested underneath. An Epic's status becomes
"completed" when all its Quests are completed. Verify by creating an Epic, adding Quests to it, and completing them to
see the Epic complete.

---

## Phase 4: Knowledge Module

### 4.1 Note CRUD Operations

Implement Tauri commands for creating, reading, updating, and deleting Notes. Notes have title (required, max 200 chars,
unique within folder), content (Markdown), and optional folder_id. Deleting a Note is a soft delete (sets `deleted_at`).
Create a Svelte store that caches Note data with reactive state. Verify by creating a Note and confirming it persists
across app restart.

### 4.2 Note Editor

Create a Markdown editor for Note content using a suitable Svelte Markdown editor component. Support basic formatting
(headings, bold, italic, lists, code blocks) with keyboard shortcuts. Auto-save content after a debounced delay (500ms)
when the user stops typing. Show a "saved" indicator when content is persisted. Verify by editing a Note, waiting for
auto-save, and refreshing to confirm content persisted.

### 4.3 Folder Hierarchy

Implement Folder CRUD with name, parent_id (for nesting), and order fields. Display folders in a tree structure in the
Knowledge sidebar with expand/collapse. Notes appear under their parent folder; root-level notes appear at the top.
Support drag-and-drop to move Notes between Folders. Verify by creating nested folders, adding Notes, and reorganizing
via drag-and-drop.

### 4.4 Wiki-Links and Backlinks

Parse Note content for wiki-link syntax `[[Note Title]]` on save and create `note_link` records. Display clickable links
in the rendered Markdown that navigate to the linked Note. Show a "Backlinks" section in the Note detail listing all
Notes that link to this one. If a link target doesn't exist, show it as a broken link that can create the Note on click.
Verify by creating links between Notes and confirming bidirectional navigation.

### 4.5 Tags

Implement Tag CRUD with name (lowercase, unique) and optional color. Add a tag input to the Note editor that supports
typeahead for existing tags and creates new tags inline. Display tags as colored pills on Note cards in the list view.
Create a tag filter in the sidebar to show Notes with a specific tag. Verify by tagging Notes and filtering by tag.

### 4.6 Note Search (Full-Text)

Add a full-text search index on Note content using SurrealDB's search capabilities. Create a search bar in the Knowledge
header that filters Notes by content match. Display search results with highlighted matching text snippets. Search should
be fast (<100ms) for up to 10,000 Notes. Verify by creating Notes with distinct content and confirming search finds the
correct ones.

### 4.7 Note Search (Semantic)

Generate embeddings for Note content using the local AI embedding provider, storing vectors in the `embedding` field.
Regenerate embeddings on content change (debounced, background task). Add a "Find similar" button on Notes that queries
for nearest neighbors by vector similarity. Display results ranked by similarity score. Verify by finding similar Notes
to one about a specific topic.

### 4.8 Attachments

Support attaching files to Notes by drag-and-drop or file picker. Store files in the app data directory with content-hash
filenames for deduplication. Display attached images inline in the Note; other files as download links. Limit file size
to 100MB. Verify by attaching an image and a PDF, then confirming they display/download correctly.

---

## Phase 5: Tracking Module

### 5.1 Item CRUD Operations

Implement Tauri commands for creating, reading, updating, and deleting Items. Items have name (required, max 200
chars), description, quantity (default 1), and optional location_id or container_id. An Item can have a location OR
be in a container, not both. Create a Svelte store for Items with reactive state. Verify by creating an Item and
confirming it persists across app restart.

### 5.2 Item List and Detail Views

Create the Item list page with a grid or list view toggle. Each Item card shows name, quantity, location/container,
and primary image if set. Clicking an Item opens a detail view with full information and edit capability. Include a
floating action button to create new Items. Verify by creating multiple Items and switching between view modes.

### 5.3 Location Hierarchy

Implement Location CRUD with name, description, and parent_id for nesting. Display Locations in a tree structure in
the Tracking sidebar. Items at a Location appear in that Location's detail view. Support drag-and-drop to move Items
between Locations. Verify by creating nested Locations and moving Items between them.

### 5.4 Containers

Implement Container CRUD with name, description, location_id, and parent_id (containers can nest). Containers appear
as a special item type that can hold other Items. Moving a Container moves all Items inside it. Display Container
contents in a collapsible list. Verify by creating nested Containers with Items and moving the parent Container.

### 5.5 Item Templates and Custom Fields

Implement ItemTemplate with name, description, and icon, plus FieldDefinition for the template's fields. When
creating an Item with a template, pre-populate CustomField entries from the template. CustomFields support types:
text, number, date, boolean, url, enum. Display custom fields in the Item detail view with appropriate input widgets.
Verify by creating a template, making an Item from it, and editing the custom fields.

### 5.6 Item Photo Capture

Add a primary image field to Items with support for file upload and clipboard paste. Display the image prominently
in the Item detail and as a thumbnail in list views. Resize images to a reasonable max dimension (1920px) on import
to save space. Support common formats (JPEG, PNG, WebP). Verify by adding an image to an Item via drag-and-drop and
clipboard paste.

### 5.7 Low Stock Alerts

Publish a `tracking:quantity_zero` event when an Item's quantity reaches zero. Display a badge or indicator on Items
with zero quantity in the list view. Create an optional "Low Stock" view that filters to Items with quantity below a
threshold. Verify by decrementing an Item to zero and confirming the visual indicator appears.

---

## Phase 6: Cross-Module Integration

### 6.1 Quest-Note References

Add a relation table `quest_note` linking Quests to Notes. In the Quest detail view, show linked Notes and provide a
picker to add more. In the Note detail view, show linked Quests. Clicking a linked item navigates to its detail view.
Verify by linking a Note to a Quest and navigating between them.

### 6.2 Quest-Item References

Add a relation table `quest_item` linking Quests to Items. In the Quest detail view, show linked Items (e.g., tools
needed) and provide a picker to add more. In the Item detail view, show linked Quests. Verify by linking an Item to a
Quest and navigating between them.

### 6.3 Note-Item Mentions

Parse Note content for Item mentions using `[[Item:Name]]` syntax. Create links from Notes to Items similar to
wiki-links. Display mentioned Items in the Note detail view. Show "Mentioned in" Notes in the Item detail view.
Verify by mentioning an Item in a Note and confirming bidirectional links.

### 6.4 Cross-Module Event Reactions

Subscribe to events across modules to trigger suggestions. When a Quest is completed, prompt to create a reflection
Note. When an Item's quantity hits zero, suggest creating a restock Quest. Display suggestions as dismissible cards
in the relevant module. Verify by completing a Quest and seeing the reflection Note prompt.

---

## Phase 7: Settings and Polish

### 7.1 Settings Panel

Create a Settings page accessible from the top bar with sections for General, AI, and Data. General: theme
(light/dark/system), default energy budget. AI: provider selection per capability, API key entry (stored in system
keychain). Data: export, import, clear data options. Verify by changing settings and confirming they persist.

### 7.2 AI Provider Configuration

Extend the AI service to support Ollama, Anthropic, OpenAI, and OpenRouter providers with user-configurable API
keys. Allow users to select which provider to use for completion, embedding, and transcription independently.
Validate API keys on save by making a test request. Show clear error messages if a provider is unavailable. Verify
by configuring an Anthropic API key and using it for completion.

### 7.3 Data Export

Implement export of all user data as a JSON file containing all entities and their relationships. Include an option
to export Notes as individual Markdown files in a ZIP. Show progress during export for large datasets. Verify by
exporting data, deleting the database, and confirming the JSON contains all records.

### 7.4 Data Import

Implement import from the JSON export format, merging or replacing existing data based on user choice. Validate the
import file structure before applying changes. Show a preview of what will be imported (counts by entity type).
Verify by importing a previously exported JSON and confirming data is restored.

### 7.5 Onboarding Flow

Create a first-launch onboarding flow that introduces the three modules and core concepts. Include a step to set the
daily energy budget default. Optionally create sample data (one Epic, a few Quests, sample Notes) to demonstrate
features. Mark onboarding complete in settings so it doesn't repeat. Verify by clearing app data and confirming
onboarding appears on next launch.

### 7.6 Keyboard Shortcuts

Implement global keyboard shortcuts: Cmd/Ctrl+N for new (Quest/Note/Item depending on context), Cmd/Ctrl+K for quick
search, Cmd/Ctrl+1/2/3 to switch modules. Show a keyboard shortcut reference accessible via Cmd/Ctrl+?. Use Tauri's
global shortcut API for app-wide shortcuts. Verify by using shortcuts to navigate and create entities.

### 7.7 Trash and Recovery

Create a Trash view showing soft-deleted entities from all modules. Allow restoring items from Trash or permanently
deleting them. Automatically purge items older than 30 days. Show empty trash confirmation before permanent deletion.
Verify by deleting an item, finding it in Trash, and restoring it.

---

## Implementation Notes

- Each feature should be implemented as a separate Droid session
- Reference `docs/architecture/` for technical patterns
- Reference `docs/requirements/` for detailed acceptance criteria
- Run the existing test suite after each feature to catch regressions
- Commit after each successful feature implementation
