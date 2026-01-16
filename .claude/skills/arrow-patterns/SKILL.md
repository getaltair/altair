---
name: arrow-patterns
description: Arrow functional programming patterns for Kotlin. Invoke when working with Either, Option, Validated, or functional error handling.
---

# Arrow Patterns

## Error Handling with Either

- Use `Either<DomainError, Success>` for operations that can fail
- Chain with `flatMap`, `map`, recover with `fold`
- Never throw exceptions for expected failures

## Example: Repository pattern

```kotlin
suspend fun getUser(id: UserId): Either<UserError, User> =
    either {
        val cached = cache.get(id).bind()
        cached ?: api.fetchUser(id).bind().also { cache.store(it) }
    }
```
