## ADDED Requirements

### Requirement: Repository Return Types

Repository interfaces SHALL use Arrow Either for fallible operations and Flow for reactive queries.

#### Scenario: Single-shot operations return Either

- **WHEN** a repository method performs a single I/O operation (find, save, delete)
- **THEN** it returns `Either<DomainError, T>`
- **AND** the left type is a module-specific error

#### Scenario: Reactive queries return Flow

- **WHEN** a repository method provides a live query
- **THEN** it returns `Flow<List<T>>`
- **AND** the flow emits updated results when underlying data changes

#### Scenario: Operations are suspending

- **WHEN** a repository method performs I/O
- **THEN** it is marked with the `suspend` modifier
- **AND** can be called from coroutine contexts

### Requirement: User Scoping in Repositories

Repository implementations SHALL automatically scope all queries to the authenticated user.

#### Scenario: User context injection

- **WHEN** a repository implementation is constructed
- **THEN** it receives the authenticated user's ID
- **AND** all queries filter by that user ID

#### Scenario: Cross-user access prevented

- **WHEN** a query attempts to access another user's data
- **THEN** the result is empty or NotFound
- **AND** no error reveals the existence of other users' data

### Requirement: Initiative Repository Interface

The system SHALL define an InitiativeRepository interface for managing initiatives.

#### Scenario: Find initiative by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching initiative is returned
- **OR** NotFound error if no match exists

#### Scenario: Find root initiatives

- **WHEN** findRoots is called
- **THEN** all initiatives without a parent are returned
- **AND** results are ordered by name

#### Scenario: Find child initiatives

- **WHEN** findChildren is called with a parent ID
- **THEN** all direct children are returned
- **AND** results are ordered by name

#### Scenario: Find focused initiative

- **WHEN** findFocused is called
- **THEN** the initiative with focused=true is returned if any

#### Scenario: Save initiative

- **WHEN** save is called with an initiative
- **THEN** the initiative is persisted
- **AND** the saved entity with any generated fields is returned

### Requirement: Inbox Repository Interface

The system SHALL define an InboxRepository interface for universal capture.

#### Scenario: Find all inbox items

- **WHEN** findAll is called
- **THEN** all pending inbox items are returned
- **AND** results are ordered by createdAt descending (newest first)

#### Scenario: Find inbox items by source

- **WHEN** findBySource is called with a CaptureSource
- **THEN** items captured from that source are returned

#### Scenario: Save inbox item

- **WHEN** save is called with an inbox item
- **THEN** the item is persisted with a generated ID and timestamp

#### Scenario: Delete inbox item after triage

- **WHEN** delete is called with an inbox item ID
- **THEN** the item is permanently removed

### Requirement: Routine Repository Interface

The system SHALL define a RoutineRepository interface for managing routines.

#### Scenario: Find routines by next due

- **WHEN** findDueBefore is called with a timestamp
- **THEN** active routines with nextDue before that time are returned

#### Scenario: Find routines by initiative

- **WHEN** findByInitiative is called with an initiative ID
- **THEN** routines linked to that initiative are returned

#### Scenario: Find active routines

- **WHEN** findActive is called
- **THEN** routines with active=true are returned

### Requirement: Quest Repository Interface

The system SHALL define a QuestRepository interface for managing quests.

#### Scenario: Find quest by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching quest is returned
- **OR** QuestError.NotFound if no match exists

#### Scenario: Find quests by status

- **WHEN** findByStatus is called with a QuestStatus
- **THEN** quests with that status are returned as a reactive Flow

#### Scenario: Find active quests (WIP)

- **WHEN** findActive is called
- **THEN** quests with status=active are returned
- **AND** count is used for WIP limit enforcement

#### Scenario: Find quests by epic

- **WHEN** findByEpic is called with an epic ID
- **THEN** quests linked to that epic are returned

#### Scenario: Update quest status

- **WHEN** updateStatus is called with a new status
- **THEN** the status is updated if the transition is valid
- **OR** QuestError.InvalidStatusTransition is returned

### Requirement: Epic Repository Interface

The system SHALL define an EpicRepository interface for managing epics.

#### Scenario: Find epic by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching epic is returned

#### Scenario: Find epics by status

- **WHEN** findByStatus is called with an EpicStatus
- **THEN** epics with that status are returned

#### Scenario: Find epics by initiative

- **WHEN** findByInitiative is called with an initiative ID
- **THEN** epics linked to that initiative are returned

### Requirement: Checkpoint Repository Interface

The system SHALL define a CheckpointRepository interface for managing checkpoints.

#### Scenario: Find checkpoints by quest

- **WHEN** findByQuest is called with a quest ID
- **THEN** checkpoints for that quest are returned ordered by order field

#### Scenario: Reorder checkpoints

- **WHEN** reorder is called with checkpoint IDs in new order
- **THEN** the order fields are updated to match

### Requirement: Energy Budget Repository Interface

The system SHALL define an EnergyBudgetRepository interface for daily energy tracking.

#### Scenario: Find budget for date

- **WHEN** findByDate is called with a date
- **THEN** the energy budget for that date is returned
- **OR** a default budget is created if none exists

#### Scenario: Update spent energy

- **WHEN** updateSpent is called with a new spent value
- **THEN** the spent field is updated
- **OR** EnergyBudgetExceeded is returned if would exceed budget

