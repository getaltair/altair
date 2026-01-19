# Change: Add Temporal Invariants to InviteCode and RefreshToken

GitHub Issue: https://github.com/getaltair/altair/issues/16

## Why

The `InviteCode` and `RefreshToken` domain types currently only check that `expiresAt` is after `createdAt` (implicitly, via the `isExpired` check at runtime), but they don't prevent construction with dates in the past (expired tokens/codes) or dates far in the future. This could lead to invalid entities being created that represent nonsensical temporal states.

## What Changes

- Add construction-time validation that `expiresAt > createdAt`
- Add validation that `createdAt` is not in the far future (allowing reasonable clock skew, e.g., 5 minutes)
- Add validation that `expiresAt` is within reasonable bounds (not years in the past or unreasonably far in the future)
- Add factory methods with "expires in X duration" semantics for convenient and correct construction

## Impact

- Affected specs: `domain-models`
- Affected code:
  - `shared/src/commonMain/kotlin/com/getaltair/altair/domain/model/system/InviteCode.kt`
  - `shared/src/commonMain/kotlin/com/getaltair/altair/domain/model/system/RefreshToken.kt`
