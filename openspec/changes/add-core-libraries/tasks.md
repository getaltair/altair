## 1. Version Catalog Updates

- [x] 1.1 Add Koin 4.x versions and libraries (koin-core, koin-compose, koin-compose-viewmodel, koin-android, koin-test)
- [x] 1.2 Add Decompose 3.x versions and libraries (decompose, decompose-extensions-compose)
- [x] 1.3 Add Arrow 2.x versions and libraries (arrow-core, arrow-optics)
- [x] 1.4 Add Arrow Optics KSP plugin
- [x] 1.5 Add Mokkery 3.x version and plugin
- [x] 1.6 Add Turbine version and library
- [x] 1.7 Add KSP plugin

## 2. Koin Dependency Injection

- [x] 2.1 Add Koin dependencies to composeApp/build.gradle.kts (commonMain, androidMain)
- [x] 2.2 Create `di/AppModule.kt` in commonMain with empty module
- [x] 2.3 Create `di/KoinInitializer.kt` in commonMain with startKoin helper (idempotent)
- [x] 2.4 Initialize Koin in Android Application class with error handling
- [x] 2.5 Initialize Koin in Desktop main.kt with error handling
- [x] 2.6 Create iOS Koin initialization helper (callable from Swift)

## 3. Decompose Navigation

- [x] 3.1 Add Decompose dependencies to composeApp/build.gradle.kts
- [x] 3.2 Create `navigation/Config.kt` sealed class with Home destination
- [x] 3.3 Create `navigation/RootComponent.kt` with childStack
- [x] 3.4 Create `navigation/RootContent.kt` Composable to render stack
- [x] 3.5 Create platform-specific ComponentContextFactory (Android, iOS, Desktop)
- [x] 3.6 Add getLifecycle() extension for Desktop lifecycle management

## 4. Arrow Error Handling

- [x] 4.1 Add KSP plugin to shared/build.gradle.kts
- [x] 4.2 Add Arrow dependencies to shared/build.gradle.kts (arrow-core, arrow-optics, arrow-optics-ksp)
- [x] 4.3 Create DomainError sealed interface with validation
- [x] 4.4 Add toUserMessage() for user-friendly error display

## 5. Testing Infrastructure

- [x] 5.1 Add Mokkery plugin to shared/build.gradle.kts and composeApp/build.gradle.kts
- [x] 5.2 Add Turbine dependency to commonTest source sets
- [x] 5.3 Add Koin checkModules() test to verify DI configuration
- [x] 5.4 Add initKoin() tests (idempotent behavior, custom config)
- [x] 5.5 Add ComponentContextFactory tests (getLifecycle, lifecycle state)
- [x] 5.6 Add DomainError validation tests and toUserMessage tests

## 6. Error Handling & UI

- [x] 6.1 Create ErrorScreen composable for startup failures
- [x] 6.2 Add error handling to all platform entry points
- [x] 6.3 Add logging to error paths

## 7. Verification

- [ ] 7.1 Run full build on all platforms (./gradlew build)
- [ ] 7.2 Run Android emulator to verify app launches with Koin
- [ ] 7.3 Run Desktop app to verify Decompose navigation works
- [ ] 7.4 Build iOS framework to verify compilation
