# dto Specification

## Purpose
TBD - created by archiving change add-interfaces-and-types. Update Purpose after archive.
## Requirements
### Requirement: DTO Serialization

All DTO types SHALL be serializable using kotlinx.serialization for JSON transport.

#### Scenario: DTO round-trip serialization

- **WHEN** any DTO is serialized to JSON
- **THEN** deserialization recreates an equivalent instance
- **AND** property names use camelCase by default

#### Scenario: Optional fields nullable

- **WHEN** a DTO field is optional
- **THEN** it is represented as nullable with default null
- **AND** null values are omitted from JSON output

### Requirement: Authentication DTOs

The system SHALL define DTOs for authentication flows.

#### Scenario: Login request

- **WHEN** a user submits login credentials
- **THEN** AuthRequest contains username and password fields
- **AND** password is never logged or persisted

#### Scenario: Login response

- **WHEN** login succeeds
- **THEN** AuthResponse contains accessToken, refreshToken, and expiresAt
- **AND** user profile data is included

#### Scenario: Token refresh request

- **WHEN** the access token expires
- **THEN** TokenRefreshRequest contains the refresh token
- **AND** TokenRefreshResponse provides new tokens

#### Scenario: Registration request

- **WHEN** a new user registers
- **THEN** RegisterRequest contains username, password, and inviteCode
- **AND** password complexity is not validated at DTO level

### Requirement: Synchronization DTOs

The system SHALL define DTOs for offline sync protocol.

#### Scenario: Sync pull request

- **WHEN** a client requests updates
- **THEN** SyncRequest contains clientId and lastSyncVersion
- **AND** an optional filter for entity types

#### Scenario: Sync pull response

- **WHEN** the server responds to a pull
- **THEN** SyncResponse contains changes, currentVersion, and hasMore
- **AND** pagination is supported via hasMore flag

#### Scenario: Entity change format

- **WHEN** an entity change is transmitted
- **THEN** EntityChange contains entityType, entityId, operation, version, and data
- **AND** operation is one of: create, update, delete

#### Scenario: Conflict info format

- **WHEN** a sync conflict is detected
- **THEN** ConflictInfo contains entityType, entityId, clientVersion, serverVersion
- **AND** both client and server data are included for resolution

### Requirement: Quest DTOs

The system SHALL define DTOs for Quest operations.

#### Scenario: Create quest request

- **WHEN** a new quest is created
- **THEN** CreateQuestRequest contains title, description, energyCost, epicId
- **AND** epicId and description are optional

#### Scenario: Update quest request

- **WHEN** a quest is updated
- **THEN** UpdateQuestRequest contains optional fields for each updatable property
- **AND** only non-null fields are applied

#### Scenario: Quest response

- **WHEN** a quest is returned from the API
- **THEN** QuestResponse contains all quest fields plus computed fields
- **AND** checkpoint count is included

### Requirement: Epic DTOs

The system SHALL define DTOs for Epic operations.

#### Scenario: Create epic request

- **WHEN** a new epic is created
- **THEN** CreateEpicRequest contains title, description, initiativeId
- **AND** description and initiativeId are optional

#### Scenario: Epic response

- **WHEN** an epic is returned from the API
- **THEN** EpicResponse contains all epic fields plus quest count

### Requirement: Note DTOs

The system SHALL define DTOs for Note operations.

#### Scenario: Create note request

- **WHEN** a new note is created
- **THEN** CreateNoteRequest contains title, content, folderId, initiativeId
- **AND** only title is required

#### Scenario: Update note request

- **WHEN** a note is updated
- **THEN** UpdateNoteRequest contains optional title, content, folderId
- **AND** wikilinks in content are parsed server-side

#### Scenario: Note response

- **WHEN** a note is returned from the API
- **THEN** NoteResponse contains all note fields plus backlink count

### Requirement: Item DTOs

The system SHALL define DTOs for Item operations.

#### Scenario: Create item request

- **WHEN** a new item is created
- **THEN** CreateItemRequest contains name, quantity, locationId, containerId, templateId
- **AND** location and container are mutually exclusive

#### Scenario: Update item request

