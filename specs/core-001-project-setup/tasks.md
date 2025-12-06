# Tasks: CORE-001-project-setup

**Branch**: `spec/core-001-project-setup` | **Date**: 2025-12-01

## Phase 1: Root Configuration (Foundation)

- [x] **1.1**: Initialize pnpm workspace

  - **Acceptance**: `pnpm-workspace.yaml` exists with correct workspace patterns
  - **Files**: `pnpm-workspace.yaml`
  - **Verification**: File contains `apps/*` and `packages/*` patterns

- [x] **1.2**: Create root package.json with scripts and pnpm.overrides

  - **Acceptance**: Root package.json has dev/build/lint/test scripts, pnpm.overrides for version consistency
  - **Files**: `package.json`
  - **Verification**: `pnpm install` resolves dependencies without conflicts

- [x] **1.3**: Configure Turborepo pipeline

  - **Acceptance**: `turbo.json` defines build/dev/lint/test tasks with correct dependencies and cache settings
  - **Files**: `turbo.json`
  - **Verification**: Turborepo can parse config without errors

- [x] **1.4**: Create shared TypeScript configs

  - **Acceptance**: Base config with strict mode, extends for apps/packages
  - **Files**: `tsconfig.json`, `tsconfig.base.json`
  - **Verification**: Valid TypeScript configuration that can be extended

- [x] **1.5**: Configure ESLint (flat config for Svelte 5)

  - **Acceptance**: Flat ESLint config with Svelte 5 parser, TypeScript support
  - **Files**: `eslint.config.js`
  - **Verification**: `pnpm eslint --version` works, config is valid

- [x] **1.6**: Configure Prettier

  - **Acceptance**: Prettier config with Svelte plugin, ignore file excludes build artifacts
  - **Files**: `.prettierrc`, `.prettierignore`
  - **Verification**: `pnpm prettier --check .` runs without errors

- [x] **1.7**: Set up prek pre-commit hooks

  - **Acceptance**: `.pre-commit-config.yaml` hooks for format, lint, type-check (Rust and TS)
  - **Files**: `.pre-commit-config.yaml`
  - **Verification**: Hooks can be registered with `prek install`

- [x] **1.8**: Create .gitignore and .editorconfig
  - **Acceptance**: Standard patterns for Node, Rust, Tauri; EditorConfig for consistent formatting
  - **Files**: `.gitignore`, `.editorconfig`
  - **Verification**: Files exist and contain expected patterns

**Phase Exit Criteria**: `pnpm install` succeeds at repo root without warnings

---

## Phase 2: Rust Backend Workspace

- [x] **2.1**: Create backend Cargo workspace

  - **Acceptance**: `backend/Cargo.toml` defines workspace with `crates/*` members
  - **Files**: `backend/Cargo.toml`
  - **Verification**: `cargo metadata` shows workspace structure

- [x] **2.2**: Scaffold altair-core crate (shared types)

  - **Acceptance**: Lib crate with common types, errors, traits
  - **Files**: `backend/crates/altair-core/Cargo.toml`, `backend/crates/altair-core/src/lib.rs`
  - **Verification**: `cargo build -p altair-core` succeeds

- [x] **2.3**: Scaffold altair-db crate (placeholder)

  - **Acceptance**: Lib crate depending on altair-core, placeholder for SurrealDB integration
  - **Files**: `backend/crates/altair-db/Cargo.toml`, `backend/crates/altair-db/src/lib.rs`
  - **Verification**: `cargo build -p altair-db` succeeds

- [x] **2.4**: Scaffold altair-sync crate (placeholder)

  - **Acceptance**: Lib crate depending on altair-core, placeholder for change feed sync
  - **Files**: `backend/crates/altair-sync/Cargo.toml`, `backend/crates/altair-sync/src/lib.rs`
  - **Verification**: `cargo build -p altair-sync` succeeds

- [x] **2.5**: Scaffold altair-storage crate (placeholder)

  - **Acceptance**: Lib crate depending on altair-core, placeholder for S3 integration
  - **Files**: `backend/crates/altair-storage/Cargo.toml`, `backend/crates/altair-storage/src/lib.rs`
  - **Verification**: `cargo build -p altair-storage` succeeds

- [ ] **2.6**: Scaffold altair-search crate (placeholder)

  - **Acceptance**: Lib crate depending on altair-core, placeholder for embeddings
  - **Files**: `backend/crates/altair-search/Cargo.toml`, `backend/crates/altair-search/src/lib.rs`
  - **Verification**: `cargo build -p altair-search` succeeds

