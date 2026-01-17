## ADDED Requirements

### Requirement: SurrealDB Server Connection

The server SHALL connect to SurrealDB using the surrealdb-java SDK with configurable connection settings.

#### Scenario: Connect with environment configuration

- **WHEN** the server starts
- **THEN** it reads SurrealDB connection settings from environment variables
- **AND** establishes a connection pool to the database

#### Scenario: Connection failure handling

- **WHEN** SurrealDB is unreachable at startup
- **THEN** the server logs the error
- **AND** health check endpoint returns unhealthy status

#### Scenario: Graceful shutdown

- **WHEN** the server shuts down
- **THEN** all database connections are closed gracefully
- **AND** pending transactions are committed or rolled back

### Requirement: SurrealDB Schema Migrations

The server SHALL apply database schema migrations at startup.

#### Scenario: Run pending migrations

- **WHEN** the server starts with unapplied migrations
- **THEN** migrations are applied in version order
- **AND** the migration table records applied versions

#### Scenario: Skip applied migrations

- **WHEN** the server starts with all migrations applied
- **THEN** no migration queries are executed
- **AND** startup proceeds normally

#### Scenario: Migration failure

- **WHEN** a migration fails to apply
- **THEN** the server logs the error with migration version
- **AND** startup is aborted
- **AND** database state is unchanged (transaction rollback)

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

### Requirement: Server Database Configuration

The server SHALL support configurable database settings via environment variables.

#### Scenario: Configure connection URL

- **WHEN** SURREALDB_URL environment variable is set
- **THEN** the server connects to that URL
- **AND** default is ws://localhost:8000/rpc

#### Scenario: Configure namespace and database

- **WHEN** SURREALDB_NAMESPACE and SURREALDB_DATABASE are set
- **THEN** the server uses those values
- **AND** defaults are "altair" and "main" respectively

#### Scenario: Configure authentication

- **WHEN** SURREALDB_USER and SURREALDB_PASS are set
- **THEN** the server authenticates with those credentials
- **AND** default user is "root"

### Requirement: Connection Pool Management

The server SHALL manage a pool of database connections for concurrent request handling.

#### Scenario: Acquire connection from pool

- **WHEN** a request needs database access
- **THEN** a connection is acquired from the pool
- **AND** the request proceeds with that connection

#### Scenario: Return connection to pool

- **WHEN** a request completes
- **THEN** the connection is returned to the pool
- **AND** the connection is available for other requests

#### Scenario: Pool exhaustion handling

- **WHEN** all connections are in use
- **THEN** new requests wait for a configurable timeout
- **AND** a timeout error is returned if no connection becomes available

### Requirement: Health Check Endpoint

The server SHALL expose a health check endpoint that validates database connectivity.

#### Scenario: Database healthy

- **WHEN** GET /health is called and database is reachable
- **THEN** response status is 200 OK
- **AND** response includes {"database": "healthy"}

#### Scenario: Database unhealthy

- **WHEN** GET /health is called and database is unreachable
- **THEN** response status is 503 Service Unavailable
- **AND** response includes {"database": "unhealthy"}
