## ADDED Requirements

### Requirement: Parameterized Query Support

The SurrealDbClient SHALL support parameterized queries using the SurrealDB Java SDK's native parameter binding.

#### Scenario: Execute query with parameters

- **WHEN** `queryBind` is called with a query string and parameter map
- **THEN** the query is executed with parameters bound using SDK's native binding
- **AND** the raw JSON result string is returned

#### Scenario: Execute query with parameters and deserialization

- **WHEN** `queryBindAs` is called with a query, parameters, and deserializer
- **THEN** the query is executed with bound parameters
- **AND** the result is deserialized using the provided function

#### Scenario: Execute DDL statement with parameters

- **WHEN** `executeBind` is called with a statement and parameter map
- **THEN** the statement is executed with parameters bound
- **AND** Unit is returned on success

#### Scenario: Parameter injection prevention

- **WHEN** a parameter value contains SQL/SurrealQL special characters (quotes, semicolons, etc.)
- **THEN** the characters are safely escaped by the SDK
- **AND** no injection attack is possible

### Requirement: Repository Parameterized Queries

All SurrealDB repository implementations SHALL use parameterized queries for user-provided values.

#### Scenario: String values use parameters

- **WHEN** a repository query includes a user-provided string value (email, name, title, etc.)
- **THEN** the value is passed as a parameter using `$paramName` syntax
- **AND** the value is NOT interpolated directly into the query string

#### Scenario: ID values use parameters

- **WHEN** a repository query filters by an entity ID
- **THEN** the ID value is passed as a parameter
- **AND** the table prefix remains in the query string (e.g., `user:$userId`)

#### Scenario: Numeric values use parameters

- **WHEN** a repository query includes a numeric value (count, bytes, etc.)
- **THEN** the value is passed as a parameter
- **AND** the database handles type conversion appropriately

#### Scenario: DateTime values use parameters

- **WHEN** a repository query includes a datetime value
- **THEN** the value is passed as a parameter
- **AND** the database handles datetime formatting appropriately

## MODIFIED Requirements

### Requirement: SurrealDB Repository Implementations

The server SHALL implement all repository interfaces using SurrealDB queries.

#### Scenario: Query user-scoped data

- **WHEN** a repository method is called
- **THEN** the query includes a user_id filter using parameterized binding
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
- **THEN** the query uses table query pattern with ID filter: `SELECT * FROM table WHERE id = table:$id AND user_id = ...`
- **AND** the query does NOT use direct record access with WHERE clause: `SELECT * FROM table:id WHERE ...`
- **BECAUSE** SurrealDB direct record access cannot be combined with WHERE clause filtering

#### Scenario: Query parameters are bound not interpolated

- **WHEN** any repository method constructs a query with variable values
- **THEN** variable values are passed using parameter binding (`$paramName`)
- **AND** values are NOT concatenated into the query string using string interpolation
