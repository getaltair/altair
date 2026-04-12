# ADR-005: Attachment Storage Abstraction

## Status

Accepted

## Date

2026-04-12

## Context

Altair supports attachments (photos, PDFs, scanned documents, audio, video) on notes, items, and quests. The system is self-hosted, so attachment storage cannot depend on commercial cloud providers. Desktop was deferred to v2 (ADR-001), removing local filesystem storage as the primary attachment strategy.

Attachments have two components:
- **Metadata** — stored in PostgreSQL, synced via PowerSync
- **Binary blobs** — stored separately, fetched on demand

The sync layer handles metadata only. Binary blobs follow a separate upload/download lifecycle.

## Decision

### Storage: Self-Hosted S3-Compatible Object Storage

Use an S3-compatible object storage service (Garage, RustFS, or MinIO) as the binary blob store. The server exposes an attachment API that abstracts the storage backend.

**Storage abstraction trait (Rust):**
- `upload(bucket, key, data, content_type) → Result<()>`
- `download(bucket, key) → Result<Stream>`
- `delete(bucket, key) → Result<()>`
- `presign_url(bucket, key, expiry) → Result<Url>`

Implementations: S3-compatible adapter (covers Garage, RustFS, MinIO, AWS S3, Backblaze B2). Additional adapters (local filesystem) deferred to v2 with desktop.

### Attachment Lifecycle

**Upload flow:**
1. Client creates attachment metadata record locally (SQLite)
2. Metadata syncs to server via PowerSync
3. Client uploads binary to server attachment endpoint (background, resumable)
4. Server writes blob to object storage
5. Server updates attachment metadata with storage location and status
6. Metadata update syncs back to all clients

**Download flow (other devices):**
1. Attachment metadata syncs to client
2. Client displays placeholder with metadata (filename, type, size)
3. On user request or viewport entry, client fetches binary via server endpoint
4. Server generates presigned URL or proxies download from object storage
5. Client caches binary locally

**Delete flow:**
- Soft delete on metadata record
- Binary cleanup deferred until all clients have synced the deletion
- Background job garbage-collects orphaned blobs

### Derivative Processing

Background worker generates thumbnails and previews after upload:
- Image thumbnails (multiple sizes)
- PDF preview images
- Video thumbnails

Derivatives stored alongside originals in object storage with predictable key patterns.

## Consequences

### Positive

- Single storage abstraction covers all self-hosted S3-compatible options
- No cloud vendor dependency — Garage and RustFS run on a Pi
- Metadata/binary separation keeps sync lightweight (no blob replication through PowerSync)
- Presigned URLs enable direct client-to-storage downloads when appropriate
- Background derivative generation keeps upload path fast

### Negative

- Object storage service adds one more container to the deployment stack
- Resumable upload support adds client-side complexity
- Garbage collection of orphaned blobs requires careful coordination with sync state
- Derivative generation adds background job work

### Neutral

- Local filesystem adapter for desktop (v2) will implement the same trait
- Deduplication by content hash is a future optimization, not v1 scope
- Storage capacity planning depends on user attachment volume — unbounded by design
