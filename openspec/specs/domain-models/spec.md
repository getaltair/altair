# domain-models Specification

## Purpose
TBD - created by archiving change add-shared-domain-models. Update Purpose after archive.
## Requirements
### Requirement: ULID Identifier Support

The system SHALL use ULID (Universally Unique Lexicographically Sortable Identifier) for all entity
identifiers.

#### Scenario: ULID generation produces valid identifier

- **WHEN** a new ULID is generated
- **THEN** the result is a 26-character string
- **AND** the characters are from the Crockford Base32 alphabet
- **AND** the identifier is lexicographically sortable by creation time

#### Scenario: ULID validation rejects invalid input

- **WHEN** a ULID is constructed with an invalid string
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message describes the validation failure

### Requirement: System-Level User Entity

The system SHALL define a User entity representing an authenticated person.

#### Scenario: User entity contains required fields

- **WHEN** a User entity is created
- **THEN** it contains id, username, role, status, storageUsed, storageQuota, createdAt, lastLoginAt
- **AND** the entity is serializable to JSON

#### Scenario: User username validation

- **WHEN** a User is created with a blank username
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message indicates username must not be blank

#### Scenario: User username length validation

- **WHEN** a User is created with a username longer than 50 characters
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message indicates the maximum length

### Requirement: System-Level Initiative Entity

The system SHALL define an Initiative entity for cross-cutting organizational units.

#### Scenario: Initiative entity contains required fields

- **WHEN** an Initiative entity is created
- **THEN** it contains id, userId, name, description, parentId, ongoing, targetDate, status, focused
- **AND** the entity is serializable to JSON

#### Scenario: Initiative name validation

- **WHEN** an Initiative is created with a blank name
- **THEN** an IllegalArgumentException is thrown

#### Scenario: Initiative nesting depth validation

- **WHEN** an Initiative is created with a parentId
- **THEN** the parent reference is stored for hierarchy queries

### Requirement: System-Level InboxItem Entity

The system SHALL define an InboxItem entity for universal capture.

#### Scenario: InboxItem entity contains required fields

- **WHEN** an InboxItem entity is created
- **THEN** it contains id, userId, content, source, createdAt
- **AND** the source is one of: keyboard, voice, camera, share, widget, watch

#### Scenario: InboxItem content validation

- **WHEN** an InboxItem is created with blank content
- **THEN** an IllegalArgumentException is thrown

### Requirement: System-Level Routine Entity

The system SHALL define a Routine entity for recurring task templates.

#### Scenario: Routine entity contains required fields

- **WHEN** a Routine entity is created
- **THEN** it contains id, userId, name, schedule, energyCost, active, nextDue
- **AND** the schedule follows the Schedule sealed interface

#### Scenario: Routine energy cost validation

- **WHEN** a Routine is created with energyCost outside 1-5 range
- **THEN** an IllegalArgumentException is thrown

### Requirement: Schedule Value Object

The system SHALL define a Schedule sealed interface supporting multiple recurrence patterns.

#### Scenario: Daily schedule serialization

- **WHEN** a Daily schedule is serialized to JSON
- **THEN** the output contains type discriminator "daily"
- **AND** deserialization recreates the same schedule

#### Scenario: Weekly schedule with specific days

- **WHEN** a Weekly schedule is created with days [Monday, Wednesday, Friday]
- **THEN** those days are stored in the schedule
- **AND** serialization preserves the day set

#### Scenario: Monthly date schedule validation

- **WHEN** a MonthlyDate schedule is created with dayOfMonth outside 1-31
- **THEN** an IllegalArgumentException is thrown

#### Scenario: Interval schedule validation

- **WHEN** an Interval schedule is created with days less than 1
- **THEN** an IllegalArgumentException is thrown

### Requirement: Guidance Module Epic Entity

The system SHALL define an Epic entity for large goals.

#### Scenario: Epic entity contains required fields

- **WHEN** an Epic entity is created
- **THEN** it contains id, userId, title, description, status, initiativeId, createdAt, completedAt
- **AND** the status is one of: active, completed, archived

#### Scenario: Epic title validation

- **WHEN** an Epic is created with a blank title
- **THEN** an IllegalArgumentException is thrown

### Requirement: Guidance Module Quest Entity

The system SHALL define a Quest entity as the core unit of work.

#### Scenario: Quest entity contains required fields

- **WHEN** a Quest entity is created
- **THEN** it contains id, userId, title, energyCost, status, epicId, routineId
- **AND** the status is one of: backlog, active, completed, abandoned

#### Scenario: Quest energy cost validation

