---
paths:
  - "apps/android/**"
  - "**/*.kt"
  - "**/*.kts"
---

# Android/Kotlin Conventions

## Language
- Kotlin 2.0+ with K2 compiler
- Target SDK: latest stable
- Compose for UI (if using Jetpack Compose)
- Coroutines for all async operations

## Code Style
- Formatter: ktlint
- Linter: detekt
- Naming: camelCase for functions/properties, PascalCase for classes, SCREAMING_SNAKE for constants
- Prefer `val` over `var` — mutability must be justified
- Use data classes for pure data, sealed classes for state hierarchies

## Coroutines
- Structured concurrency: never use `GlobalScope`
- `viewModelScope` in ViewModels, `lifecycleScope` in Activities/Fragments
- `CoroutineExceptionHandler` at scope boundaries
- Always catch `CancellationException` separately or don't catch `Exception` broadly

## Android Patterns
- ViewModels for UI state, Activities/Fragments for navigation
- Dependency injection with Hilt or Koin (specify which is used)
- LiveData or StateFlow for reactive data
- Repository pattern for data layer
- Use WorkManager for background tasks
- Room for local database (if applicable)

## Testing
- Framework: JUnit5 + MockK
- Test files: `*[Test].kt` in `test/` (unit) and `androidTest/` (instrumented)
- Use `@ExperimentalCoroutinesApi` for testing coroutines
- Mock external dependencies — don't hit real APIs in tests

## Error Handling
- Sealed class hierarchies for domain errors
- `Result<T>` wrapper or similar for operations that can fail predictably
- Never swallow exceptions silently
- Use `require()` / `check()` for preconditions
- Log errors with proper context using Timber or Log
