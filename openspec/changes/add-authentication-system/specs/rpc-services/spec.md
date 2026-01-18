## MODIFIED Requirements

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
