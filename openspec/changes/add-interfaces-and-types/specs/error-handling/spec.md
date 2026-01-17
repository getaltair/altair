## ADDED Requirements

### Requirement: Module-Specific Quest Errors

The system SHALL define a QuestError sealed interface for Guidance module failures.

#### Scenario: QuestNotFound error with context

- **WHEN** a quest lookup fails because the quest does not exist
- **THEN** QuestError.NotFound is returned with the quest ID
- **AND** the error is serializable for RPC transmission

#### Scenario: EnergyBudgetExceeded error with amounts

- **WHEN** activating a quest would exceed the daily energy budget
- **THEN** QuestError.EnergyBudgetExceeded is returned
- **AND** it contains the required and available energy values

#### Scenario: InvalidStatusTransition error

- **WHEN** a quest status change violates the state machine
- **THEN** QuestError.InvalidStatusTransition is returned
- **AND** it contains the current and attempted statuses

#### Scenario: WipLimitExceeded error

- **WHEN** activating a quest would exceed the WIP limit
- **THEN** QuestError.WipLimitExceeded is returned
- **AND** it contains the current active count and limit

### Requirement: Module-Specific Note Errors

The system SHALL define a NoteError sealed interface for Knowledge module failures.

#### Scenario: NoteNotFound error

- **WHEN** a note lookup fails because the note does not exist
- **THEN** NoteError.NotFound is returned with the note ID

#### Scenario: TitleConflict error

- **WHEN** creating or renaming a note would cause a duplicate title in the same folder
- **THEN** NoteError.TitleConflict is returned
- **AND** it contains the conflicting title and folder ID

#### Scenario: InvalidWikiLink error

- **WHEN** a wikilink target cannot be resolved
- **THEN** NoteError.InvalidWikiLink is returned
- **AND** it contains the unresolved link text

#### Scenario: FolderNotFound error

- **WHEN** a folder reference cannot be resolved
- **THEN** NoteError.FolderNotFound is returned with the folder ID

### Requirement: Module-Specific Item Errors

The system SHALL define an ItemError sealed interface for Tracking module failures.

#### Scenario: ItemNotFound error

- **WHEN** an item lookup fails because the item does not exist
- **THEN** ItemError.NotFound is returned with the item ID

#### Scenario: InvalidQuantity error

- **WHEN** an item quantity operation would result in negative quantity
- **THEN** ItemError.InvalidQuantity is returned
- **AND** it contains the attempted quantity and minimum allowed

#### Scenario: ContainerCycle error

- **WHEN** moving a container would create a circular containment
- **THEN** ItemError.ContainerCycle is returned
- **AND** it contains the container IDs forming the cycle

#### Scenario: LocationNotFound error

- **WHEN** a location reference cannot be resolved
- **THEN** ItemError.LocationNotFound is returned with the location ID

### Requirement: Synchronization Errors

The system SHALL define a SyncError sealed interface for client-server sync failures.

#### Scenario: ConflictDetected error

- **WHEN** the server detects a concurrent modification
- **THEN** SyncError.ConflictDetected is returned
- **AND** it contains the entity type, ID, and conflicting versions

#### Scenario: VersionMismatch error

- **WHEN** the client's sync version does not match server expectations
- **THEN** SyncError.VersionMismatch is returned
- **AND** it contains the client and server versions

#### Scenario: InvalidChangeSet error

- **WHEN** a sync payload fails validation
- **THEN** SyncError.InvalidChangeSet is returned
- **AND** it contains a description of the validation failure

### Requirement: Authentication Errors

The system SHALL define an AuthError sealed interface for authentication failures.

#### Scenario: InvalidCredentials error

- **WHEN** login credentials are incorrect
- **THEN** AuthError.InvalidCredentials is returned
- **AND** no details about which credential failed are exposed

#### Scenario: TokenExpired error

- **WHEN** an access token has expired
- **THEN** AuthError.TokenExpired is returned
- **AND** it contains the expiration timestamp

#### Scenario: TokenInvalid error

- **WHEN** a token fails signature or format validation
- **THEN** AuthError.TokenInvalid is returned
- **AND** it contains a reason code

#### Scenario: AccountLocked error

- **WHEN** too many failed login attempts have occurred
- **THEN** AuthError.AccountLocked is returned
- **AND** it contains the unlock timestamp if known

#### Scenario: InviteRequired error

- **WHEN** registration is attempted without an invite code
- **THEN** AuthError.InviteRequired is returned

### Requirement: Error Serialization

All domain error types SHALL be serializable for transmission over RPC and sync protocols.

#### Scenario: Error round-trip serialization

- **WHEN** any DomainError subtype is serialized to JSON
- **THEN** deserialization recreates an equivalent error instance
- **AND** the error type is preserved through @SerialName discriminators

#### Scenario: Transient fields excluded

- **WHEN** an error contains a Throwable cause
- **THEN** the cause is marked @Transient and excluded from serialization
