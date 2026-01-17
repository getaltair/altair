# Change: Implement Hybrid Database Layer

## Why

Altair requires persistent storage to be usable. The shared module defines repository interfaces but has no implementations. Server needs SurrealDB for primary storage and sync hub. Mobile needs SQLite for quick capture and offline support. Desktop needs embedded SurrealDB for full-featured offline operation with graph queries and vector search.

## What Changes

- Add `surrealdb-java` dependency to server module for SurrealDB connectivity
- Add SQLDelight plugin and dependencies to shared module for mobile SQLite
- Add `surrealdb-java` dependency to composeApp/jvmMain for desktop embedded SurrealDB
- Create `DatabaseConfig` for server connection management
- Implement all 18 repository interfaces with SurrealDB on server
- Create SQLDelight `.sq` schema files mirroring domain entities
- Configure SQLDelight drivers for Android and iOS
- Add SurrealDB embedded initialization for desktop
- Create database migration infrastructure for both engines

## Impact

- Affected specs: `repositories` (implementations added)
- New specs: `database-server`, `database-mobile`, `database-desktop`
- Affected code:
  - `gradle/libs.versions.toml` (new dependencies)
  - `server/build.gradle.kts` (surrealdb-java)
  - `server/src/main/kotlin/.../db/` (new)
  - `shared/build.gradle.kts` (SQLDelight)
  - `shared/src/commonMain/sqldelight/` (new .sq files)
  - `shared/src/androidMain/` (SQLDelight driver)
  - `shared/src/iosMain/` (SQLDelight driver)
  - `composeApp/build.gradle.kts` (SurrealDB for desktop)
  - `composeApp/src/jvmMain/kotlin/.../db/` (new)
