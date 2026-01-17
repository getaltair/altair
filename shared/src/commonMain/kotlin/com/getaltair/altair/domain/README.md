# Domain Layer

This package contains domain models and error types for the Altair application.

## Arrow Error Handling

Arrow is included in this project to enable functional error handling with `Either<DomainError, T>`.

### Why Arrow?

As specified in CLAUDE.md, we use Arrow's `Either` for operations that can fail:
- Type-safe error handling without exceptions
- Composable error handling with `flatMap`, `map`, and `fold`
- Clear API contracts showing all possible failure modes

### Usage Example

```kotlin
import arrow.core.Either
import arrow.core.raise.either

// Repository method returning Either
suspend fun getUserById(id: String): Either<DomainError, User> = either {
    validateUserId(id).bind()  // Short-circuits on Left (error)
    database.query("SELECT * FROM users WHERE id = ?", id)
        .mapLeft { DomainError.NotFoundError("User", id) }
        .bind()
}

// Calling code
when (val result = getUserById("123")) {
    is Either.Left -> handleError(result.value)  // DomainError
    is Either.Right -> displayUser(result.value) // User
}
```

### Arrow Dependencies

The following Arrow libraries are configured in `shared/build.gradle.kts`:
- `arrow-core`: Core functional types (Either, Option, etc.)
- `arrow-optics`: Lens-based data manipulation (for future use)
- `arrow-optics-ksp`: KSP plugin for generating optics

### DomainError Hierarchy

All domain errors implement the `DomainError` sealed interface with:
- **Validation**: All required fields are validated to be non-blank
- **toUserMessage()**: User-friendly messages for UI display
- **Pattern matching**: Exhaustive `when` expressions for error handling

Available error types:
- `NetworkError` - Connectivity and timeout issues
- `ValidationError` - Invalid input or constraints
- `NotFoundError` - Resource not found
- `UnauthorizedError` - Access denied
- `UnexpectedError` - Generic fallback (use sparingly)

### When to Use Either

**Use Either for:**
- Network calls that can fail
- Database operations that might not find data
- Validation that can reject input
- Any operation with expected failure modes

**Don't use Either for:**
- Programming errors (bugs) - use require/check/assert
- Truly unexpected errors - let exceptions propagate
- Simple null checks - use nullable types

### Next Steps

As features are implemented:
1. Define specific error types extending `DomainError`
2. Create repository interfaces returning `Either<DomainError, T>`
3. Use `Either` in ViewModels to handle success/error states
4. Consider adding `Validated` for accumulating validation errors