- [ ] **2.7**: Scaffold altair-auth crate (placeholder)

  - **Acceptance**: Lib crate depending on altair-core, placeholder for auth plugins
  - **Files**: `backend/crates/altair-auth/Cargo.toml`, `backend/crates/altair-auth/src/lib.rs`
  - **Verification**: `cargo build -p altair-auth` succeeds

- [ ] **2.8**: Scaffold altair-commands crate (Tauri commands)
  - **Acceptance**: Lib crate depending on altair-core, exports empty command handlers
  - **Files**: `backend/crates/altair-commands/Cargo.toml`, `backend/crates/altair-commands/src/lib.rs`
  - **Verification**: `cargo build -p altair-commands` succeeds

**Phase Exit Criteria**: `cargo check` succeeds in `backend/` workspace

---

## Phase 3: Shared TypeScript Packages

- [ ] **3.1**: Scaffold ui package (Svelte 5 components)

  - **Acceptance**: Package with `src/index.ts` exporting placeholder Button component
  - **Files**: `packages/ui/package.json`, `packages/ui/src/index.ts`, `packages/ui/tsconfig.json`, `packages/ui/svelte.config.js`
  - **Verification**: Package builds successfully, exports are importable

- [ ] **3.2**: Scaffold bindings package (tauri-specta output)

  - **Acceptance**: Package with `src/index.ts` and placeholder types, README explains generation
  - **Files**: `packages/bindings/package.json`, `packages/bindings/src/index.ts`, `packages/bindings/tsconfig.json`, `packages/bindings/README.md`
  - **Verification**: Package builds, types are valid TypeScript

- [ ] **3.3**: Scaffold db package (SurrealDB utilities)

  - **Acceptance**: Package with `src/index.ts` exporting placeholder schema types
  - **Files**: `packages/db/package.json`, `packages/db/src/index.ts`, `packages/db/tsconfig.json`
  - **Verification**: Package builds successfully

- [ ] **3.4**: Scaffold sync package (sync utilities)

  - **Acceptance**: Package with `src/index.ts` exporting placeholder sync helpers
  - **Files**: `packages/sync/package.json`, `packages/sync/src/index.ts`, `packages/sync/tsconfig.json`
  - **Verification**: Package builds successfully

- [ ] **3.5**: Scaffold storage package (S3 utilities)

  - **Acceptance**: Package with `src/index.ts` exporting placeholder S3 client wrapper
  - **Files**: `packages/storage/package.json`, `packages/storage/src/index.ts`, `packages/storage/tsconfig.json`
  - **Verification**: Package builds successfully

- [ ] **3.6**: Scaffold search package (embedding utilities)
  - **Acceptance**: Package with `src/index.ts` exporting placeholder embedding functions
  - **Files**: `packages/search/package.json`, `packages/search/src/index.ts`, `packages/search/tsconfig.json`
  - **Verification**: Package builds successfully

**Phase Exit Criteria**: `pnpm build` succeeds for all packages, Turborepo caches outputs

---

## Phase 4: Tauri Applications

- [ ] **4.1**: Scaffold Guidance app (Tauri 2 + Svelte 5)

  - **Acceptance**: App with Tauri 2.9.3+, Svelte 5 frontend, basic routing, launches successfully
  - **Files**: `apps/guidance/package.json`, `apps/guidance/src/`, `apps/guidance/src-tauri/`, full Tauri app structure
  - **Verification**: `pnpm --filter guidance dev` opens app window

- [ ] **4.2**: Scaffold Knowledge app (Tauri 2 + Svelte 5)

  - **Acceptance**: App with Tauri 2.9.3+, Svelte 5 frontend, basic routing, launches successfully
  - **Files**: `apps/knowledge/package.json`, `apps/knowledge/src/`, `apps/knowledge/src-tauri/`, full Tauri app structure
  - **Verification**: `pnpm --filter knowledge dev` opens app window

- [ ] **4.3**: Scaffold Tracking app (Tauri 2 + Svelte 5)

  - **Acceptance**: App with Tauri 2.9.3+, Svelte 5 frontend, basic routing, launches successfully
  - **Files**: `apps/tracking/package.json`, `apps/tracking/src/`, `apps/tracking/src-tauri/`, full Tauri app structure
  - **Verification**: `pnpm --filter tracking dev` opens app window

