# Tauri 2 Conventions

Applies to: `apps/desktop/`

## Framework

- Tauri 2 wrapping the shared SvelteKit UI from `apps/web/`
- Targets: Linux, Windows
- Rust backend for native capabilities

## Commands

- `#[tauri::command]` handlers must validate all inputs before use
- Return types must implement `serde::Serialize`
- Use `Result<T, E>` return types with meaningful error variants
- Keep command handlers thin; delegate to service modules

## Capabilities & Permissions

- Scope capability files to minimum required permissions
- No overly broad filesystem access; scope to app directories
- CSP headers in `tauri.conf.json` must exist; no `unsafe-inline` for scripts

## IPC

- Verify all IPC payloads are typed on both Rust and TypeScript sides
- Use `invoke` with typed generics, not untyped string payloads
- Window creation URLs must be scoped appropriately

## Shared UI

- The SvelteKit UI in `apps/web/` is the single source for all UI components
- Desktop-specific behavior uses Tauri APIs behind feature detection
- Check `window.__TAURI__` or equivalent before calling Tauri-specific APIs

## File System

- All file paths scoped to app data directories (`app_data_dir`, `app_config_dir`)
- Never access arbitrary filesystem paths without user consent
- Use Tauri's dialog API for file picker interactions

## Security

- `unsafe` blocks require `// SAFETY:` documentation comments
- No `unwrap()` in production code; use `?`, `expect()`, or proper error handling
- Run `cargo audit` regularly
