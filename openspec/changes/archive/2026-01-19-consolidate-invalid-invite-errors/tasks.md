## 1. Remove Duplicate Error Type

- [x] 1.1 Remove `InvalidInvite(val code: String)` from `AuthError.kt`

## 2. Update Server Repository

- [x] 2.1 Update `SurrealInviteCodeRepository.create()` to use `InvalidInviteCode`
- [x] 2.2 Update `SurrealInviteCodeRepository.findByCode()` to use `InvalidInviteCode`
- [x] 2.3 Update `SurrealInviteCodeRepository.markUsed()` to use `InvalidInviteCode`
- [x] 2.4 Update `SurrealInviteCodeRepository.deleteExpiredAndUsed()` to use `InvalidInviteCode`

## 3. Update Shared Module

- [x] 3.1 Update KDoc in `InviteCodeRepository.findByCode()` to reference `InvalidInviteCode`

## 4. Update UI Code

- [x] 4.1 Simplify `RegisterComponent.kt` type check to only use `InvalidInviteCode`

## 5. Update Tests

- [x] 5.1 Remove `InvalidInvite` test case from `ModuleErrorsTest.kt`
- [x] 5.2 Remove `InvalidInvite` from `all DomainError subtypes` test data

## 6. Validation

- [x] 6.1 Run `./gradlew :shared:jvmTest` to verify shared module tests pass
- [x] 6.2 Run `./gradlew :server:test` to verify server tests pass
- [x] 6.3 Run `./gradlew :composeApp:jvmTest` to verify UI tests compile
- [x] 6.4 Run `./gradlew build` to verify full build passes
