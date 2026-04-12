# Altair — Project Overview

| Field | Value |
|---|---|
| **Document** | 00-project-overview |
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-04-12 |
| **Source Docs** | `docs/altair-core-prd.md`, `docs/altair-architecture-spec.md` |

---

## Executive Summary

Altair is a personal operating system for managing knowledge, goals, and resources across everyday life. It integrates three primary domains — **Guidance** (goals, initiatives, routines), **Knowledge** (notes and linked information), and **Tracking** (inventory and resource monitoring) — into a single offline-first, sync-aware, self-hosted platform.

Altair is not three unrelated apps in a trench coat. It is one system with shared core concepts: initiatives, tags, attachments, search, and a cross-domain entity relationship graph that enables connections like "this note supports that initiative" or "this quest requires that item."

---

## Philosophy

The architecture is driven by product constraints, not framework fandom:

- **Mobile is a daily driver**, not a companion
- **Web is a hard requirement**, not a limp dashboard
- **Desktop adds power-user capability**, but is not required for basic viability
- **Offline mode** is mandatory for mobile and desktop
- **Sync conflicts must never silently lose data**
- **Cross-app integration** is a defining product differentiator
- **Self-hosting and privacy** are first-class constraints
- **AI must be optional and degrade gracefully**

The visual identity follows the **"Digital Sanctuary / Ethereal Canvas"** design system (see [`./DESIGN.md`](../../DESIGN.md)) — atmospheric, unhurried, and distinguished by tonal depth rather than structural lines.

---

## Design Principles

1. **Offline-first** — Every client writes locally and syncs when connectivity returns. No spinner-gated CRUD.
2. **One system, many surfaces** — Guidance, Knowledge, and Tracking share a unified data model. Cross-domain relationships are a feature, not an accident.
3. **Right tool for the job** — Native Android for capture/notifications, shared SvelteKit for web/desktop, Rust for the server. No universal hammer.
4. **Modular monolith** — Server is structured by bounded context with clean interfaces, not premature microservices.
5. **Contracts over shared logic** — Platforms share schemas, wire contracts, and design tokens. Platform behavior stays native.
6. **AI as optional enrichment** — Never required for core CRUD. Never the sole source of truth.

---

## Target Users

- **Primary:** Individual power users who want a single system for goals, notes, and household inventory
- **Secondary:** Small households sharing inventory, chores, and shopping lists
- **Tertiary:** Self-hosters who want full data ownership

---

## Platform Strategy

| Tier | Platform | Role | Stack |
|---|---|---|---|
| 1 | Android | Daily interaction engine — capture, notifications, routines, barcode scanning | Kotlin / Jetpack Compose |
| 1 | Web | Universal access — planning, review, knowledge editing, admin | SvelteKit 2 / Svelte 5 / TypeScript |
| 2 | Desktop (Linux, Windows) | Power-user shell — deep editing, multi-window, bulk ops, local AI | Tauri 2 + shared SvelteKit UI |
| 3 | WearOS | Future companion — reminders, routine completion, quick logging | TBD |
| 3 | iOS | Best effort / community scope | TBD |

---

## Tech Stack

| Layer | Technology | Purpose |
|---|---|---|
| Web Client | SvelteKit 2 / Svelte 5 / TypeScript | Web + Desktop shared UI |
| Desktop Shell | Tauri 2 | Native OS integration for Linux/Windows |
| Mobile Client | Kotlin / Jetpack Compose | Android native |
| Backend | Rust / Axum | REST API, modular monolith |
| Server Database | PostgreSQL | Source of truth |
| Sync Layer | PowerSync | Bidirectional client-server sync |
| Client Database | SQLite (Room on Android, PowerSync JS on web) | Offline-first local storage |
| Object Storage | S3-compatible (MinIO for self-hosted) | Attachment binaries |
| Search | FTS + vector index | Keyword + semantic hybrid search |
| Deployment | Docker Compose | Self-hosted first |

---

## Success Metrics

- Local actions < 200ms
- Remote actions < 1s
- Must tolerate intermittent connectivity
- Data must not be lost during sync conflicts
- Offline capture works on all Tier 1 platforms

---

## Document Index

| File | Type | Description |
|---|---|---|
| `00-project-overview.md` | Overview | This document |
| `01-PRD-001-core.md` | PRD | Core platform — identity, sync, attachments, search, tags |
| `01-PRD-002-guidance.md` | PRD | Guidance domain — goals, initiatives, quests, routines |
| `01-PRD-003-knowledge.md` | PRD | Knowledge domain — notes, backlinks, capture |
| `01-PRD-004-tracking.md` | PRD | Tracking domain — items, locations, inventory |
| `02-domain-model.md` | Domain Model | Bounded contexts, entities, aggregates, relationships |
| `03-invariants.md` | Invariants | System-wide rules that must never be violated |
| `04-architecture.md` | Architecture | Modules, layers, data flow, deployment |
| `05-erd.md` | ERD | Database schema, indices, contract surfaces |
| `06-state-machines.md` | State Machines | Entity lifecycles and status transitions |
| `07-user-flows.md` | User Flows | Screen flows, onboarding, key interaction sequences |
| `08-SUB-001-notifications.md` | Subsystem | Notification types, scheduling, delivery |
| `09-PLAT-001-android.md` | Platform | Android-specific scope, components, constraints |
| `09-PLAT-002-web.md` | Platform | Web-specific scope, components, constraints |
| `09-PLAT-003-desktop.md` | Platform | Desktop-specific scope, components, constraints |
| `09-PLAT-004-wearos.md` | Platform | WearOS future scope |
| `10-PLAN-001-v1.md` | Plan | v1 implementation plan with dependency graph |
| `GAPS.md` | Report | Sections needing human review |

---

## Related Documents

- [`../../DESIGN.md`](../../DESIGN.md) — Visual design system ("The Ethereal Canvas / Digital Sanctuary")
- `docs/altair-core-prd.md` — Original core PRD
- `docs/altair-architecture-spec.md` — Original architecture specification
- `docs/altair-shared-contracts-spec.md` — Shared contracts specification
- `docs/altair-powersync-sync-spec.md` — PowerSync sync scope specification
- `docs/altair-schema-design-spec.md` — Schema design specification