- **WHEN** an item is updated
- **THEN** UpdateItemRequest contains optional fields for each updatable property
- **AND** quantity delta can be specified instead of absolute value

#### Scenario: Item response

- **WHEN** an item is returned from the API
- **THEN** ItemResponse contains all item fields plus location and container names

### Requirement: Initiative DTOs

The system SHALL define DTOs for Initiative operations.

#### Scenario: Create initiative request

- **WHEN** a new initiative is created
- **THEN** CreateInitiativeRequest contains name, description, parentId, targetDate
- **AND** only name is required

#### Scenario: Update initiative request

- **WHEN** an initiative is updated
- **THEN** UpdateInitiativeRequest contains optional fields
- **AND** focused can be set (will unfocus any other initiative)

#### Scenario: Initiative response

- **WHEN** an initiative is returned from the API
- **THEN** InitiativeResponse contains all initiative fields plus child count

### Requirement: Inbox DTOs

The system SHALL define DTOs for Inbox operations.

#### Scenario: Capture request

- **WHEN** content is captured to inbox
- **THEN** CaptureRequest contains content and source
- **AND** source indicates capture method (keyboard, voice, camera, etc.)

#### Scenario: Triage request

- **WHEN** an inbox item is triaged
- **THEN** TriageRequest contains inboxItemId, targetType, and targetData
- **AND** targetType is one of: quest, note, item, discard

#### Scenario: Inbox item response

- **WHEN** an inbox item is returned from the API
- **THEN** InboxItemResponse contains all inbox item fields

### Requirement: Routine DTOs

The system SHALL define DTOs for Routine operations.

#### Scenario: Create routine request

- **WHEN** a new routine is created
- **THEN** CreateRoutineRequest contains name, schedule, energyCost, initiativeId
- **AND** schedule is serialized according to Schedule sealed class

#### Scenario: Update routine request

- **WHEN** a routine is updated
- **THEN** UpdateRoutineRequest contains optional fields
- **AND** active status can be toggled

#### Scenario: Routine response

- **WHEN** a routine is returned from the API
- **THEN** RoutineResponse contains all routine fields plus next instance preview

### Requirement: Folder DTOs

The system SHALL define DTOs for Folder operations.

#### Scenario: Create folder request

- **WHEN** a new folder is created
- **THEN** CreateFolderRequest contains name and parentId
- **AND** parentId is optional for root folders

#### Scenario: Folder response

- **WHEN** a folder is returned from the API
- **THEN** FolderResponse contains all folder fields plus note count and child count

### Requirement: Location DTOs

The system SHALL define DTOs for Location operations.

#### Scenario: Create location request

- **WHEN** a new location is created
- **THEN** CreateLocationRequest contains name, description, parentId
- **AND** only name is required

#### Scenario: Location response

- **WHEN** a location is returned from the API
- **THEN** LocationResponse contains all location fields plus item count

### Requirement: Container DTOs

The system SHALL define DTOs for Container operations.

#### Scenario: Create container request

- **WHEN** a new container is created
- **THEN** CreateContainerRequest contains name, locationId, parentId
- **AND** locationId is required for root containers

#### Scenario: Container response

- **WHEN** a container is returned from the API
- **THEN** ContainerResponse contains all container fields plus item count

### Requirement: Pagination DTOs

The system SHALL define DTOs for paginated responses.

#### Scenario: Page request

- **WHEN** a paginated endpoint is called
- **THEN** PageRequest contains offset, limit, and optional sortBy
- **AND** default limit is 20, maximum is 100

#### Scenario: Page response wrapper

- **WHEN** a paginated response is returned
- **THEN** PageResponse contains items, totalCount, offset, and hasMore
- **AND** hasMore indicates if more pages exist

### Requirement: Error Response DTOs

The system SHALL define DTOs for error responses.

#### Scenario: Error response format

- **WHEN** an API error occurs
- **THEN** ErrorResponse contains code, message, and optional details
- **AND** code maps to DomainError subtype

#### Scenario: Validation error details

- **WHEN** validation fails
- **THEN** ValidationErrorResponse contains field-level errors
- **AND** each error has field name and message

