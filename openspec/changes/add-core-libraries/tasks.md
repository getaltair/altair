## 1. Version Catalog Updates

- [ ] 1.1 Add Koin 4.x versions and libraries (koin-core, koin-compose, koin-compose-viewmodel)
- [ ] 1.2 Add Decompose 3.x versions and libraries (decompose, decompose-extensions-compose)
- [ ] 1.3 Add Arrow 2.x versions and libraries (arrow-core, arrow-optics)
- [ ] 1.4 Add Arrow Optics KSP plugin
- [ ] 1.5 Add Mokkery 3.x version and plugin
- [ ] 1.6 Add Turbine version and library

## 2. Koin Dependency Injection

- [ ] 2.1 Add Koin dependencies to composeApp/build.gradle.kts (commonMain, androidMain)
- [ ] 2.2 Create `di/AppModule.kt` in commonMain with empty module
- [ ] 2.3 Create `di/KoinInitializer.kt` in commonMain with startKoin helper
- [ ] 2.4 Initialize Koin in Android MainActivity
- [ ] 2.5 Initialize Koin in Desktop main.kt
- [ ] 2.6 Create iOS Koin initialization helper (callable from Swift)

## 3. Decompose Navigation

- [ ] 3.1 Add Decompose dependencies to composeApp/build.gradle.kts
- [ ] 3.2 Create `navigation/Config.kt` sealed class with Home destination
- [ ] 3.3 Create `navigation/RootComponent.kt` with childStack
- [ ] 3.4 Create `navigation/RootContent.kt` Composable to render stack
- [ ] 3.5 Wire RootComponent into Android entry point with ComponentContext
- [ ] 3.6 Wire RootComponent into Desktop entry point
- [ ] 3.7 Wire RootComponent into iOS entry point

## 4. Arrow Error Handling

- [ ] 4.1 Add KSP plugin to shared/build.gradle.kts
- [ ] 4.2 Add Arrow dependencies to shared/build.gradle.kts (arrow-core, arrow-optics, arrow-optics-ksp-plugin)
- [ ] 4.3 Verify shared module builds successfully on all targets

## 5. Testing Infrastructure

- [ ] 5.1 Add Mokkery plugin to shared/build.gradle.kts and composeApp/build.gradle.kts
- [ ] 5.2 Add Turbine dependency to commonTest source sets
- [ ] 5.3 Add Koin checkModules() test to verify DI configuration

## 6. Verification

- [ ] 6.1 Run full build on all platforms (./gradlew build)
- [ ] 6.2 Run Android emulator to verify app launches with Koin
- [ ] 6.3 Run Desktop app to verify Decompose navigation works
- [ ] 6.4 Build iOS framework to verify compilation
