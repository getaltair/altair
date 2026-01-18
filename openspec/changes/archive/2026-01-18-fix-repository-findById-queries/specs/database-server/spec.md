## MODIFIED Requirements

### Requirement: SurrealDB Repository Implementations

The server SHALL implement all repository interfaces using SurrealDB queries.

#### Scenario: Query user-scoped data

- **WHEN** a repository method is called
- **THEN** the query includes a user_id filter
- **AND** only the authenticated user's data is returned

#### Scenario: Handle not found

- **WHEN** a findById query returns no results
- **THEN** the repository returns a NotFound error
- **AND** the error includes the requested ID

#### Scenario: Handle database errors

- **WHEN** a database query fails
- **THEN** the repository returns a DatabaseError
- **AND** the original exception is logged
- **AND** sensitive details are not exposed to callers

#### Scenario: Query by ID with user scope

- **WHEN** findById is called with a record ID
- **THEN** the query uses table query pattern with ID filter: `SELECT * FROM table WHERE id = table:id AND user_id = ...`
- **AND** the query does NOT use direct record access with WHERE clause: `SELECT * FROM table:id WHERE ...`
- **BECAUSE** SurrealDB direct record access cannot be combined with WHERE clause filtering
