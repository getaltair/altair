# Database Service Documentation

This directory contains comprehensive documentation for the Altair database service implementation.

## Files

- **[altair-db-schema-and-development.md](./altair-db-schema-and-development.md)** - Complete schema design, testing strategy, backup/restore, and development workflow
- **[altair-db-service-implementation.md](./altair-db-service-implementation.md)** - Step-by-step implementation guide for the shared SurrealDB service
- **[altair-guidance-integration-example.md](./altair-guidance-integration-example.md)** - Practical integration example showing how to use the database service in apps

## Overview

The Altair database service uses a shared local SurrealDB instance that all Altair applications connect to. This enables:

- **Cross-app data integration** - Tasks, notes, and items can reference each other
- **Single source of truth** - One database for all apps
- **Consistent data model** - Shared schemas and relationships
- **Powerful queries** - Graph traversal, full-text search, and relational queries

## Quick Links

- Package source: `packages/altair-db-service/`
- Package README: `packages/altair-db-service/README.md`
- Integration examples: See altair-guidance-integration-example.md

## Getting Started

1. Read the [implementation guide](./altair-db-service-implementation.md) for setup instructions
2. Review the [schema design](./altair-db-schema-and-development.md) to understand the data model
3. See [integration examples](./altair-guidance-integration-example.md) for usage patterns
