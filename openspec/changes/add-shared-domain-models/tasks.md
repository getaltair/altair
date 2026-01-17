# Tasks: Add Shared Domain Models

## 1. Foundation Types

- [ ] 1.1 Create `Ulid` value class with generator in `domain/types/Ulid.kt`
- [ ] 1.2 Create `Timestamped` interface in `domain/common/Timestamped.kt`
- [ ] 1.3 Create `SoftDeletable` interface in `domain/common/SoftDeletable.kt`
- [ ] 1.4 Create `Schedule` sealed interface in `domain/types/Schedule.kt`

## 2. Enum Definitions

- [ ] 2.1 Create `UserRole` enum (`admin`, `member`)
- [ ] 2.2 Create `UserStatus` enum (`active`, `disabled`, `deleted`)
- [ ] 2.3 Create `InitiativeStatus` enum (`active`, `paused`, `completed`, `archived`)
- [ ] 2.4 Create `QuestStatus` enum (`backlog`, `active`, `completed`, `abandoned`)
- [ ] 2.5 Create `EpicStatus` enum (`active`, `completed`, `archived`)
- [ ] 2.6 Create `CaptureSource` enum (`keyboard`, `voice`, `camera`, `share`, `widget`, `watch`)
- [ ] 2.7 Create `ExtractionStatus` enum (`pending`, `processing`, `completed`, `failed`, `stale`)
- [ ] 2.8 Create `AnchorType` enum (`document`, `page`, `heading`, `selection`)
- [ ] 2.9 Create `FieldType` enum (`text`, `number`, `date`, `boolean`, `url`, `enum`)
- [ ] 2.10 Create `SourceType` enum (`file`, `uri`, `watched`)
- [ ] 2.11 Create `JobStatus` enum (`queued`, `processing`, `completed`, `failed`)
- [ ] 2.12 Create `WeekOfMonth` enum (`first`, `second`, `third`, `fourth`, `last`)

## 3. System-Level Entities

- [ ] 3.1 Create `User` data class in `domain/model/system/User.kt`
- [ ] 3.2 Create `Initiative` data class in `domain/model/system/Initiative.kt`
- [ ] 3.3 Create `InboxItem` data class in `domain/model/system/InboxItem.kt`
- [ ] 3.4 Create `Routine` data class in `domain/model/system/Routine.kt`
- [ ] 3.5 Create `SourceDocument` data class in `domain/model/system/SourceDocument.kt`
- [ ] 3.6 Create `SourceAnnotation` data class in `domain/model/system/SourceAnnotation.kt`
- [ ] 3.7 Create `WatchedFolder` data class in `domain/model/system/WatchedFolder.kt`
- [ ] 3.8 Create `ExtractionJob` data class in `domain/model/system/ExtractionJob.kt`

## 4. Guidance Module Entities

- [ ] 4.1 Create `Epic` data class in `domain/model/guidance/Epic.kt`
- [ ] 4.2 Create `Quest` data class in `domain/model/guidance/Quest.kt`
- [ ] 4.3 Create `Checkpoint` data class in `domain/model/guidance/Checkpoint.kt`
- [ ] 4.4 Create `EnergyBudget` data class in `domain/model/guidance/EnergyBudget.kt`

## 5. Knowledge Module Entities

- [ ] 5.1 Create `Note` data class in `domain/model/knowledge/Note.kt`
- [ ] 5.2 Create `NoteLink` data class in `domain/model/knowledge/NoteLink.kt`
- [ ] 5.3 Create `Folder` data class in `domain/model/knowledge/Folder.kt`
- [ ] 5.4 Create `Tag` data class in `domain/model/knowledge/Tag.kt`
- [ ] 5.5 Create `Attachment` data class in `domain/model/knowledge/Attachment.kt`

## 6. Tracking Module Entities

- [ ] 6.1 Create `Item` data class in `domain/model/tracking/Item.kt`
- [ ] 6.2 Create `Location` data class in `domain/model/tracking/Location.kt`
- [ ] 6.3 Create `Container` data class in `domain/model/tracking/Container.kt`
- [ ] 6.4 Create `ItemTemplate` data class in `domain/model/tracking/ItemTemplate.kt`
- [ ] 6.5 Create `CustomField` data class in `domain/model/tracking/CustomField.kt`
- [ ] 6.6 Create `FieldDefinition` data class in `domain/model/tracking/FieldDefinition.kt`

## 7. Testing

- [ ] 7.1 Add unit tests for `Ulid` generation and validation
- [ ] 7.2 Add unit tests for `Schedule` serialization round-trip
- [ ] 7.3 Add unit tests for entity validation constraints (Quest, Note, User, etc.)
- [ ] 7.4 Add serialization round-trip tests for all entities

## 8. Verification

- [ ] 8.1 Run `./gradlew :shared:build` to verify compilation on all targets
- [ ] 8.2 Run `./gradlew :shared:allTests` to verify tests pass
- [ ] 8.3 Verify no lint/detekt warnings in new code
