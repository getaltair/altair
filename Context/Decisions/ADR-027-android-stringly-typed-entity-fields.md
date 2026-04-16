# ADR-027: Stringly-Typed Status/Enum Fields on Android Entities

| Field | Value |
|---|---|
| **Status** | Accepted with follow-up |
| **Date** | 2026-04-16 |
| **Feature** | 009-AndroidClient |

## Context

All 20 Android entities store enumerated domain concepts (quest status, routine frequency, shopping list item status, membership role, etc.) as raw `String` fields. The domain spec defines exact state machines for these values (e.g., `not_started → in_progress → completed / cancelled / deferred`), but these constraints are invisible in the type system.

A misspelled string literal in any ViewModel SQL or mapper is undetectable at compile time. The existing `EntityType` and `RelationType` enums in `Dtos.kt` demonstrate the correct pattern.

The complication: PowerSync writes raw SQL to Room entities and entities must round-trip through PowerSync cursors. Room `@TypeConverter` annotations apply to Room DAO operations but not to PowerSync's `db.execute()` / `db.watch()` paths. Applying an enum `@TypeConverter` that only covers Room means the PowerSync path writes the raw string and Room reads it through a converter — this can work, but only if the string values exactly match the enum ordinal or name.

## Decision

Accept stringly-typed fields on entity classes for the current feature. Introduce **enum companion objects in the domain layer** (not on entities) as the single source of truth for valid status strings.

Concretely:
- Entity fields remain `String` (e.g., `QuestEntity.status: String`)
- Add domain-layer sealed classes or enum companion objects (e.g., `QuestStatus`) with `const val` string constants
- All ViewModel SQL strings and mapper code must reference these constants, not inline string literals
- A Detekt custom rule or `@StringDef` annotation can enforce this at the call site

## Rationale

- Changing entity fields to Kotlin enums requires TypeConverters that only work on the Room path, not the PowerSync path — a partial solution that creates two incompatible code paths
- Enum companion objects provide compile-time safety where it matters most (ViewModel write operations) without touching the entity layer
- This is a cross-cutting change affecting 20 entities; phasing it through a domain-layer abstraction is lower risk

## Consequences

- String entities remain, meaning PowerSync cursor compatibility is preserved
- A follow-up task (backlog) adds domain-layer enum companions for all status fields
- ViewModel SQL strings that currently inline status literals are flagged as technical debt until the constants are introduced

## Follow-up

See backlog for: "Introduce domain-layer enum companions for all stringly-typed entity status fields (20 entities)."
