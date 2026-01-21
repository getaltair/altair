# Instant Type Usage

## Standard: Use `kotlin.time.Instant`

This project uses `kotlin.time.Instant` from the Kotlin standard library (introduced in Kotlin 2.1.20).

**DO NOT** use:
- `kotlinx.datetime.Instant` 
- `typealias Instant = kotlinx.datetime.Instant` or similar aliases

## Why

1. `kotlin.time.Instant` is now part of the Kotlin stdlib and is the canonical Instant type
2. `kotlinx.serialization` 1.9.0+ provides built-in serialization support
3. `kotlinx-datetime` 0.7.1+ provides type aliases for migration compatibility

## Imports

```kotlin
// CORRECT
import kotlin.time.Instant
import kotlin.time.Clock

// INCORRECT - do not use
import kotlinx.datetime.Instant
import kotlinx.datetime.Clock
```

## Serialization

`kotlin.time.Instant` is serializable out of the box with kotlinx.serialization 1.9.0+. No custom serializer is needed.

## Clock Usage

For getting the current time:

```kotlin
import kotlin.time.Clock

val now: Instant = Clock.System.now()
```
