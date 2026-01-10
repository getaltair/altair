# Desktop Application Background Service Architectures

**Research conducted: January 2026**

> Analysis of how popular desktop applications implement background services, offline sync, and
> inter-process communication.

---

## Executive Summary

Modern desktop applications employ surprisingly diverse approaches to background services, ranging
from no separate daemon at all to sophisticated multi-process architectures with custom IPC
protocols. **The dominant pattern is not a traditional system daemon, but rather tightly integrated
child processes communicating via Unix sockets, named pipes, or JSON-RPC over stdio.**

### Key Finding

Most local-first desktop applications **do not use traditional background daemons** in the
systemd/launchd sense. Instead, they rely on:

- In-process sync engines (Obsidian, Notion, Linear)
- Child processes spawned by the main application (Raycast, VS Code)
- Small helper utilities that run only when needed (Figma)

---

## Application Analysis

### Obsidian: In-process sync with no background service

Obsidian Sync runs as a **core plugin within the main Electron process**—there is no separate
daemon.

| Aspect              | Implementation                                   |
| ------------------- | ------------------------------------------------ |
| Transport           | WebSocket Secure (WSS) to regional servers       |
| Conflict resolution | Google's diff-match-patch algorithm (not CRDT)   |
| Local storage       | Plain UTF-8 Markdown files with YAML frontmatter |
| Encryption          | AES-256-GCM with scrypt key derivation           |

**Key insight**: When Obsidian is closed, no sync occurs. Changes queue locally until the app
reopens.

---

### 1Password: Platform-native IPC with code signature verification

1Password 8 uses a **unified Rust core library** across all platforms, with the most sophisticated
multi-client architecture studied.

| Platform | IPC Mechanism                                                       |
| -------- | ------------------------------------------------------------------- |
| macOS    | NSXPCConnection (XPC)                                               |
| Linux    | Unix domain socket (`/run/user/$UID/1Password-BrowserSupport.sock`) |
| Windows  | Named pipes with Authenticode verification                          |

**Key insight**: The desktop app verifies browser authenticity by checking code signatures before
accepting connections.

---

### Notion: WebSocket sync with WASM SQLite caching

Notion evolved from cloud-first to "offline-capable" with CRDTs specifically for offline mode.

| Platform       | Local Cache                                       |
| -------------- | ------------------------------------------------- |
| Desktop/Mobile | Native SQLite                                     |
| Browser        | WebAssembly SQLite via Origin Private File System |

**Key insight**: Browser implementation uses SharedWorker pattern—only one "active tab" can write to
SQLite.

---

### Raycast: Native app with out-of-process Node.js extension runtime

Raycast is a **fully native macOS application** (Swift/AppKit) with extensions running in a single
Node.js child process.

| Aspect                        | Implementation                                 |
| ----------------------------- | ---------------------------------------------- |
| Extension ↔ App communication | JSON-RPC over stdio                            |
| UI rendering                  | Custom React reconciler → JSON → native AppKit |
| Process isolation             | Node crash doesn't affect main UI              |

**Key insight**: Extensions run in V8 isolates (worker threads) within a single Node.js process.

---

### Linear: Custom sync engine with IndexedDB persistence

Linear built a homegrown sync engine (LSE) enabling offline-first with real-time collaboration.

| Aspect              | Implementation                                             |
| ------------------- | ---------------------------------------------------------- |
| Bootstrap           | Full state via `/sync/bootstrap` as newline-delimited JSON |
| Real-time           | WebSockets push SyncActions with incremental sync IDs      |
| Conflict resolution | Last-Writer-Wins at property level                         |

**Key insight**: "Conflicts are actually not that common in Linear"—LWW at property level avoids
most issues.

---

### Figma: Local HTTP font server with multiplayer sync

Figma runs **FigmaAgent**, the closest to a traditional persistent daemon.

| Aspect        | Implementation                                  |
| ------------- | ----------------------------------------------- |
| Font server   | HTTP on localhost:44950                         |
| Multiplayer   | One process per document, WebSocket connections |
| Checkpointing | Every 30-60 seconds to S3                       |

**Key insight**: On Linux, FigmaAgent runs as a systemd user service.

---

### Docker Desktop: VM-based architecture with socket communication

Docker Desktop runs a lightweight LinuxKit-based VM to host the Docker daemon.

| Platform              | VM Backend                        | Socket      |
| --------------------- | --------------------------------- | ----------- |
| macOS (Apple Silicon) | Docker VMM / Apple Virtualization | Unix socket |
| macOS (Intel)         | Apple Virtualization / HyperKit   | Unix socket |
| Windows               | WSL 2 or Hyper-V                  | Named pipe  |
| Linux                 | KVM                               | Unix socket |

---

### VS Code: Multi-process extension architecture with LSP

VS Code uses the most complex multi-process architecture with separate extension hosts and language
servers.

| Transport   | Use Case                                |
| ----------- | --------------------------------------- |
| stdio       | Most common for language servers        |
| sockets     | TCP/IP for remote scenarios             |
| named pipes | Windows IPC, Unix domain sockets        |
| node IPC    | When both client and server are Node.js |

**Key insight**: Each VS Code window spawns its own extension host process.

---

## IPC Mechanisms by Platform

| Platform       | Recommended                                     | Alternative   |
| -------------- | ----------------------------------------------- | ------------- |
| macOS          | XPC (system integration), Unix sockets (simple) | Named pipes   |
| Windows        | Named pipes                                     | TCP localhost |
| Linux          | Unix domain sockets                             | Named pipes   |
| Cross-platform | JSON-RPC over stdio or WebSocket                | gRPC          |

---

## Architectural Patterns Summary

| Pattern                         | Examples                 | When to Use                          |
| ------------------------------- | ------------------------ | ------------------------------------ |
| **No daemon** (in-process sync) | Obsidian, Notion, Linear | Single-app local-first               |
| **Child process**               | Raycast, VS Code         | Extensions/plugins needing isolation |
| **Persistent helper**           | Figma, 1Password         | Multi-client coordination            |
| **VM-based**                    | Docker Desktop           | Heavy isolation requirements         |

---

## Recommendations for Altair

Based on this research:

1. **Kotlin Multiplatform with embedded database** — Similar to Obsidian's pattern: self-contained
   app with local data
2. **Self-hosted server for sync** — Cleaner than peer-to-peer; WebSocket/gRPC for real-time updates
3. **Offline-capable desktop** — SurrealDB embedded enables full functionality without server
4. **Server-dependent mobile** — Quick capture scope means simpler architecture is acceptable

> **Note**: This research predates the pivot from Tauri to Kotlin Multiplatform. The core insights
> about avoiding traditional daemons and using in-process architectures still apply.

---

## References

- [Obsidian Sync](https://obsidian.md/sync)
- [1Password Developer Docs](https://developer.1password.com/docs/cli/app-integration-security/)
- [Notion WASM SQLite Blog](https://www.notion.com/blog/how-we-sped-up-notion-in-the-browser-with-wasm-sqlite)
- [Raycast Extension Architecture](https://www.raycast.com/blog/how-raycast-api-extensions-work)
- [Linear Sync Engine Reverse Engineering](https://marknotfound.com/posts/reverse-engineering-linears-sync-magic/)
- [Figma Multiplayer Blog](https://www.figma.com/blog/how-figmas-multiplayer-technology-works/)
- [VS Code Sandboxing](https://code.visualstudio.com/blogs/2022/11/28/vscode-sandbox)
