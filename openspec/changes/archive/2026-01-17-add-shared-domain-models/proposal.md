# Change: Add Shared Domain Models (Phase 2)

## Why

The shared KMP module needs domain entity definitions to enable client-server communication and
consistent data handling across all platforms (Android, iOS, Desktop, Server). Without these models,
we cannot build features in any module since all business logic depends on these core types.

This implements Phase 2 of the implementation plan, establishing the foundation for all subsequent
feature development.

## What Changes

### System-Level Entities

- `User` — Authenticated person with role, status, storage tracking
- `Initiative` — Cross-cutting organizational unit (projects/areas)
- `InboxItem` — Universal capture point awaiting triage
- `Routine` — Recurring template that spawns Quest instances

### Guidance Module Entities

- `Epic` — Large goal broken into Quests
- `Quest` — Core unit of work with energy cost
- `Checkpoint` — Optional sub-step within a Quest
- `EnergyBudget` — Daily energy allocation per user

### Knowledge Module Entities

- `Note` — Markdown content with links and attachments
- `NoteLink` — Directional connection between Notes
- `Folder` — Hierarchical container for Notes
- `Tag` — Flat-namespace label for categorization
- `Attachment` — File associated with Note or InboxItem
- `SourceDocument` — Imported external document
- `SourceAnnotation` — User annotation on SourceDocument

### Tracking Module Entities

- `Item` — Physical object being tracked
- `Location` — Physical place for storing Items
- `Container` — Movable storage unit
- `ItemTemplate` — Predefined schema for Item categories
- `CustomField` — User-defined attribute on Item
- `FieldDefinition` — Field specification in ItemTemplate

### Supporting Types

- Enums: `UserRole`, `UserStatus`, `InitiativeStatus`, `QuestStatus`, `EpicStatus`,
  `CaptureSource`, `ExtractionStatus`, `AnchorType`, `FieldType`, `SourceType`
- Value objects: `Schedule` sealed class for routine patterns
- Identifiers: `Ulid` value class with generation utility
- Common types: `SoftDeletable` interface, `Timestamped` interface

## Impact

- **Affected specs**: New `domain-models` capability (no existing specs affected)
- **Affected code**:
    - `shared/src/commonMain/kotlin/com/getaltair/altair/domain/` — All entity definitions
    - `shared/build.gradle.kts` — Already has required dependencies (Arrow, kotlinx-serialization)
- **Dependencies**: None (this is foundational)
- **Dependents**: All future features depend on these models