- **WHEN** a Quest is created with energyCost outside 1-5 range
- **THEN** an IllegalArgumentException is thrown

#### Scenario: Quest title length validation

- **WHEN** a Quest is created with title longer than 200 characters
- **THEN** an IllegalArgumentException is thrown

### Requirement: Guidance Module Checkpoint Entity

The system SHALL define a Checkpoint entity for optional sub-steps within Quests.

#### Scenario: Checkpoint entity contains required fields

- **WHEN** a Checkpoint entity is created
- **THEN** it contains id, questId, title, completed, order

#### Scenario: Checkpoint title validation

- **WHEN** a Checkpoint is created with a blank title
- **THEN** an IllegalArgumentException is thrown

### Requirement: Guidance Module EnergyBudget Entity

The system SHALL define an EnergyBudget entity for daily energy allocation.

#### Scenario: EnergyBudget entity contains required fields

- **WHEN** an EnergyBudget entity is created
- **THEN** it contains userId, date, budget, spent

#### Scenario: EnergyBudget range validation

- **WHEN** an EnergyBudget is created with budget outside 1-10 range
- **THEN** an IllegalArgumentException is thrown

### Requirement: Knowledge Module Note Entity

The system SHALL define a Note entity for knowledge content.

#### Scenario: Note entity contains required fields

- **WHEN** a Note entity is created
- **THEN** it contains id, userId, title, content, folderId, initiativeId, createdAt, updatedAt

#### Scenario: Note title uniqueness tracking

- **WHEN** a Note entity is created
- **THEN** the title and folderId are available for uniqueness checks

### Requirement: Knowledge Module NoteLink Entity

The system SHALL define a NoteLink entity for directional connections.

#### Scenario: NoteLink entity contains required fields

- **WHEN** a NoteLink entity is created
- **THEN** it contains id, sourceId, targetId, context, createdAt

### Requirement: Knowledge Module Folder Entity

The system SHALL define a Folder entity for hierarchical organization.

#### Scenario: Folder entity contains required fields

- **WHEN** a Folder entity is created
- **THEN** it contains id, userId, name, parentId, order

#### Scenario: Folder name validation

- **WHEN** a Folder is created with a name longer than 100 characters
- **THEN** an IllegalArgumentException is thrown

### Requirement: Knowledge Module Tag Entity

The system SHALL define a Tag entity for categorization.

#### Scenario: Tag entity contains required fields

- **WHEN** a Tag entity is created
- **THEN** it contains id, userId, name, color

#### Scenario: Tag name normalization

- **WHEN** a Tag name is stored
- **THEN** it is normalized to lowercase

### Requirement: Knowledge Module Attachment Entity

The system SHALL define an Attachment entity for file storage references.

#### Scenario: Attachment entity contains required fields

- **WHEN** an Attachment entity is created
- **THEN** it contains id, userId, noteId, filename, mimeType, sizeBytes, storageKey, hash

### Requirement: Knowledge Module SourceDocument Entity

The system SHALL define a SourceDocument entity for imported external documents.

#### Scenario: SourceDocument entity contains required fields

- **WHEN** a SourceDocument entity is created
- **THEN** it contains id, userId, title, sourceType, sourcePath, mimeType, contentHash, status

#### Scenario: SourceDocument status values

- **WHEN** a SourceDocument status is set
- **THEN** it is one of: pending, processing, completed, failed, stale

### Requirement: Knowledge Module SourceAnnotation Entity

The system SHALL define a SourceAnnotation entity for user annotations on documents.

#### Scenario: SourceAnnotation entity contains required fields

- **WHEN** a SourceAnnotation entity is created
- **THEN** it contains id, userId, sourceDocumentId, anchorType, anchorValue, content

#### Scenario: SourceAnnotation anchor types

- **WHEN** a SourceAnnotation anchorType is set
- **THEN** it is one of: document, page, heading, selection

### Requirement: Tracking Module Item Entity

The system SHALL define an Item entity for physical objects.

#### Scenario: Item entity contains required fields

- **WHEN** an Item entity is created
- **THEN** it contains id, userId, name, quantity, locationId, containerId, templateId

#### Scenario: Item quantity validation

- **WHEN** an Item is created with quantity less than 0
- **THEN** an IllegalArgumentException is thrown

### Requirement: Tracking Module Location Entity

The system SHALL define a Location entity for physical places.

#### Scenario: Location entity contains required fields

- **WHEN** a Location entity is created
- **THEN** it contains id, userId, name, description, parentId

### Requirement: Tracking Module Container Entity

