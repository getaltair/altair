# Altair Project Overview

## Purpose

Altair is an ADHD-focused productivity ecosystem with three desktop apps:

| App | Purpose | Key Entities |
|-----|---------|--------------|
| **Guidance** | Task management (Quest-Based Agile) | Quest, Campaign |
| **Knowledge** | Personal knowledge management | Note, Folder |
| **Tracking** | Inventory management | Item, Location |

Plus **Quick Capture** for zero-friction input across all apps.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Database | SurrealDB 2.x (embedded + cloud) |
| Object Storage | S3-compatible (Minio local, Backblaze B2 cloud) |
| Desktop | Tauri 2.0 + Svelte |
| Backend | Rust + Axum (localhost:3847) |
| Mobile | Tauri 2.0 Android |
| IPC | Tauri Commands (not REST for desktop) |
| Type Safety | tauri-specta (Rust → TypeScript) |
| Embeddings | Local ONNX (all-MiniLM-L6-v2, ~25MB) |

## Project Status

The project is currently in **planning/documentation phase**. Core docs exist:
- `docs/technical-architecture.md` - System architecture
- `docs/domain-model.md` - Entities and relationships
- `docs/glossary.md` - Terminology (Quest, Campaign, etc.)
- `docs/decision-log.md` - Architectural decisions
- `docs/spec-backlog.md` - Prioritized feature backlog
- `.spectrena/memory/constitution.md` - Core principles

Implementation code (apps/, packages/, backend/) not yet created.

## Project Structure (Planned)

```
altair/
├── apps/
│   ├── guidance/           # Tauri - Quest management
│   ├── knowledge/          # Tauri - PKM
│   ├── tracking/           # Tauri - Inventory
│   └── mobile/             # Tauri Android
├── packages/
│   ├── ui/                 # Svelte design system
│   ├── bindings/           # Generated TypeScript (tauri-specta)
│   ├── db/                 # SurrealDB schema + queries
│   ├── sync/               # Change feed sync
│   ├── storage/            # S3 client
│   └── search/             # Embeddings + hybrid search
├── backend/
│   ├── src/
│   │   ├── commands/       # Tauri IPC handlers
│   │   ├── api/            # REST handlers (mobile only)
│   │   ├── auth/           # Auth plugins
│   │   ├── sync/           # Sync engine
│   │   ├── embeddings/     # ONNX inference
│   │   ├── providers/      # AI plugins (optional)
│   │   └── storage/        # S3 integration
│   └── migrations/         # SurrealDB migrations
├── specs/                  # Specifications (SDD)
├── docs/                   # Architecture, domain model, etc.
├── CLAUDE.md               # Project context for Claude Code
└── AGENTS.md               # AI agent guidelines
```

## Core Principles (Constitution)

1. **Local-First** - All features work offline; cloud is optional
2. **ADHD-Friendly** - WIP=1, progressive disclosure, zero-friction capture
3. **Ubiquitous Language** - Quest/Campaign/Note/Item terminology
4. **Soft Delete** - Archive only, no hard deletes
5. **Plugin Architecture** - Trait-based auth/AI providers
6. **Privacy by Default** - Local embeddings, no telemetry without opt-in
7. **Spec-Driven Development** - spectrena workflow for features
