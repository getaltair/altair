## MODIFIED Requirements

### Requirement: Authentication DTOs

The system SHALL define DTOs for authentication flows.

#### Scenario: Login request

- **WHEN** a user submits login credentials
- **THEN** AuthRequest contains email and password fields
- **AND** password is never logged or persisted

#### Scenario: Login response

- **WHEN** login succeeds
- **THEN** AuthResponse contains accessToken, refreshToken, and expiresIn
- **AND** userId is typed as Ulid for compile-time safety
- **AND** role is typed as UserRole enum for compile-time safety
- **AND** user displayName is included

#### Scenario: Token refresh request

- **WHEN** the access token expires
- **THEN** TokenRefreshRequest contains the refresh token
- **AND** TokenRefreshResponse provides new tokens

#### Scenario: Registration request

- **WHEN** a new user registers
- **THEN** RegisterRequest contains email, password, displayName, and optional inviteCode
- **AND** password complexity is not validated at DTO level
