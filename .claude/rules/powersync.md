# PowerSync Conventions

Applies to: sync layer between clients and server

## Role

PowerSync provides offline-first sync between client-side SQLite and server-side PostgreSQL. Clients read/write to a local PowerSync-managed SQLite database. Changes propagate bidirectionally through the PowerSync service.

## Schema

- PowerSync schema defined in `apps/web/src/lib/sync/schema.ts`
- Schema must mirror the PostgreSQL table structure for synced tables
- Column types map: Postgres `uuid` -> PowerSync `text`, Postgres `timestamptz` -> PowerSync `text`
- Every synced table requires `id` (text) as primary key

## Sync Rules

- Define sync rules to scope data per user (bucket-based partitioning)
- Sync rules must enforce user isolation; never sync another user's data
- Partial sync: only sync tables and rows the client actually needs

## Connector

- `PowerSyncBackendConnector` handles auth token refresh and CRUD upload
- Upload endpoint batches client changes to the server API
- Auth tokens must be refreshed before expiry; handle token refresh failures gracefully

## Conflict Resolution

- Last-write-wins by default (based on `updated_at`)
- Document any tables where custom conflict resolution is needed
- Sync conflicts must never silently lose data (core architectural constraint)

## Client Integration

- Web: PowerSync JS SDK with SvelteKit
- Android: PowerSync Kotlin SDK with Room integration
- Desktop: shares web client PowerSync instance via Tauri

## Testing

- Unit tests mock the PowerSync connector
- Integration tests verify sync round-trips against a real backend
- Test offline-to-online transitions and conflict scenarios
