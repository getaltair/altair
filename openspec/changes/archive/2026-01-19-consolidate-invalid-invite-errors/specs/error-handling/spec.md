## MODIFIED Requirements

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

#### Scenario: InvalidInviteCode error

- **WHEN** an invite code is invalid, expired, or already used
- **THEN** AuthError.InvalidInviteCode is returned
- **AND** no details about the rejected code are exposed (consistent with EmailAlreadyExists)
