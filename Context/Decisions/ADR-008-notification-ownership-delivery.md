# ADR-008: Notification Ownership and Delivery

## Status

Accepted

## Date

2026-04-12

## Context

Altair generates notifications across multiple domains: routine reminders, timer completions, daily check-ins, low stock alerts, sync conflict warnings, and scheduled reviews. These must reach users across web and Android clients, including when a device is offline.

The core question is where notification logic lives (server vs. client) and how notifications reach each platform.

## Decision

### Notification Generation: Server-Owned with Client-Local Fallback

**Server is the primary notification source.** The server evaluates triggers, respects user timezone and preferences, and dispatches to all devices. This ensures cross-device consistency — a notification dismissed on one device is dismissed everywhere.

**Client-local fallback for time-critical offline scenarios:**
- Focus session timer completion (must fire even without connectivity)
- Offline routine reminders (if server is unreachable)
- Local device alarms (Android AlarmManager / Web setTimeout)

Client-local notifications are temporary — when connectivity returns, the server's canonical notification state takes precedence.

### Notification Categories

| Category | Priority | Trigger | Source |
|----------|----------|---------|--------|
| Timer Complete | High | Focus session ends | Client-local |
| Sync Conflict | High | Conflict detected during sync | Server |
| Routine Due | Normal | Schedule evaluation | Server |
| Daily Check-in | Normal | User preference schedule | Server |
| Evening Wrap-up | Normal | User preference schedule | Server |
| Weekly Harvest | Low | Weekly schedule | Server |
| Low Stock Alert | Normal | `item_event` threshold trigger | Server |
| Maintenance Due | Low | Scheduled maintenance date | Server |

### Delivery Channels

**Android:**
- Firebase Cloud Messaging (FCM) for push notifications
- Notification channels per category (Android O+) — high-priority channel for timer/conflicts
- Grouped notifications when multiple pending
- Widget refresh on relevant notifications

**Web:**
- Server-Sent Events (SSE) for real-time in-app notifications while connected
- In-app notification bell with unread count
- Toast-style alerts for real-time events
- Web Push API optional (user can enable for background notifications)

**Cross-platform:**
- Notification state synced via PowerSync (read/dismissed status)
- Dismissing on one device dismisses everywhere
- Quiet hours enforced at dispatch time (server-side)

### Server-Side Architecture

Notification processing runs in the background worker (same Rust binary, separate async task):

1. **Trigger evaluation** — Domain events (item below threshold, routine schedule hit, conflict detected) emit notification triggers
2. **Routing** — Notification router checks user preferences, quiet hours, channel availability
3. **Dispatch** — Send to FCM (Android), SSE channel (web), store in notifications table
4. **Delivery guarantees** — At-least-once via job queue with retry. Idempotent processing by `notification_id`. Max 3 retries before logging failure.

**Targets:** < 5 second latency from trigger to device, > 95% delivery success rate.

### Notification Data Model

```
notifications table:
  id              UUID PRIMARY KEY
  user_id         UUID NOT NULL (FK → users)
  category        TEXT NOT NULL
  title           TEXT NOT NULL
  body            TEXT
  entity_type     TEXT          -- what entity this relates to
  entity_id       UUID          -- deep link target
  priority        TEXT NOT NULL  -- high, normal, low
  read_at         TIMESTAMPTZ
  dismissed_at    TIMESTAMPTZ
  created_at      TIMESTAMPTZ NOT NULL
  updated_at      TIMESTAMPTZ NOT NULL
```

Synced to clients via PowerSync for cross-device read/dismiss state.

## Consequences

### Positive

- Server-owned ensures consistency across devices
- Client-local fallback handles the most time-critical case (timers) without connectivity
- FCM is the standard Android push mechanism — well-supported, no custom infrastructure
- SSE for web is lightweight — no WebSocket server needed
- Notification state syncs naturally through existing PowerSync infrastructure

### Negative

- FCM requires a Firebase project and API key — adds one external dependency (free tier sufficient)
- SSE requires an open connection per web client — acceptable for household-scale concurrent users
- Client-local notifications can temporarily diverge from server state until reconnection
- Quiet hours logic must be timezone-aware, adding complexity to scheduling

### Neutral

- Email notifications are a P2 feature — not in v1 scope
- Desktop notifications (Tauri OS API) deferred with desktop app to v2
- Notification preferences UI is required — users must be able to configure categories, quiet hours, and channels
