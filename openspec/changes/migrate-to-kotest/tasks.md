# Tasks: Migrate to Kotest

## 1. Infrastructure Setup

- [ ] 1.1 Add `kotest-runner-junit5` to version catalog
- [ ] 1.2 Add `kotest-extensions-testcontainers` to version catalog
- [ ] 1.3 Add `kotest-assertions-arrow` to version catalog for Either matchers
- [ ] 1.4 Update `shared/build.gradle.kts`:
  - Add JUnit runner for jvmTest
  - Configure `useJUnitPlatform()`
- [ ] 1.5 Update `composeApp/build.gradle.kts`:
  - Add all Kotest dependencies to commonTest
  - Add JUnit runner for jvmTest
  - Configure `useJUnitPlatform()`
- [ ] 1.6 Update `server/build.gradle.kts`:
  - Replace JUnit 5 with Kotest runner
  - Add Testcontainers extension
  - Remove direct JUnit dependencies
- [ ] 1.7 Create `ProjectConfig.kt` in `shared/src/commonTest/`:
  - Enable `assertionMode = AssertionMode.Error`
  - Enable `coroutineTestScope = true`
  - Set `timeout = 30.seconds`
  - Set `isolationMode = IsolationMode.InstancePerLeaf`
- [ ] 1.8 Create `ProjectConfig.kt` in `composeApp/src/jvmTest/`
- [ ] 1.9 Create `ProjectConfig.kt` in `server/src/test/`
- [ ] 1.10 Create `TestGenerators.kt` in `shared/src/commonTest/` with domain Arb generators

## 2. Shared Module - Domain Model Tests

### Validation Tests (BehaviorSpec + Property Testing)

- [ ] 2.1 Rewrite `EntityValidationTest.kt` as BehaviorSpec:
  - Group by entity type (User, Quest, Initiative, etc.)
  - Use property tests for valid input ranges
  - Use data-driven tests for invalid input edge cases
- [ ] 2.2 Rewrite `PropertyBasedValidationTest.kt`:
  - Expand property test coverage
  - Add custom generators for all domain types
  - Test invariants across entity lifecycle
- [ ] 2.3 Rewrite `TemporalInvariantsTest.kt` as BehaviorSpec:
  - Property tests for timestamp ordering
  - Edge cases for timezone handling

### Type Tests (FunSpec + Property Testing)

- [ ] 2.4 Rewrite `UlidTest.kt` with property tests:
  - Uniqueness property
  - Lexicographic ordering property
  - Serialization round-trip property
- [ ] 2.5 Rewrite `ScheduleTest.kt` as BehaviorSpec:
  - Test each schedule type (Daily, Weekly, etc.)
  - Property tests for next occurrence calculation

### Serialization Tests (DescribeSpec + Data-Driven)

- [ ] 2.6 Rewrite `EntitySerializationTest.kt` as DescribeSpec:
  - Organize by entity type
  - Data-driven round-trip tests
  - Edge case handling (null fields, max lengths)
- [ ] 2.7 Rewrite `DtoSerializationTest.kt` as DescribeSpec:
  - Group by DTO type
  - Verify JSON structure matches API contract

### Error Handling Tests (BehaviorSpec)

- [ ] 2.8 Rewrite `DomainErrorTest.kt` as BehaviorSpec:
  - Test error hierarchy
  - Test error message formatting
- [ ] 2.9 Rewrite `ModuleErrorsTest.kt` as BehaviorSpec:
  - Test module-specific error types
  - Test error conversion/mapping

### Repository Tests (BehaviorSpec)

- [ ] 2.10 Rewrite `EpicProgressTest.kt` as BehaviorSpec:
  - Given/when/then for progress calculations
  - Property tests for percentage bounds
- [ ] 2.11 Rewrite `PaginationTest.kt` as BehaviorSpec:
  - Test cursor-based pagination
  - Edge cases (empty results, last page)

### Auth Tests (BehaviorSpec)

- [ ] 2.12 Rewrite `AuthManagerTest.kt` as BehaviorSpec:
  - Test authentication flows
  - Test token refresh scenarios
  - Test session state transitions

### Platform-Specific Tests

- [ ] 2.13 Rewrite `SqlDelightDatabaseTest.kt` (jvmTest) as BehaviorSpec
- [ ] 2.14 Rewrite `CredentialStoreFallbackTest.kt` (jvmTest) as DescribeSpec
- [ ] 2.15 Rewrite `NativeCredentialStoreFactoryTest.kt` (jvmTest) as BehaviorSpec
- [ ] 2.16 Rewrite `DesktopSecureTokenStorageTest.kt` (jvmTest) as BehaviorSpec
- [ ] 2.17 Rewrite `NativeSecureTokenStorageTest.kt` (jvmTest) as BehaviorSpec
- [ ] 2.18 Rewrite `NativeCredentialStoreIntegrationTest.kt` (jvmTest) as BehaviorSpec
- [ ] 2.19 Rewrite `IosSecureTokenStorageTest.kt` (iosTest) as BehaviorSpec

## 3. ComposeApp Module Tests

### Navigation Tests (BehaviorSpec)

- [ ] 3.1 Rewrite `MainComponentTest.kt` as BehaviorSpec:
  - Test navigation state transitions
  - Test deep linking scenarios
