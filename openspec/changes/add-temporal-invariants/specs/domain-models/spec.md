## ADDED Requirements

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
