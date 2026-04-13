# packages/contracts — Shared Contract Registry

This package is the single source of truth for cross-platform data contracts in Altair. It defines the canonical identifiers and schema definitions shared across all clients (web, Android, server), ensuring consistency in entity types, relation types, and sync configuration without duplication or drift.

## Registry Files

The following files will constitute this package once populated:

- **`entity-types.json`** — canonical registry of entity type identifiers (notes, goals, items, etc.) shared across all clients
- **`relation-types.json`** — registry of relation type identifiers used in the knowledge graph
- **`sync-streams.json`** — PowerSync sync stream definitions (which tables each client subscribes to)

## Code Generation Targets

From these registries, bindings are generated for each client platform:

- **TypeScript (`apps/web/`)** — type-safe entity/relation constants
- **Kotlin (`apps/android/`)** — enum/sealed class definitions
- **Rust (`apps/server/`)** — enum and serde-compatible type definitions

A code generation script (to be defined) will read the JSON registry files and emit the appropriate bindings into each application's source tree. This ensures all platforms stay in sync from a single authoritative source.

## Status

Population of this package is deferred to Step 2 (Shared Contracts). This directory is a placeholder only — no generated code or registry files exist yet.