- [ ] 3.2 Rewrite `RootComponentTest.kt` as BehaviorSpec:
  - Test root component initialization
  - Test child component creation

### Auth Component Tests (BehaviorSpec)

- [ ] 3.3 Rewrite `LoginComponentTest.kt` as BehaviorSpec:
  - Test login flow scenarios
  - Test validation feedback
  - Test error handling
- [ ] 3.4 Rewrite `RegisterComponentTest.kt` as BehaviorSpec:
  - Test registration flow
  - Test input validation
  - Test success/failure scenarios

### Infrastructure Tests

- [ ] 3.5 Rewrite `KoinCheckTest.kt` as FunSpec:
  - Verify DI configuration
- [ ] 3.6 Rewrite `DesktopMigrationRunnerTest.kt` as BehaviorSpec
- [ ] 3.7 Rewrite `EmbeddedSurrealClientTest.kt` as BehaviorSpec
- [ ] 3.8 Rewrite `WindowSizeClassTest.kt` as DescribeSpec with data-driven tests

### Android Instrumented Tests (Limited Kotest Support)

- [ ] 3.9 Update `AndroidSecureTokenStorageTest.kt`:
  - Keep `@Test` annotations (Android Test Runner constraint)
  - Replace assertions with Kotest matchers

## 4. Server Module Tests

### Testcontainers Infrastructure

- [ ] 4.1 Create `SurrealDbContainerExtension.kt`:
  - Implement Kotest `MountableExtension` for SurrealDB
  - Handle container lifecycle
  - Provide connection configuration
- [ ] 4.2 Update `SurrealDbTestContainer.kt` for extension compatibility

### Unit Tests

- [ ] 4.3 Rewrite `ApplicationTest.kt` as BehaviorSpec:
  - Test server startup scenarios
  - Test configuration loading
- [ ] 4.4 Rewrite `Argon2PasswordServiceTest.kt` as DescribeSpec:
  - Test password hashing
  - Test password verification
  - Property tests for hash uniqueness
- [ ] 4.5 Rewrite `JwtTokenServiceImplTest.kt` as DescribeSpec:
  - Test token generation
  - Test token validation
  - Test expiration handling
  - Edge cases (invalid tokens, tampered tokens)

### Repository Integration Tests (BehaviorSpec)

- [ ] 4.6 Rewrite `SurrealDbClientTest.kt` as BehaviorSpec:
  - Test connection lifecycle
  - Test query execution
- [ ] 4.7 Rewrite `MigrationRunnerTest.kt` as BehaviorSpec:
  - Test migration execution order
  - Test idempotency
- [ ] 4.8 Rewrite `UserScopeIntegrationTest.kt` as BehaviorSpec:
  - Test user isolation
  - Test cross-user access prevention
- [ ] 4.9 Rewrite `SurrealContainerRepositoryTest.kt` as BehaviorSpec:
  - CRUD operations
  - Hierarchy queries
- [ ] 4.10 Rewrite `SurrealQuestRepositoryTest.kt` as BehaviorSpec:
  - CRUD operations
  - Status transitions with data-driven tests
  - WIP limit enforcement
- [ ] 4.11 Rewrite `SurrealUserRepositoryTest.kt` as BehaviorSpec
- [ ] 4.12 Rewrite `SurrealInviteCodeRepositoryTest.kt` as BehaviorSpec
- [ ] 4.13 Rewrite `SurrealNoteRepositoryTest.kt` as BehaviorSpec
- [ ] 4.14 Rewrite `SurrealRefreshTokenRepositoryTest.kt` as BehaviorSpec
- [ ] 4.15 Rewrite `SurrealInitiativeRepositoryTest.kt` as BehaviorSpec

### RPC Integration Tests (BehaviorSpec)

- [ ] 4.16 Rewrite `RpcIntegrationTest.kt` as BehaviorSpec:
  - Test RPC protocol
  - Test serialization
  - Test error propagation
- [ ] 4.17 Rewrite `AuthIntegrationTest.kt` as BehaviorSpec:
  - Test full auth flow
  - Test token refresh
  - Test unauthorized access

## 5. Verification & Documentation

- [ ] 5.1 Run `./gradlew allTests` - verify all tests pass
- [ ] 5.2 Run `./gradlew :shared:iosSimulatorArm64Test` - verify iOS tests
- [ ] 5.3 Run `./gradlew :composeApp:connectedAndroidTest` - verify Android instrumented tests
- [ ] 5.4 Verify property test seed logging in CI output
- [ ] 5.5 Remove unused kotlin-test and JUnit dependencies
- [ ] 5.6 Update `openspec/project.md` Testing Strategy section:
  - Document Kotest as primary framework
  - Document spec style guidelines
  - Document property testing standards
- [ ] 5.7 Update `CLAUDE.md` with testing guidelines:
  - Reference Kotest skill
  - Document assertion patterns
  - Document test organization

## Reference

Use the Kotest skill (`.claude/skills/kotest/`) for:
- BehaviorSpec patterns: `references/framework.md`
- Matcher syntax: `references/assertions.md`
- Property testing with `Arb`: `references/proptest.md`
- Testcontainers extension: `references/extensions.md`
- Data-driven testing: `references/framework.md` (search for `withData`)
