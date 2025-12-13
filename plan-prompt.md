Tech Stack:

- Desktop: Tauri 2.0 with Svelte 5 (runes) frontend
- Mobile: Tauri Android (same codebase)
- Database: SurrealDB 2.x embedded (surrealkv storage engine)
- Object Storage: S3-compatible (Minio local, Backblaze B2/Cloudflare R2/etc cloud)
- Backend: Rust with Cargo workspace at repo root

Architecture:

- Monorepo with pnpm workspaces + Turborepo for TypeScript, Cargo workspace for Rust
- Shared crates: altair-core, altair-db, altair-sync, altair-storage,
  altair-search, altair-auth, altair-commands
- Each desktop app (guidance, knowledge, tracking) has own src-tauri/ as
  workspace member
- Apps share same SurrealDB file, no separate daemon process
- Type-safe IPC via tauri-specta (generates TypeScript bindings)

Key Libraries:

- UI: shadcn-svelte + Tailwind CSS, TipTap v for rich text editing
- Embeddings: ONNX Runtime with all-MiniLM-L6-v2 (~25MB model)
- Testing: Vitest + @testing-library/svelte for frontend, cargo test for Rust

Reference docs in docs/ folder:

- technical-architecture.md (full architecture)
- domain-model.md (entities, relationships)
- decision-log.md (ADRs)
- design-system.md (UI/UX principles)
