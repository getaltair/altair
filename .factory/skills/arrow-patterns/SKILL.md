---
name: arrow-patterns
description: Expert guidance on Arrow functional programming patterns for Kotlin. Use when working with Either, Option, Validated, or any functional error handling scenarios.
---

# Skill: Arrow Functional Programming Patterns

## Purpose

Provide expert guidance on using Arrow functional programming patterns in Kotlin code, with focus on proper error handling using `Either`, `Option`, `Validated`, and other Arrow constructs. Ensure error handling follows functional principles rather than exception-based approaches.

## When to use this skill

Use this skill when:

- **Implementing repository patterns** that can fail (e.g., database operations, network calls)
- **Adding error handling** to operations expected to fail (authentication, validation, I/O)
- **Refactoring exception-based code** to use functional error handling
- **Working with chains of operations** where each step can fail
- **Writing type-safe error flows** in Kotlin multiplatform code

## Inputs

- **Operation context**: What operation is being performed (repository method, API call, etc.)
- **Error types**: Domain-specific error types (e.g., `UserError`, `NetworkError`, `ValidationError`)
- **Success types**: Expected successful return types
- **Existing code**: Code patterns already using Arrow in the codebase

## Conventions

### Altair Project Conventions

Based on the project's coding guidelines:

- **Always use `Either<DomainError, T>`** for operations that can fail
- **Chain operations** with `flatMap`, `map`, and recover with `fold`
- **Never throw exceptions** for expected failures
- Use `either { ... }` builder syntax for clean composition
- Use `.bind()` to unwrap values within `either` blocks

### Arrow Version

The project uses Arrow for type-safe error handling. Common patterns:

**Either for error handling**:
```kotlin
suspend fun getUser(id: UserId): Either<UserError, User> = either {
    val cached = cache.get(id).bind()
    cached ?: api.fetchUser(id).bind().also { cache.store(it) }
}
```

## Required behavior

1. **Use Either for fallible operations**: Any operation that can fail should return `Either<Error, Success>`
2. **Chain dependencies properly**: Use `flatMap` for dependent operations, `map` for transformations
3. **Handle all error cases**: Ensure error types are comprehensive and cover all failure modes
4. **Avoid exception-based flows**: Convert existing exception-based error handling to Either as appropriate

## Required artifacts

- Updated functions returning `Either<Error, T>` where fallible operations exist
- Proper error domain types defined (sealed classes for domain errors)
- Tests covering both success and failure paths

## Implementation checklist

1. Identify operations that can fail and define appropriate error types
2. Convert return types from nullable returns or exceptions to `Either<Error, T>`
3. Update callers to handle `Either` results using `fold`, `getOrElse`, or `.bind()`
4. Add tests for error paths (left branch) and success paths (right branch)

## Verification

The skill is complete when:

- No expected failures use exceptions (use `Either` instead)
- All error types are properly defined as sealed classes
- Error handling preserves type safety across the call chain
- Tests cover both success and failure scenarios for each fallible operation

## Safety and escalation

- If you encounter code where exceptions are the only viable option (e.g., integrations with third-party libraries that throw), document why `Either` cannot be used
- If error types become too complex, consider using `ValidatedNel` for accumulating validation errors instead of short-circuiting on first failure

## Project-specific resources

- `@docs/architecture/` - System architecture and design patterns
- `@CLAUDE.md` - Project coding guidelines and Arrow usage examples
- Search for existing `Either` usage in the codebase with: `rg "Either<" --type kotlin`

## Quick patterns reference

### Basic Either usage
```kotlin
// Creating Either
Right(successValue)
Left(errorValue)

// Transforming
either.eager { ... }.bind()  // Auto-wrapping in either context
eitherResult.map { ... }    // Transform success
eitherResult.flatMap { ... } // Chain operations

// Handling
eitherResult.fold(
    onError = { /* handle error */ },
    onSuccess = { /* handle success */ }
)
```

### Option for nullable-like behavior
```kotlin
val result: Option<User> = user.toOption()

result.onNone { /* handle absence */ }
result.onSome { /* handle presence */ }
```

### Validated for accumulating errors
```kotlin
val result = Validated.zip(
    validateName(name),
    validateEmail(email),
    ::User
)

// result is Validated<Nel<ValidationError>, User>
// Accumulates all validation errors instead of short-circuiting
```
