# ADR-002: Deployment Targets and Minimum Hardware Requirements

## Status

Accepted

## Date

2026-04-12

## Context

Altair is a self-hosted platform. Users run the full server stack on their own hardware. The deployment footprint includes:

- **Rust/Axum** — API server
- **PostgreSQL** — Primary database
- **PowerSync Open Edition** — Offline-first sync service (Dart binary)
- **MongoDB** — Required by PowerSync as internal metadata store
- **S3-compatible storage** (Garage, RustFS) — Attachment/file storage
- **SvelteKit static build** — Web client served via reverse proxy or Axum

A Raspberry Pi 4 was evaluated as a target minimum. Research into PowerSync's self-hosted requirements revealed a mandatory MongoDB dependency, which adds ~200-400MB RAM baseline.

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

On a 4GB device this leaves 2-2.8GB free — functional but tight under memory pressure. On 8GB, comfortable headroom exists.

### Alternatives to PowerSync Evaluated

No other sync solution meets all requirements (offline local writes + PostgreSQL + Web SDK + Android native SDK + self-hostable). ElectricSQL dropped offline writes. Zero and Triplit lack Android SDKs. Custom sync would require 3-6 months of infrastructure work. The MongoDB dependency is the cost of not building a sync engine from scratch.

## Decision

Define two deployment tiers:

### Minimum (Personal)

| Resource | Target |
|----------|--------|
| RAM | 4GB |
| CPU | 4-core ARM64 or x86_64 |
| Storage | 32GB + external for attachments |
| Reference | Raspberry Pi 4 (4GB), $5-10/mo VPS |
| Users | 1-2 concurrent |

### Recommended (Household)

| Resource | Target |
|----------|--------|
| RAM | 8GB |
| CPU | 4-core ARM64 or x86_64 |
| Storage | 64GB+ |
| Reference | Raspberry Pi 5, $15-20/mo VPS, NAS |
| Users | 3-5 concurrent |

Deployment method: `docker compose` with all services defined in a single compose file.

SvelteKit web client is built as static output (static adapter) — no Bun/Node runtime required in production. Served directly by the Axum server or a reverse proxy.

## Consequences

### Positive

- Clear hardware expectations for users evaluating self-hosting
- Docker Compose deployment keeps operational complexity low
- Static web build eliminates a runtime dependency in production
- Both ARM64 and x86_64 supported (Pi and VPS users)

### Negative

- 4GB minimum is tight; users on minimum hardware may experience degraded performance under load
- MongoDB dependency solely for PowerSync adds operational surface area
- No "runs on a Pi Zero" story — some self-hosting enthusiasts may find 4GB too high

### Neutral

- Deployment tiers are guidance, not hard limits — users with more resources get more headroom
- These targets should be validated with real benchmarks during implementation
