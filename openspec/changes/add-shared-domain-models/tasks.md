# Tasks: Add Shared Domain Models

## 1. Foundation Types

- [x] 1.1 Create `Ulid` value class with generator in `domain/types/Ulid.kt`
- [x] 1.2 Create `Timestamped` interface in `domain/common/Timestamped.kt`
- [x] 1.3 Create `SoftDeletable` interface in `domain/common/SoftDeletable.kt`
- [x] 1.4 Create `Schedule` sealed interface in `domain/types/Schedule.kt`

## 2. Enum Definitions

- [x] 2.1 Create `UserRole` enum (`admin`, `member`)
- [x] 2.2 Create `UserStatus` enum (`active`, `disabled`, `deleted`)
- [x] 2.3 Create `InitiativeStatus` enum (`active`, `paused`, `completed`, `archived`)
- [x] 2.4 Create `QuestStatus` enum (`backlog`, `active`, `completed`, `abandoned`)
- [x] 2.5 Create `EpicStatus` enum (`active`, `completed`, `archived`)
- [x] 2.6 Create `CaptureSource` enum (`keyboard`, `voice`, `camera`, `share`, `widget`, `watch`)
- [x] 2.7 Create `ExtractionStatus` enum (`pending`, `processing`, `completed`, `failed`, `stale`)
- [x] 2.8 Create `AnchorType` enum (`document`, `page`, `heading`, `selection`)
- [x] 2.9 Create `FieldType` enum (`text`, `number`, `date`, `boolean`, `url`, `enum`)
- [x] 2.10 Create `SourceType` enum (`file`, `uri`, `watched`)
- [x] 2.11 Create `JobStatus` enum (`queued`, `processing`, `completed`, `failed`)
- [x] 2.12 Create `WeekOfMonth` enum (`first`, `second`, `third`, `fourth`, `last`)

## 3. System-Level Entities

- [x] 3.1 Create `User` data class in `domain/model/system/User.kt`
- [x] 3.2 Create `Initiative` data class in `domain/model/system/Initiative.kt`
- [x] 3.3 Create `InboxItem` data class in `domain/model/system/InboxItem.kt`
- [x] 3.4 Create `Routine` data class in `domain/model/system/Routine.kt`
- [x] 3.5 Create `SourceDocument` data class in `domain/model/system/SourceDocument.kt`
- [x] 3.6 Create `SourceAnnotation` data class in `domain/model/system/SourceAnnotation.kt`
- [x] 3.7 Create `WatchedFolder` data class in `domain/model/system/WatchedFolder.kt`
- [x] 3.8 Create `ExtractionJob` data class in `domain/model/system/ExtractionJob.kt`

## 4. Guidance Module Entities

- [x] 4.1 Create `Epic` data class in `domain/model/guidance/Epic.kt`
- [x] 4.2 Create `Quest` data class in `domain/model/guidance/Quest.kt`
- [x] 4.3 Create `Checkpoint` data class in `domain/model/guidance/Checkpoint.kt`
- [x] 4.4 Create `EnergyBudget` data class in `domain/model/guidance/EnergyBudget.kt`

## 5. Knowledge Module Entities

- [x] 5.1 Create `Note` data class in `domain/model/knowledge/Note.kt`
- [x] 5.2 Create `NoteLink` data class in `domain/model/knowledge/NoteLink.kt`
- [x] 5.3 Create `Folder` data class in `domain/model/knowledge/Folder.kt`
- [x] 5.4 Create `Tag` data class in `domain/model/knowledge/Tag.kt`
- [x] 5.5 Create `Attachment` data class in `domain/model/knowledge/Attachment.kt`

## 6. Tracking Module Entities

- [x] 6.1 Create `Item` data class in `domain/model/tracking/Item.kt`
- [x] 6.2 Create `Location` data class in `domain/model/tracking/Location.kt`
- [x] 6.3 Create `Container` data class in `domain/model/tracking/Container.kt`
- [x] 6.4 Create `ItemTemplate` data class in `domain/model/tracking/ItemTemplate.kt`
- [x] 6.5 Create `CustomField` data class in `domain/model/tracking/CustomField.kt`
- [x] 6.6 Create `FieldDefinition` data class in `domain/model/tracking/FieldDefinition.kt`

## 7. Testing

- [x] 7.1 Add unit tests for `Ulid` generation and validation
- [x] 7.2 Add unit tests for `Schedule` serialization round-trip
- [x] 7.3 Add unit tests for entity validation constraints (Quest, Note, User, etc.)
- [x] 7.4 Add serialization round-trip tests for all entities

## 8. Verification

- [x] 8.1 Run `./gradlew :shared:build` to verify compilation on all targets
- [x] 8.2 Run `./gradlew :shared:allTests` to verify tests pass
- [x] 8.3 Verify no lint/detekt warnings in new code
