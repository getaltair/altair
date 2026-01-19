# Tasks: Add Temporal Invariants

## 1. InviteCode Temporal Invariants

- [x] 1.1 Add `require(expiresAt > createdAt)` invariant to InviteCode init block
- [x] 1.2 Add `require(createdAt <= now + MAX_CLOCK_SKEW)` invariant (prevent far-future createdAt)
- [x] 1.3 Add `require(expiresAt <= createdAt + MAX_EXPIRY_DURATION)` invariant (reasonable upper bound)
- [x] 1.4 Add companion object constants for `MAX_CLOCK_SKEW` and `MAX_EXPIRY_DURATION`
- [x] 1.5 Add factory method `InviteCode.create(...)` with duration-based expiration

## 2. RefreshToken Temporal Invariants

- [x] 2.1 Add `require(expiresAt > createdAt)` invariant to RefreshToken init block
- [x] 2.2 Add `require(createdAt <= now + MAX_CLOCK_SKEW)` invariant (prevent far-future createdAt)
- [x] 2.3 Add `require(expiresAt <= createdAt + MAX_EXPIRY_DURATION)` invariant (reasonable upper bound)
- [x] 2.4 Add companion object constants for `MAX_CLOCK_SKEW` and `MAX_EXPIRY_DURATION`
- [x] 2.5 Add factory method `RefreshToken.create(...)` with duration-based expiration

## 3. Testing

- [x] 3.1 Add unit tests for InviteCode temporal invariants (valid and invalid cases)
- [x] 3.2 Add unit tests for RefreshToken temporal invariants (valid and invalid cases)
- [x] 3.3 Add unit tests for factory methods
- [x] 3.4 Verify existing tests still pass

## 4. Integration

- [x] 4.1 Review existing code creating InviteCode/RefreshToken instances
- [x] 4.2 Migrate to factory methods where appropriate
- [x] 4.3 Run full test suite
