# ADR-024: Remove `passwordHash` from Android Client Entity and PowerSync Schema

| Field | Value |
|---|---|
| **Status** | Accepted |
| **Date** | 2026-04-16 |
| **Feature** | 009-AndroidClient |

## Context

`UserEntity.passwordHash: String?` persists the server-side bcrypt hash in the client-side SQLite database. The hash serves no function on the client — all authentication is handled server-side. The PowerSync `users` table definition includes `password_hash`, meaning the hash is transmitted over the sync channel to every client device.

This represents unnecessary exposure of sensitive credential data. A compromised device or SQLite file reveals the hash, which is one step closer to a plaintext password than necessary.

## Decision

Remove `password_hash` from:
1. `AltairPowerSyncSchema` — `users` table column list
2. `UserEntity` — remove `passwordHash: String?` field
3. PowerSync server-side sync rules — exclude `password_hash` from the `users` bucket

Update:
- `TodayViewModel.currentUser` cursor mapping (column indices shift when `password_hash` is removed from the schema)
- Room database version bump with a migration that removes the column from local storage

## Rationale

- The column provides zero client-side value
- Transmitting credential data over sync violates least-privilege data exposure
- ADR-012 (security invariant SEC-6) requires minimizing sensitive data on clients
- Standard practice: only sync what the client needs to function

## Consequences

- Requires a Room database migration to drop the column from existing installations
- Cursor index mapping in `TodayViewModel.currentUser` must be updated after column removal
- PowerSync sync rules on the server must be updated to exclude the column (server-side change)
- No behavioral impact on auth flows — `passwordHash` is never read in client logic

## Implementation Notes

When removing from PowerSync schema, `SELECT *` on the `users` table will return columns in order:
`id, email, display_name, is_admin, status, created_at, updated_at, deleted_at`

Update `TodayViewModel.currentUser` cursor mapping accordingly.
