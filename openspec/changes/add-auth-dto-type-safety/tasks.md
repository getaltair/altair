## 1. Update AuthResponse DTO

- [ ] 1.1 Update `AuthResponse.userId` from `String` to `Ulid` in `AuthDtos.kt`
- [ ] 1.2 Update `AuthResponse.role` from `String` to `UserRole` in `AuthDtos.kt`
- [ ] 1.3 Update KDoc comments to reflect typed fields

## 2. Update Server Implementation

- [ ] 2.1 Update `PublicAuthServiceImpl.login()` to pass `user.id` directly (already `Ulid`)
- [ ] 2.2 Update `PublicAuthServiceImpl.login()` to pass `user.role` directly (already `UserRole`)
- [ ] 2.3 Update `PublicAuthServiceImpl.register()` similarly
- [ ] 2.4 Verify RPC serialization works (types are already `@Serializable`)

## 3. Update SecureTokenStorage Interface

- [ ] 3.1 Update `saveUserId(userId: String)` to `saveUserId(userId: Ulid)` in interface
- [ ] 3.2 Update `getUserId(): String?` to `getUserId(): Ulid?` in interface
- [ ] 3.3 Update KDoc comments

## 4. Update Platform-Specific Implementations

- [ ] 4.1 Update `AndroidSecureTokenStorage.saveUserId()` to convert `Ulid.value` for storage
- [ ] 4.2 Update `AndroidSecureTokenStorage.getUserId()` to parse string as `Ulid`
- [ ] 4.3 Update `IosSecureTokenStorage` similarly
- [ ] 4.4 Update `DesktopSecureTokenStorage` similarly
- [ ] 4.5 Update `NativeSecureTokenStorage` similarly (if separate)

## 5. Update AuthManager

- [ ] 5.1 Update `AuthState.Authenticated.userId` from `String` to `Ulid`
- [ ] 5.2 Update `login()` return type from `Either<AuthError, String>` to `Either<AuthError, Ulid>`
- [ ] 5.3 Update `register()` return type similarly
- [ ] 5.4 Update `storeTokens()` to pass `Ulid` directly
- [ ] 5.5 Update internal methods that read/write userId

## 6. Update Test Fakes

- [ ] 6.1 Update `FakeSecureTokenStorage` in `shared/src/commonTest/`
- [ ] 6.2 Update `FakeSecureTokenStorage` in `composeApp/src/commonTest/`
- [ ] 6.3 Update `FakePublicAuthService` in both locations
- [ ] 6.4 Update any tests that construct `AuthResponse` or `AuthState.Authenticated`

## 7. Verification

- [ ] 7.1 Run `./gradlew :shared:jvmTest` to verify shared module tests pass
- [ ] 7.2 Run `./gradlew :server:test` to verify server tests pass
- [ ] 7.3 Run `./gradlew :composeApp:jvmTest` to verify desktop tests pass
- [ ] 7.4 Run `./gradlew build` to verify full build succeeds
