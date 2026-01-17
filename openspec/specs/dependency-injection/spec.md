# dependency-injection Specification

## Purpose
TBD - created by archiving change add-core-libraries. Update Purpose after archive.
## Requirements
### Requirement: Koin Dependency Injection Container

The system SHALL use Koin as the dependency injection container across all platforms (Android, iOS, Desktop).

#### Scenario: Application startup initializes Koin

- **WHEN** the application starts on any platform
- **THEN** the Koin container is initialized before any UI is rendered
- **AND** all registered dependencies are available for injection

#### Scenario: Common modules shared across platforms

- **WHEN** dependencies are registered in commonMain modules
- **THEN** those dependencies are available on all platform targets
- **AND** platform-specific overrides can be provided in platform source sets

### Requirement: Platform Entry Point Initialization

Each platform entry point SHALL initialize Koin with the application modules.

#### Scenario: Android initialization in Application class

- **WHEN** the Android application launches
- **THEN** Koin is started in Application.onCreate before any Activity is created
- **AND** the Android context is available in the Koin module
- **AND** initialization errors are logged before propagating

#### Scenario: Desktop initialization in main function

- **WHEN** the Desktop application launches
- **THEN** Koin is started in the main function before Window creation
- **AND** initialization errors display the ErrorScreen instead of crashing

#### Scenario: iOS initialization via helper function

- **WHEN** the iOS application launches
- **THEN** a Kotlin helper function is available for Swift to call
- **AND** Koin is initialized before the Compose view controller is created
- **AND** the helper returns an error message on failure, null on success

### Requirement: Idempotent Initialization

The initKoin() function SHALL be safe to call multiple times.

#### Scenario: Multiple initialization calls

- **WHEN** initKoin() is called when Koin is already running
- **THEN** the function returns false without throwing an exception
- **AND** the existing Koin instance is preserved

### Requirement: Modular Dependency Organization

Dependencies SHALL be organized into logical modules that can be composed together.

#### Scenario: App module as composition root

- **WHEN** the application starts
- **THEN** the AppModule aggregates all feature modules
- **AND** feature modules can declare their own dependencies independently

#### Scenario: Test module isolation

- **WHEN** running tests
- **THEN** individual modules can be loaded in isolation
- **AND** Koin's checkModules() validates all dependencies resolve correctly

