# error-handling Specification

## Purpose
TBD - created by archiving change add-core-libraries. Update Purpose after archive.
## Requirements
### Requirement: Typed Error Handling with Arrow Either

The system SHALL use Arrow's Either type for operations that can fail, providing compile-time error type safety.

#### Scenario: Repository methods return Either

- **WHEN** a repository method can fail
- **THEN** it returns `Either<DomainError, T>` instead of throwing exceptions
- **AND** callers must handle both success and failure cases

#### Scenario: Error types are domain-specific

- **WHEN** defining errors for a domain operation
- **THEN** errors are modeled as sealed interfaces/classes
- **AND** each error type carries relevant context (e.g., NotFound includes the ID)

### Requirement: Arrow Optics for Immutable State Updates

The system SHALL use Arrow Optics for updating nested immutable data structures in the shared module.

#### Scenario: Optics annotation generates lenses

- **WHEN** a data class is annotated with @optics
- **THEN** KSP generates lens accessors for each property
- **AND** nested updates can be performed without manual copy() chains

#### Scenario: State updates preserve immutability

- **WHEN** updating a deeply nested property using optics
- **THEN** a new copy of the entire structure is created
- **AND** the original structure is not modified

### Requirement: Error Accumulation for Validation

The system SHALL support accumulating multiple validation errors using Arrow's validation combinators.

#### Scenario: Form validation collects all errors

- **WHEN** validating a form with multiple fields
- **THEN** all validation errors are collected (not just the first)
- **AND** the user sees all issues at once

#### Scenario: zipOrAccumulate combines validations

- **WHEN** multiple fields must be validated together
- **THEN** zipOrAccumulate runs all validations
- **AND** returns either all errors or the valid result

### Requirement: CancellationException Safety

Arrow Either operations SHALL not catch CancellationException, preserving coroutine cancellation semantics.

#### Scenario: Cancelled coroutine propagates correctly

- **WHEN** a coroutine is cancelled during an Either operation
- **THEN** CancellationException propagates up the call stack
- **AND** the coroutine is properly cancelled (not converted to Left)