### Requirement: Note Repository Interface

The system SHALL define a NoteRepository interface for managing notes.

#### Scenario: Find note by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching note is returned

#### Scenario: Find notes by folder

- **WHEN** findByFolder is called with a folder ID
- **THEN** notes in that folder are returned

#### Scenario: Find note by title

- **WHEN** findByTitle is called with a title and optional folder ID
- **THEN** the matching note is returned for wikilink resolution

#### Scenario: Search notes by content

- **WHEN** search is called with a query string
- **THEN** notes matching the query are returned
- **AND** matching is performed on title and content

#### Scenario: Find backlinks

- **WHEN** findBacklinks is called with a note ID
- **THEN** notes that link to the given note are returned

### Requirement: Note Link Repository Interface

The system SHALL define a NoteLinkRepository interface for bidirectional links.

#### Scenario: Find outgoing links

- **WHEN** findBySource is called with a note ID
- **THEN** all links from that note are returned

#### Scenario: Find incoming links

- **WHEN** findByTarget is called with a note ID
- **THEN** all links to that note are returned

#### Scenario: Create or update links

- **WHEN** syncLinks is called with a note ID and link targets
- **THEN** links are created, updated, or removed to match

### Requirement: Folder Repository Interface

The system SHALL define a FolderRepository interface for folder hierarchy.

#### Scenario: Find folder by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching folder is returned

#### Scenario: Find root folders

- **WHEN** findRoots is called
- **THEN** folders without a parent are returned

#### Scenario: Find child folders

- **WHEN** findChildren is called with a parent ID
- **THEN** direct child folders are returned ordered by order field

#### Scenario: Move folder

- **WHEN** move is called with a folder ID and new parent ID
- **THEN** the folder's parentId is updated
- **AND** circular references are prevented

### Requirement: Tag Repository Interface

The system SHALL define a TagRepository interface for tag management.

#### Scenario: Find all tags

- **WHEN** findAll is called
- **THEN** all user's tags are returned ordered by name

#### Scenario: Find tag by name

- **WHEN** findByName is called with a tag name
- **THEN** the matching tag is returned (case-insensitive)

#### Scenario: Find notes by tag

- **WHEN** findNotesByTag is called with a tag ID
- **THEN** notes with that tag are returned

### Requirement: Attachment Repository Interface

The system SHALL define an AttachmentRepository interface for file references.

#### Scenario: Find attachments by note

- **WHEN** findByNote is called with a note ID
- **THEN** all attachments for that note are returned

#### Scenario: Find attachment by storage key

- **WHEN** findByStorageKey is called with a storage key
- **THEN** the matching attachment is returned

### Requirement: Source Document Repository Interface

The system SHALL define a SourceDocumentRepository interface for external documents.

#### Scenario: Find by extraction status

- **WHEN** findByStatus is called with an ExtractionStatus
- **THEN** documents with that status are returned

#### Scenario: Find by content hash

- **WHEN** findByContentHash is called with a hash
- **THEN** documents with that hash are returned for deduplication

### Requirement: Item Repository Interface

The system SHALL define an ItemRepository interface for tracking items.

#### Scenario: Find item by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching item is returned

#### Scenario: Find items by location

- **WHEN** findByLocation is called with a location ID
- **THEN** items at that location are returned

#### Scenario: Find items by container

- **WHEN** findByContainer is called with a container ID
- **THEN** items in that container are returned

#### Scenario: Search items

- **WHEN** search is called with a query string
- **THEN** items matching by name or description are returned

#### Scenario: Update quantity

- **WHEN** updateQuantity is called with a delta
- **THEN** the quantity is adjusted
- **OR** InvalidQuantity error if result would be negative

### Requirement: Location Repository Interface

The system SHALL define a LocationRepository interface for location hierarchy.

#### Scenario: Find location by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching location is returned

#### Scenario: Find root locations

- **WHEN** findRoots is called
- **THEN** locations without a parent are returned

#### Scenario: Find child locations

- **WHEN** findChildren is called with a parent ID
- **THEN** direct child locations are returned

### Requirement: Container Repository Interface

The system SHALL define a ContainerRepository interface for movable storage.

#### Scenario: Find container by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching container is returned

#### Scenario: Find containers by location

- **WHEN** findByLocation is called with a location ID
- **THEN** containers at that location are returned

#### Scenario: Move container

- **WHEN** move is called with a container ID and new location ID
- **THEN** the container's locationId is updated
- **AND** nested containers are moved with it

### Requirement: Item Template Repository Interface

The system SHALL define an ItemTemplateRepository interface for item schemas.

#### Scenario: Find all templates

- **WHEN** findAll is called
- **THEN** all item templates are returned

#### Scenario: Find template with field definitions

- **WHEN** findWithFields is called with a template ID
- **THEN** the template and its field definitions are returned

### Requirement: User Repository Interface

The system SHALL define a UserRepository interface for multi-user support.

#### Scenario: Find user by ID

- **WHEN** findById is called with a valid ID
- **THEN** the matching user is returned

#### Scenario: Find user by username

- **WHEN** findByUsername is called with a username
- **THEN** the matching user is returned for authentication

#### Scenario: Update storage usage

- **WHEN** updateStorageUsed is called with a new value
- **THEN** the user's storageUsed is updated
