# Change: Add Core Libraries & Architecture (Phase 1)

## Why

The Altair client needs foundational libraries for dependency injection, navigation, and error handling to enable structured development of features. Without these core patterns established, feature development would be ad-hoc and inconsistent across modules.

## What Changes

- **Version Catalog**: Added Koin 4.x, Decompose 3.x, Arrow 2.x, Mokkery 3.x, KSP, and Turbine to `gradle/libs.versions.toml`
- **Dependency Injection**: Wired Koin into all platform entry points (Android, Desktop, iOS) with proper error handling
- **Navigation**: Set up Decompose with RootComponent and basic destination Config
- **Error Handling**: Added Arrow to shared module with KSP for optics generation; DomainError sealed interface with validation

## Impact

- Affected specs: Creates new `dependency-injection`, `navigation`, `error-handling` capabilities
- Affected code:
  - `gradle/libs.versions.toml` - New versions and library declarations
  - `composeApp/build.gradle.kts` - Added Koin, Decompose, Mokkery dependencies
  - `shared/build.gradle.kts` - Added Arrow dependencies with KSP
  - `composeApp/src/commonMain/` - DI module, navigation components, ErrorScreen
  - `composeApp/src/androidMain/` - Koin initialization in Application class with error handling
  - `composeApp/src/jvmMain/` - Koin initialization in main.kt with error handling
  - `composeApp/src/iosMain/` - Koin initialization helper for Swift
