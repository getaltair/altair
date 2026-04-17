# ADR-029: `SyncStatusViewModel.isPending` Uses Boolean, Not Sealed Type

| Field | Value |
|---|---|
| **Status** | Accepted |
| **Date** | 2026-04-16 |
| **Feature** | 010-AndroidClientContd |

## Context

`SyncStatusViewModel` exposes `isPending: StateFlow<Boolean>`, mapping both `!status.connected` (offline/disconnected) and `status.uploading` (actively syncing while online) to `true`. The UI renders a single `CloudOff` icon for both conditions.

The alternative: a sealed `SyncState { Synced, Uploading, Disconnected }` would allow distinct icons and accessibility labels. `Uploading` + online is meaningfully different from `Disconnected` — one is normal background activity, the other is a network problem.

## Decision

Accept the Boolean for now. Defer the sealed type to a future UI polish pass.

## Rationale

- The current UI renders a single icon with no label. Boolean is sufficient for a single-icon indicator.
- Adding a sealed type now would require: (1) updating `SyncStatusViewModel`, (2) updating the composable to branch on three states, and (3) adding distinct icons/strings for each state — scope beyond the current feature.
- The Boolean conflation only misleads users if the sync icon carries a tooltip or accessibility label distinguishing the two states. Today it does not.

## Consequences

- The sync icon shows identically whether the device is offline or uploading. Users cannot distinguish the two from the icon alone.
- If a future design adds a tooltip, label, or distinct icon for sync vs. offline, this ADR must be revisited and the Boolean replaced with a sealed type.

## Future Trigger

Revisit when: the sync indicator is given an accessibility label, a tooltip, or distinct iconography for online-syncing vs. offline states. At that point, replace `isPending: StateFlow<Boolean>` with `syncState: StateFlow<SyncState>` where `SyncState` is a sealed class with `Synced`, `Uploading`, and `Disconnected` variants.
