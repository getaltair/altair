# Change: Consolidate InvalidInvite and InvalidInviteCode error types

## Why

The `AuthError` sealed interface has two similar error types for invite code issues:
- `InvalidInvite(val code: String)` - stores the rejected code
- `InvalidInviteCode` - data object without the code

This duplication is confusing and inconsistent with the established pattern for other auth errors like `EmailAlreadyExists`, which intentionally omits the email to prevent information leakage.

## What Changes

- **Remove** `AuthError.InvalidInvite(val code: String)` data class
- **Keep** `AuthError.InvalidInviteCode` data object as the single error type for invalid invite codes
- **Update** all usages of `InvalidInvite` to use `InvalidInviteCode`:
  - `SurrealInviteCodeRepository.kt` (5 usages)
  - `InviteCodeRepository.kt` (1 KDoc reference)
  - `RegisterComponent.kt` (1 type check)
  - `ModuleErrorsTest.kt` (2 test usages)
- **Update** the spec to document the consolidated error type

## Impact

- Affected specs: `error-handling`
- Affected code:
  - `shared/src/commonMain/kotlin/com/getaltair/altair/domain/AuthError.kt`
  - `server/src/main/kotlin/com/getaltair/altair/db/repository/SurrealInviteCodeRepository.kt`
  - `shared/src/commonMain/kotlin/com/getaltair/altair/repository/InviteCodeRepository.kt`
  - `composeApp/src/commonMain/kotlin/com/getaltair/altair/ui/auth/RegisterComponent.kt`
  - `shared/src/commonTest/kotlin/com/getaltair/altair/domain/ModuleErrorsTest.kt`

## Security Benefit

Removing the code from the error type follows the principle of minimal information exposure, consistent with how `EmailAlreadyExists` intentionally doesn't store the email address. This prevents potential logging or serialization of rejected codes.

## References

- GitHub Issue: https://github.com/getaltair/altair/issues/19
