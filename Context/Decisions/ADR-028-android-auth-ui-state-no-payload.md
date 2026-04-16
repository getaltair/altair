# ADR-028: `AuthUiState.Success` Carries No User Payload

| Field | Value |
|---|---|
| **Status** | Accepted |
| **Date** | 2026-04-16 |
| **Feature** | 009-AndroidClient |

## Context

`AuthUiState.Success` is a bare `object` with no data. After a successful login, `MainActivity` navigates to `MainScaffold` based on the `isAuthenticated` StateFlow. The receiving screen (`TodayViewModel`) then independently queries the local DB for the current user, creating a timing window where the user has navigated to `MainScaffold` but `currentUser.value` is still null (before PowerSync first-sync completes).

The alternative: pass the user record (or at least `userId`) through `AuthUiState.Success`, allowing `MainScaffold` or `TodayViewModel` to begin loading with a pre-populated identity.

## Decision

Accept the current two-step pattern: authenticate → navigate → fetch user from local DB.

Do not add user identity to `AuthUiState.Success`.

## Rationale

- The login response from the server returns tokens only, not the full user record. Populating `AuthUiState.Success` with user data would require an additional `/auth/me` API call during the login flow, adding latency.
- `TodayViewModel.currentUser` already handles the null state gracefully (shows empty/loading state until the first sync populates the `users` table). This is acceptable offline-first behavior.
- Adding identity to `AuthUiState.Success` would couple the auth flow to the user profile schema, making future user record changes require changes to auth state.

## Consequences

- `TodayViewModel.currentUser` shows `null` briefly after first login until PowerSync completes its first sync
- The UI should show a loading or empty state when `currentUser == null` — this is expected behavior for offline-first apps
- If the null window causes visible UX issues (e.g., blank screens on slow connections), add an explicit "syncing" splash screen rather than passing identity through auth state

## Future Trigger

Revisit if: a dedicated `/api/auth/me` endpoint is added that returns the user record and can be called cheaply after login without an extra round trip.