- [ ] **4.4**: Scaffold Mobile app (Tauri Android)

  - **Acceptance**: Tauri 2 Android app with Svelte 5 frontend, builds APK
  - **Files**: `apps/mobile/package.json`, `apps/mobile/src/`, `apps/mobile/src-tauri/`, Android configs
  - **Verification**: `pnpm --filter mobile build` produces APK (or `dev` opens in emulator)

- [ ] **4.5**: Configure tauri-specta for bindings generation

  - **Acceptance**: Build script generates TypeScript bindings to `packages/bindings/src/`
  - **Files**: Update `apps/*/src-tauri/build.rs`, `packages/bindings/src/generated.ts`
  - **Verification**: Build produces `generated.ts` with command types

- [ ] **4.6**: Wire src-tauri as Cargo workspace members
  - **Acceptance**: Each app's `src-tauri/Cargo.toml` depends on backend crates, all part of workspace
  - **Files**: `backend/Cargo.toml` (add members), `apps/*/src-tauri/Cargo.toml` (add dependencies)
  - **Verification**: `cargo check` in workspace resolves all app backends

**Phase Exit Criteria**: `pnpm dev` starts all apps; `pnpm build` produces desktop binaries and Android APK

---

## Phase 5: Verification & Documentation

- [ ] **5.1**: Verify fresh clone + install + dev workflow

  - **Acceptance**: New clone, `pnpm install`, all dev servers start without errors
  - **Verification**: Manual test in clean directory
  - **Command**: `git clone <repo> test-clone && cd test-clone && pnpm install && pnpm dev`

- [ ] **5.2**: Verify build produces all artifacts

  - **Acceptance**: Desktop binaries in `apps/*/src-tauri/target/`, APK in `apps/mobile/src-tauri/gen/android/`
  - **Verification**: Check filesystem after `pnpm build`
  - **Command**: `pnpm build && find . -name "*.AppImage" -o -name "*.dmg" -o -name "*.msi" -o -name "*.apk"`

- [ ] **5.3**: Verify pre-commit blocks bad formatting

  - **Acceptance**: Committing unformatted code triggers prek and fails
  - **Verification**: Stage intentionally bad file, attempt commit
  - **Command**: `echo "bad()" > test.ts && git add test.ts && git commit -m "test"`

- [ ] **5.4**: Verify Turborepo cache reduces rebuild time

  - **Acceptance**: Second build after no changes is < 10 seconds
  - **Verification**: Time comparison: `time pnpm build` (twice)
  - **Command**: `pnpm build && pnpm build` (second should show cache hits)

- [ ] **5.5**: Update docs/technical-architecture.md if needed
  - **Acceptance**: Architecture doc reflects actual monorepo structure
  - **Verification**: Review doc against implementation, update if outdated
  - **Files**: `docs/technical-architecture.md`

**Phase Exit Criteria**: All success criteria from `spec.md` pass

---

## Success Criteria Summary (from spec.md)

All tasks above support these criteria:

1. ✅ **Fresh clone workflow**: `git clone → pnpm install → pnpm dev` works
2. ✅ **All apps launch**: Guidance, Knowledge, Tracking, Mobile all start
3. ✅ **Build artifacts**: Desktop binaries and Android APK produced
4. ✅ **Pre-commit hooks**: Linting and formatting enforced
5. ✅ **Turborepo caching**: Rebuilds faster than initial builds
6. ✅ **Type generation**: tauri-specta produces bindings in `packages/bindings/`
7. ✅ **Shared packages**: UI components importable across apps

---

## Task Completion Checklist

**Before starting implementation:**

- [ ] Read `specs/core-001-project-setup/spec.md` for full context
- [ ] Review constitution check (Phase 0 of plan)
- [ ] Ensure Node.js 24+, pnpm 10+, Rust 1.91.1+ installed

**During implementation:**

- [ ] Complete tasks in phase order (Phase 1 → 2 → 3 → 4 → 5)
- [ ] Mark tasks complete as you go
- [ ] Run phase exit criteria after each phase
- [ ] Commit after each phase with message format: `feat(core-001): <description>`

**After implementation:**

- [ ] Run all verification tasks (Phase 5)
- [ ] Update CLAUDE.md if structure differs from spec
- [ ] Commit final changes: `feat(core-001): Complete monorepo setup`
- [ ] Create PR from `spec/core-001-project-setup` → `main`
