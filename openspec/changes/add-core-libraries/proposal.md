# Change: Add Core Libraries & Architecture (Phase 1)

## Why

The Altair client needs foundational libraries for dependency injection, navigation, and error handling to enable structured development of features. Without these core patterns established, feature development would be ad-hoc and inconsistent across modules.

## What Changes

- **Version Catalog**: Add Koin 4.x, Decompose 3.x, Arrow 2.x, Mokkery 3.x, and Turbine to `gradle/libs.versions.toml`
- **Dependency Injection**: Wire Koin into all platform entry points (Android, Desktop, iOS)
- **Navigation**: Set up Decompose with RootComponent and basic destination Config
- **Error Handling**: Add Arrow to shared module with KSP for optics generation

## Impact

- Affected specs: Creates new `dependency-injection`, `navigation`, `error-handling` capabilities
- Affected code:
  - `gradle/libs.versions.toml` - New versions and library declarations
  - `composeApp/build.gradle.kts` - Add Koin and Decompose dependencies
  - `shared/build.gradle.kts` - Add Arrow dependencies with KSP
  - `composeApp/src/commonMain/` - New DI module and navigation components
  - `composeApp/src/androidMain/` - Koin initialization in MainActivity
  - `composeApp/src/jvmMain/` - Koin initialization in main.kt
  - `composeApp/src/iosMain/` - Koin initialization hook