The system SHALL define a Container entity for movable storage.

#### Scenario: Container entity contains required fields

- **WHEN** a Container entity is created
- **THEN** it contains id, userId, name, locationId, parentId

### Requirement: Tracking Module ItemTemplate Entity

The system SHALL define an ItemTemplate entity for predefined schemas.

#### Scenario: ItemTemplate entity contains required fields

- **WHEN** an ItemTemplate entity is created
- **THEN** it contains id, userId, name, description, icon

### Requirement: Tracking Module CustomField Entity

The system SHALL define a CustomField entity for user-defined attributes.

#### Scenario: CustomField entity contains required fields

- **WHEN** a CustomField entity is created
- **THEN** it contains id, itemId, name, fieldType, value

#### Scenario: CustomField type support

- **WHEN** a CustomField fieldType is set
- **THEN** it is one of: text, number, date, boolean, url, enum

### Requirement: Tracking Module FieldDefinition Entity

The system SHALL define a FieldDefinition entity for template field specifications.

#### Scenario: FieldDefinition entity contains required fields

- **WHEN** a FieldDefinition entity is created
- **THEN** it contains id, templateId, name, fieldType, required, defaultValue, enumOptions, order

### Requirement: Soft Delete Support

Entities supporting soft delete SHALL include a deletedAt timestamp.

#### Scenario: SoftDeletable interface

- **WHEN** an entity implements SoftDeletable
- **THEN** it has a nullable deletedAt field
- **AND** an isDeleted computed property returns true when deletedAt is not null

### Requirement: Timestamp Tracking

Entities requiring audit timestamps SHALL implement the Timestamped interface.

#### Scenario: Timestamped interface

- **WHEN** an entity implements Timestamped
- **THEN** it has createdAt and updatedAt fields of type Instant

### Requirement: InviteCode Temporal Invariants

The InviteCode entity SHALL enforce temporal validity at construction time.

#### Scenario: Expiration must be after creation

- **WHEN** an InviteCode is constructed with `expiresAt <= createdAt`
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message indicates expiration must be after creation

#### Scenario: Creation time must not be in far future

- **WHEN** an InviteCode is constructed with `createdAt` more than 5 minutes in the future
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message indicates creation time is invalid

#### Scenario: Expiration must be within reasonable bounds

- **WHEN** an InviteCode is constructed with `expiresAt` more than 1 year after `createdAt`
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message indicates expiration duration is too long

#### Scenario: Valid temporal values accepted

- **WHEN** an InviteCode is constructed with valid temporal values
- **AND** `expiresAt > createdAt`
- **AND** `createdAt <= now + 5 minutes`
- **AND** `expiresAt <= createdAt + 1 year`
- **THEN** the InviteCode is created successfully

### Requirement: InviteCode Factory Method

The InviteCode entity SHALL provide a factory method for duration-based construction.

#### Scenario: Create with expiration duration

- **WHEN** `InviteCode.create()` is called with an expiration duration
- **THEN** the InviteCode is created with `createdAt` set to the current time
- **AND** `expiresAt` set to `createdAt + duration`
- **AND** all temporal invariants are satisfied

### Requirement: RefreshToken Temporal Invariants

The RefreshToken entity SHALL enforce temporal validity at construction time.

#### Scenario: Expiration must be after creation

- **WHEN** a RefreshToken is constructed with `expiresAt <= createdAt`
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message indicates expiration must be after creation

#### Scenario: Creation time must not be in far future

- **WHEN** a RefreshToken is constructed with `createdAt` more than 5 minutes in the future
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message indicates creation time is invalid

#### Scenario: Expiration must be within reasonable bounds

- **WHEN** a RefreshToken is constructed with `expiresAt` more than 90 days after `createdAt`
- **THEN** an IllegalArgumentException is thrown
- **AND** the error message indicates expiration duration is too long

#### Scenario: Valid temporal values accepted

- **WHEN** a RefreshToken is constructed with valid temporal values
- **AND** `expiresAt > createdAt`
- **AND** `createdAt <= now + 5 minutes`
- **AND** `expiresAt <= createdAt + 90 days`
- **THEN** the RefreshToken is created successfully

### Requirement: RefreshToken Factory Method

The RefreshToken entity SHALL provide a factory method for duration-based construction.

#### Scenario: Create with expiration duration

- **WHEN** `RefreshToken.create()` is called with an expiration duration
- **THEN** the RefreshToken is created with `createdAt` set to the current time
- **AND** `expiresAt` set to `createdAt + duration`
- **AND** all temporal invariants are satisfied

