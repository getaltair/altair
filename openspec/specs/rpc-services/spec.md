# rpc-services Specification

## Purpose
TBD - created by archiving change add-rpc-service-layer. Update Purpose after archive.
## Requirements
### Requirement: RPC Service Infrastructure

The system SHALL provide kotlinx-rpc infrastructure enabling type-safe communication between
Compose Multiplatform clients and the Ktor server.

#### Scenario: Service interfaces compile across all targets

- **WHEN** the shared module is built
- **THEN** RPC service interfaces compile for Android, iOS, and JVM targets

#### Scenario: Server exposes RPC endpoints

- **WHEN** the Ktor server starts
- **THEN** RPC services are accessible via WebSocket transport

#### Scenario: Client connects to server

- **WHEN** a client creates an RPC service stub
- **THEN** the client can invoke service methods on the server

### Requirement: Sync Service

The system SHALL provide a SyncService interface for synchronizing entity changes between
clients and server.

#### Scenario: Pull changes from server

- **WHEN** the client calls `SyncService.pull(since, entityTypes)`
- **THEN** the server returns all changes since the given version for requested entity types

#### Scenario: Push changes to server

- **WHEN** the client calls `SyncService.push(changes)`
- **THEN** the server acknowledges received changes and returns any conflicts

#### Scenario: Stream real-time changes (optional)

- **WHEN** the client subscribes to `SyncService.streamChanges(entityTypes)`
- **THEN** the client receives a Flow of entity changes as they occur on the server

### Requirement: Auth Service

The system SHALL provide an AuthService interface for user authentication, token management,
session control, and invite code generation.

#### Scenario: User login
- **WHEN** the client calls `AuthService.login(credentials)`
- **THEN** the server validates credentials against stored Argon2id hash
- **AND** returns access token, refresh token, and user information on success
- **AND** returns InvalidCredentials error on failure

#### Scenario: Token refresh
- **WHEN** the client calls `AuthService.refresh(refreshToken)`
- **THEN** the server validates the refresh token against the database
- **AND** returns a new access token and rotated refresh token if valid
- **AND** invalidates the old refresh token
- **AND** returns InvalidRefreshToken error if invalid or expired

#### Scenario: User logout
- **WHEN** the client calls `AuthService.logout()`
- **THEN** the server revokes the current refresh token
- **AND** the refresh token cannot be used for subsequent requests

#### Scenario: User registration
- **WHEN** the client calls `AuthService.register(registrationData)`
- **AND** a valid invite code is provided (or no users exist for bootstrap)
- **THEN** the server creates a new user with hashed password
- **AND** returns authentication tokens for the new account
- **AND** marks the invite code as consumed

#### Scenario: Generate invite code
- **WHEN** an admin user calls `AuthService.generateInviteCode(expirationDays)`
- **THEN** the server generates a unique invite code
- **AND** stores it with the specified expiration
- **AND** returns the invite code to the admin

#### Scenario: Change password
- **WHEN** an authenticated user calls `AuthService.changePassword(currentPassword, newPassword)`
- **AND** the current password is correct
- **THEN** the server updates the password hash
- **AND** returns success

#### Scenario: Revoke all sessions
- **WHEN** an authenticated user calls `AuthService.revokeAllSessions()`
- **THEN** the server invalidates all refresh tokens for that user
- **AND** the user must re-authenticate on all devices

### Requirement: AI Service

The system SHALL provide an AiService interface for server-centralized AI capabilities.

#### Scenario: Generate embeddings

- **WHEN** the client calls `AiService.embed(texts)`
- **THEN** the server returns embedding vectors for each input text

#### Scenario: Transcribe audio

- **WHEN** the client calls `AiService.transcribe(audioData, format)`
- **THEN** the server returns the transcribed text

#### Scenario: Stream completions

- **WHEN** the client calls `AiService.complete(request)`
- **THEN** the server returns a Flow of completion tokens as they are generated

### Requirement: RPC Error Handling

The system SHALL propagate domain errors through RPC calls in a type-safe manner.

#### Scenario: Domain error returned

- **WHEN** a service method encounters a domain error (e.g., invalid credentials)
- **THEN** the error is serialized and returned to the client as a structured response

#### Scenario: Network error handling

- **WHEN** the RPC connection fails
- **THEN** the client receives a network error that can be handled gracefully

### Requirement: RPC Client Factory

The system SHALL provide a factory for creating RPC service stubs in client applications.

#### Scenario: Create service stub via Koin

- **WHEN** a component requests an RPC service from Koin
- **THEN** the DI container provides a configured service stub

#### Scenario: Configure server connection

- **WHEN** the RpcClientFactory is initialized with server configuration
- **THEN** all service stubs connect to the specified server endpoint

