## ADDED Requirements

### Requirement: SurrealDB Embedded Mode

The desktop application SHALL run SurrealDB in embedded mode using SurrealKV storage engine.

#### Scenario: Start embedded database

- **WHEN** the desktop application starts
- **THEN** SurrealDB is initialized in-process
- **AND** the SurrealKV storage engine is used for persistence

#### Scenario: Database location

- **WHEN** the embedded database is created
- **THEN** data files are stored in $APP_DATA/altair/db/
- **AND** the directory is created if it does not exist

#### Scenario: No external process required

- **WHEN** the desktop application runs
- **THEN** no separate SurrealDB server process is needed
- **AND** all database operations happen in the application process

### Requirement: Desktop Database Initialization

The desktop application SHALL initialize the database at startup via Koin.

#### Scenario: Database available at startup

- **WHEN** the application main() function runs
- **THEN** the embedded SurrealDB is initialized before UI is displayed
- **AND** repository implementations are available for injection

#### Scenario: Graceful shutdown

- **WHEN** the application exits
- **THEN** database connections are closed
- **AND** data is flushed to disk

### Requirement: Shared Repository Implementations

The desktop application SHALL reuse the server's SurrealDB repository implementations.

#### Scenario: Common query logic

- **WHEN** a repository method is called on desktop
- **THEN** the same SurrealQL queries are used as on server
- **AND** only connection configuration differs

#### Scenario: User scoping applies

- **WHEN** a repository query executes
- **THEN** user_id filtering is applied
- **AND** desktop data isolation matches server behavior

### Requirement: Desktop Database Configuration

The desktop application SHALL support configurable database settings.

#### Scenario: Configure data directory

- **WHEN** ALTAIR_DATA_DIR environment variable is set
- **THEN** the database is stored in that directory
- **AND** default is platform-specific app data directory

#### Scenario: Configure memory limits

- **WHEN** ALTAIR_DB_MEMORY_MB is set
- **THEN** the embedded database uses that memory limit
- **AND** default is a reasonable percentage of system RAM

### Requirement: Desktop Graph Query Support

The desktop database SHALL support SurrealDB graph traversal queries.

#### Scenario: Follow relationships

- **WHEN** a query uses graph operators (-> or <-)
- **THEN** related records are traversed
- **AND** results include connected entities

#### Scenario: Cross-entity queries

- **WHEN** a query joins multiple entity types
- **THEN** the graph relationships are resolved
- **AND** results are returned efficiently

### Requirement: Desktop Full-Text Search

The desktop database SHALL support full-text search on text content.

#### Scenario: Search notes by content

- **WHEN** a search query is executed against notes
- **THEN** full-text search indexes are used
- **AND** results are ranked by relevance

#### Scenario: Search across entities

- **WHEN** a global search is executed
- **THEN** multiple entity types are searched
- **AND** results include entity type and match context

### Requirement: Desktop Schema Migrations

The desktop database SHALL apply schema migrations at startup.

#### Scenario: Run migrations on startup

- **WHEN** the application starts with unapplied migrations
- **THEN** migrations are applied in order
- **AND** the user's data is preserved

#### Scenario: Share migrations with server

- **WHEN** a new migration is created
- **THEN** the same migration applies to both server and desktop
- **AND** schema compatibility is maintained
