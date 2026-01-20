# Testing

Testing infrastructure and standards for the Altair platform.

## ADDED Requirements

### Requirement: BehaviorSpec as Primary Style

Tests SHALL use BehaviorSpec with given/when/then structure as the primary testing style for behavior-driven test organization.

#### Scenario: Domain validation test uses BehaviorSpec

- **GIVEN** a test for domain model validation
- **WHEN** the test class is created
- **THEN** it extends `BehaviorSpec`
- **AND** tests use `given`, `when`, `then` blocks to separate arrangement, action, and assertion

#### Scenario: Repository test uses BehaviorSpec

- **GIVEN** a test for repository operations
- **WHEN** testing CRUD or query operations
- **THEN** tests use `given` to describe preconditions (e.g., "an empty repository")
- **AND** `when` describes the action (e.g., "saving a new quest")
- **AND** `then` describes the expected outcome

#### Scenario: DescribeSpec for API organization

- **GIVEN** a test for a component with multiple methods or behaviors
- **WHEN** the test class is created
- **THEN** it extends `DescribeSpec`
- **AND** tests are organized using `describe` blocks per method/feature and `it` blocks per behavior

### Requirement: Property-Based Testing for Domain Models

Domain model validation SHALL use property-based testing with Kotest generators for comprehensive input coverage.

#### Scenario: Valid input ranges tested with property tests

- **GIVEN** a domain entity with validation constraints
- **WHEN** testing valid input acceptance
- **THEN** tests use `checkAll` with appropriate `Arb` generators
- **AND** generators produce thousands of valid inputs within constraints

#### Scenario: Invalid input rejection tested with property tests

- **GIVEN** a domain entity with validation constraints
- **WHEN** testing invalid input rejection
- **THEN** tests use `checkAll` with generators for invalid inputs
- **AND** all generated invalid inputs cause validation failure

#### Scenario: Custom generators for domain types

- **GIVEN** domain types like `Ulid`, `Quest`, or `User`
- **WHEN** property tests need instances of these types
- **THEN** shared `Arb` generators in `TestGenerators.kt` provide type-safe generation
- **AND** generators respect domain invariants (e.g., `energyCost` between 1-5)

### Requirement: Data-Driven Testing for Edge Cases

Tests with enumerated cases or boundary conditions SHALL use data-driven testing with `withData` for systematic coverage.

#### Scenario: Status transitions tested with data-driven approach

- **GIVEN** a state machine with defined transitions (e.g., QuestStatus)
- **WHEN** testing transition validity
- **THEN** tests use `withData` with all transition combinations
- **AND** each combination is a separate test case with clear pass/fail

#### Scenario: Validation boundaries tested with data-driven approach

- **GIVEN** an entity with boundary constraints (e.g., title max 200 chars)
- **WHEN** testing boundary behavior
- **THEN** tests use `withData` with boundary values (199, 200, 201)
- **AND** each boundary is explicitly tested

### Requirement: Kotest Assertions

All test assertions SHALL use Kotest matchers exclusively for consistent failure messages and composability.

#### Scenario: Equality uses shouldBe

- **GIVEN** a test verifying equality
- **WHEN** comparing actual and expected values
- **THEN** the test uses `actual shouldBe expected` syntax

#### Scenario: Arrow Either uses Kotest Arrow matchers

- **GIVEN** a test verifying an Either result
- **WHEN** checking success or failure
- **THEN** the test uses `result.shouldBeRight()` or `result.shouldBeLeft()`
- **AND** nested assertions use the lambda form `result.shouldBeRight { it.id shouldBe expected }`

#### Scenario: Multiple assertions use assertSoftly

- **GIVEN** a test with multiple related assertions
- **WHEN** all assertions should be checked regardless of earlier failures
- **THEN** assertions are wrapped in `assertSoftly { }`
- **AND** all failures are reported together

### Requirement: Strict Project Configuration

Each module SHALL have a ProjectConfig with strict settings to enforce test quality.

#### Scenario: Tests without assertions fail

- **GIVEN** a test that executes code but makes no assertions
- **WHEN** the test suite runs
- **THEN** the test fails due to `assertionMode = AssertionMode.Error`

#### Scenario: Coroutine tests use test dispatcher

- **GIVEN** a test that exercises coroutine code
- **WHEN** the test runs
- **THEN** it uses a `TestDispatcher` via `coroutineTestScope = true`
- **AND** time can be controlled programmatically

#### Scenario: Tests have timeout enforcement

- **GIVEN** a test that takes too long
- **WHEN** the test exceeds the configured timeout
- **THEN** the test fails with a timeout error

### Requirement: Testcontainers Kotest Extension

Server integration tests SHALL use Kotest extensions for container lifecycle management.

#### Scenario: Container starts with spec lifecycle

- **GIVEN** an integration test requiring SurrealDB
- **WHEN** the spec begins execution
- **THEN** the container is started via `install(ContainerExtension(...))`
- **AND** the container is available for all tests

#### Scenario: Container cleaned up automatically

- **GIVEN** an integration test spec has completed
- **WHEN** all tests have finished
- **THEN** the container is stopped and resources released automatically
- **AND** no manual cleanup code is required

### Requirement: Test Organization Structure

Tests SHALL mirror source structure with descriptive naming conventions.

#### Scenario: Test packages mirror source packages

- **GIVEN** a source file at `domain/model/Quest.kt`
- **WHEN** creating tests for that file
- **THEN** tests are placed at `domain/model/QuestValidationTest.kt`
- **AND** the package structure matches

#### Scenario: Test names describe behavior

- **GIVEN** a BehaviorSpec test
- **WHEN** naming given/when/then blocks
- **THEN** `given` describes the precondition state
- **AND** `when` describes the action being tested
- **AND** `then` describes the expected outcome

### Requirement: Property Test Reproducibility

Property test failures SHALL be reproducible via seed logging.

#### Scenario: Failing property test logs seed

- **GIVEN** a property test that fails
- **WHEN** examining the test output
- **THEN** the output includes the seed value used for generation
- **AND** the seed can be used to reproduce the exact failure

#### Scenario: Seed can be fixed for debugging

- **GIVEN** a failing seed value from test output
- **WHEN** investigating the failure
- **THEN** the test can be run with `PropTestConfig(seed = X)` to reproduce
- **AND** the same inputs are generated deterministically
