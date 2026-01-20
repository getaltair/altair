# Change: Add type safety to AuthDtos using Ulid and UserRole types

## Why

The authentication DTOs currently use `String` types for fields that have well-defined domain types (`Ulid` for user IDs, `UserRole` enum for roles). This loses compile-time type safety and requires runtime string parsing, increasing the risk of bugs from typos or incorrect format assumptions.

## What Changes

- `AuthResponse.userId` changes from `String` to `Ulid`
- `AuthResponse.role` changes from `String` to `UserRole`
- `AuthState.Authenticated.userId` changes from `String` to `Ulid`
- `SecureTokenStorage.saveUserId` and `getUserId` change from `String` to `Ulid`
- Server implementations convert typed values to DTOs directly (already have typed `User` model)
- Platform-specific `SecureTokenStorage` implementations store `Ulid` as its string representation

## Impact

- Affected specs: `dto`, `authentication`
- Affected code:
  - `shared/src/commonMain/kotlin/com/getaltair/altair/dto/auth/AuthDtos.kt`
  - `shared/src/commonMain/kotlin/com/getaltair/altair/service/auth/AuthManager.kt`
  - `shared/src/commonMain/kotlin/com/getaltair/altair/service/auth/SecureTokenStorage.kt`
  - `server/src/main/kotlin/com/getaltair/rpc/PublicAuthServiceImpl.kt`
  - Platform implementations: `AndroidSecureTokenStorage`, `IosSecureTokenStorage`, `DesktopSecureTokenStorage`
  - Test fakes: `FakeSecureTokenStorage`, `FakePublicAuthService`

## Notes

- Both `Ulid` and `UserRole` are already `@Serializable`, so RPC serialization will work automatically
- The `Ulid` value class serializes as its string value, maintaining wire compatibility
- The `UserRole` enum uses `@SerialName` annotations (`"admin"`, `"member"`) for JSON serialization
- Storage implementations will continue storing the string representation internally
