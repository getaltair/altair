# ADR-002: Deployment Targets and Minimum Hardware Requirements

## Status

Accepted (Amended 2026-04-12 — see Amendment below)

## Date

2026-04-12

## Context

Altair is a self-hosted platform. Users run the full server stack on their own hardware. The deployment footprint includes:

- **Rust/Axum** — API server
- **PostgreSQL** — Primary database
- **PowerSync Open Edition** — Offline-first sync service (Dart binary)
- **MongoDB** — Required by PowerSync as internal metadata store
- **S3-compatible storage** (RustFS) — Attachment/file storage
- **SvelteKit static build** — Web client served via reverse proxy or Axum

~~A Raspberry Pi 4 was evaluated as a target minimum.~~ ADR-012 removed Zitadel from the stack (2 fewer containers). The remaining services still include MongoDB (PowerSync dependency), which makes 4GB tight. The v1 focus is functionality, not footprint optimization — RPi 4 (4GB) is deferred to future optimization work.

### Estimated Memory Budget (Idle / Light Load, 1-5 Users)

| Component | RAM |
|-----------|-----|
| OS + overhead | ~500MB |
| PostgreSQL | ~200-400MB |
| PowerSync service | ~100-200MB |
| MongoDB (PowerSync dep) | ~200-400MB |
| Axum server | ~20-50MB |
| S3-compatible storage | ~200-300MB |
| **Total baseline** | **~1.2-1.8GB** |

Zitadel removal (ADR-012) saves an additional ~200-400MB that was previously on top of this baseline. On 8GB, comfortable headroom exists. On 4GB, functional but tight — deferred as an optimization target.

### Alternatives to PowerSync Evaluated

No other sync solution meets all requirements (offline local writes + PostgreSQL + Web SDK + Android native SDK + self-hostable). ElectricSQL dropped offline writes. Zero and Triplit lack Android SDKs. Custom sync would require 3-6 months of infrastructure work. The MongoDB dependency is the cost of not building a sync engine from scratch.

## Decision

Define two deployment tiers:

### Practical Minimum (v1)

| Resource | Target |
|----------|--------|
| RAM | 8GB |
| CPU | 4-core ARM64 or x86_64 |
| Storage | 64GB+ |
| Reference | Raspberry Pi 5 (8GB), $15-20/mo VPS, NAS |
| Users | 1-5 concurrent |

### Aspirational Minimum (Future Optimization)

| Resource | Target |
|----------|--------|
| RAM | 4GB |
| CPU | 4-core ARM64 or x86_64 |
| Storage | 32GB + external for attachments |
| Reference | Raspberry Pi 4 (4GB), $5-10/mo VPS |
| Users | 1-2 concurrent |
| Status | Deferred — requires footprint optimization (MongoDB alternatives, memory tuning) |

Deployment method: `docker compose` with all services defined in a single compose file.

SvelteKit web client is built as static output (static adapter) — no Bun/Node runtime required in production. Served directly by the Axum server or a reverse proxy.

## Consequences

### Positive

- Clear hardware expectations for users evaluating self-hosting
- Docker Compose deployment keeps operational complexity low
- Static web build eliminates a runtime dependency in production
- Both ARM64 and x86_64 supported (Pi and VPS users)

### Negative

- 8GB practical minimum is higher than some self-hosting enthusiasts prefer
- MongoDB dependency solely for PowerSync adds operational surface area
- 4GB support deferred — not validated or optimized for v1

### Neutral

- Deployment tiers are guidance, not hard limits — users with more resources get more headroom
- These targets should be validated with real benchmarks during implementation

## Amendment (2026-04-12)

**Context:** ADR-012 removed Zitadel from the deployment stack, reducing containers from 6 to 4. A Renaissance Architecture review identified that optimizing for RPi 4 (4GB) was premature — the v1 priority is functionality over footprint.

**Changes:**
- Practical minimum raised from 4GB to 8GB for v1
- RPi 4 (4GB) moved to aspirational/future optimization target
- Memory budget updated to reflect removal of Zitadel + postgres-init
- Stack reduced from 6 to 4 containers (see ADR-012)
