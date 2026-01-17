## ADDED Requirements

### Requirement: SQLDelight Schema Definition

The mobile database SHALL define type-safe schemas for all domain entities using SQLDelight .sq files.

#### Scenario: Generate Kotlin code from schema

- **WHEN** the project is built
- **THEN** SQLDelight generates Kotlin query interfaces
- **AND** the generated code is type-safe for all tables

#### Scenario: Schema matches domain model

- **WHEN** a domain entity exists in the shared module
- **THEN** a corresponding SQLDelight table definition exists
- **AND** column types match the entity property types

### Requirement: SQLDelight Driver Configuration

The mobile database SHALL use platform-specific SQLDelight drivers.

#### Scenario: Android driver initialization

- **WHEN** the app runs on Android
- **THEN** the AndroidSqliteDriver is used
- **AND** the database file is stored in the app's internal storage

#### Scenario: iOS driver initialization

- **WHEN** the app runs on iOS
- **THEN** the NativeSqliteDriver is used
- **AND** the database file is stored in the app's documents directory

### Requirement: User Scoping in Mobile Queries

All mobile database queries SHALL filter by the authenticated user's ID.

#### Scenario: Query includes user filter

- **WHEN** a query retrieves user data
- **THEN** the WHERE clause includes user_id = :userId
- **AND** only the current user's data is returned

#### Scenario: Insert includes user ID

- **WHEN** a new record is inserted
- **THEN** the user_id column is populated with the current user's ID

### Requirement: SQLDelight Query Definitions

The mobile database SHALL define named queries for common operations.

#### Scenario: Find by ID query

- **WHEN** a findById named query is defined
- **THEN** it accepts an id parameter and returns a single row or null

#### Scenario: Find all query with ordering

- **WHEN** a findAll named query is defined
- **THEN** it returns all rows for the current user
- **AND** results are ordered by a sensible default (e.g., created_at DESC)

#### Scenario: Parameterized queries

- **WHEN** a query requires filtering
- **THEN** parameters are passed using named parameters (:paramName)
- **AND** SQL injection is prevented by the driver

### Requirement: Mobile Schema Migrations

The mobile database SHALL support schema migrations for version upgrades.

#### Scenario: Apply migration on upgrade

- **WHEN** the app is upgraded with a new schema version
- **THEN** pending migrations are applied automatically
- **AND** existing data is preserved

#### Scenario: Migration files convention

- **WHEN** a schema change is needed
- **THEN** a new .sqm file is created in the migrations directory
- **AND** the file is numbered sequentially (1.sqm, 2.sqm, etc.)

### Requirement: Mobile Database Initialization

The mobile database SHALL be initialized at app startup via Koin.

#### Scenario: Database created on first launch

- **WHEN** the app launches for the first time
- **THEN** the SQLite database file is created
- **AND** all tables are created from the schema

#### Scenario: Koin module provides database

- **WHEN** Koin is initialized
- **THEN** the Database instance is available for injection
- **AND** repository implementations can depend on it

### Requirement: Sync Version Tracking

All mobile database entities SHALL include a sync_version column for synchronization.

#### Scenario: Sync version column exists

- **WHEN** a table is defined
- **THEN** it includes a sync_version INTEGER NOT NULL DEFAULT 0 column

#### Scenario: Track local changes

- **WHEN** a record is created or updated locally
- **THEN** the sync_version is set to indicate pending sync
