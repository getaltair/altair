---
type: prior-art
context: altair-v0.1
---

# Prior Art

| Candidate | Source | Verdict | Notes |
|---|---|---|---|
| Bun | https://bun.sh/docs/installation | Adopt | Bun 1.4.2 is installed for frontend dependency management and scripts. MIT licensed. |
| Naive UI | https://www.naiveui.com and https://www.npmjs.com/package/naive-ui | Adopt | Vue 3 and TypeScript component library with tree-shakable components and typed theme overrides. Version 2.45.3 was current during review; MIT licensed. Touch and accessibility behavior still require application-level verification. |
| XIcons Ionicons 5 | https://www.xicons.org and https://www.npmjs.com/package/@vicons/ionicons5 | Adopt | Use `@vicons/ionicons5` rather than installing unrelated icon sets. Version 0.13.0 was current during review; MIT licensed. Icons require accessible labels when meaning is not also expressed in text. |
| CodeMirror Vim | https://www.npmjs.com/package/@replit/codemirror-vim | Adopt | CodeMirror 6-compatible Vim keymap maintained by Replit; MIT licensed. Keep a discoverable disable path. |
| markdown-it | https://www.npmjs.com/package/markdown-it | Adopt | Mature CommonMark-oriented renderer; MIT licensed. Rendered HTML remains untrusted. |
| DOMPurify | https://www.npmjs.com/package/dompurify | Compose | Maintained HTML sanitizer with TypeScript support; Apache-2.0/MPL-2.0. Compose with markdown-it and test unsafe HTML and URL schemes. |
| Rust openidconnect | https://crates.io/crates/openidconnect | Adopt | Typed standards-focused OIDC/OAuth2 client; MIT licensed. Use only the authorization-code, PKCE, state, and nonce subset required by Authentik. |
| tower-sessions | https://crates.io/crates/tower-sessions | Adopt | Tower/Axum-compatible server session middleware; MIT licensed. Persist opaque server-owned sessions rather than provider tokens in the browser. |
| reqwest | https://crates.io/crates/reqwest | Adopt | Maintained async Rust HTTP client; MIT/Apache-2.0. Restrict it to bounded Garage-independent inference/OIDC infrastructure adapters. |
| Vue, Vite, Pinia | https://vuejs.org, https://vite.dev, https://pinia.vuejs.org | Compose | Maintained Vue 3 client foundation with TypeScript support; MIT licensed. Use Pinia only for shared client state. |
| TypeScript | https://www.typescriptlang.org | Adopt | Maintained strict typing toolchain; Apache-2.0 licensed and compatible with Bun/Vite. |
| CodeMirror 6 | https://codemirror.net | Adopt | Modular browser editor selected by the accepted build; MIT licensed. Keep authored Markdown independent of editor state. |
| Vitest and Vue Test Utils | https://vitest.dev and https://test-utils.vuejs.org | Compose | Vue/Vite-compatible unit and component testing; MIT licensed. |
| Playwright | https://playwright.dev | Adopt | Maintained cross-browser workflow testing; Apache-2.0 licensed. Use for critical desktop and Android-sized journeys. |
| Axum and Tokio | https://docs.rs/axum and https://tokio.rs | Compose | Maintained async Rust HTTP stack; MIT licensed. Keep domain behavior outside framework types. |
| SQLx | https://docs.rs/sqlx | Adopt | Compile-time checked asynchronous SQL for PostgreSQL; MIT/Apache-2.0 licensed. Use explicit migrations and transactions. |
| PostgreSQL, pg_trgm, pgvector | https://www.postgresql.org/docs, https://www.postgresql.org/docs/current/pgtrgm.html, https://github.com/pgvector/pgvector | Compose | Accepted authoritative persistence and retrieval stack under PostgreSQL-compatible licenses. Indexes remain derived data. |
| Authentik | https://goauthentik.io | Adopt | Selected self-hosted OIDC provider; MIT licensed. Browser tokens remain behind server-owned sessions. |
| Garage | https://garagehq.deuxfleurs.fr | Adopt | Selected S3-compatible object store; AGPL-3.0 licensed. Use immutable replacement keys and PostgreSQL-owned metadata. |
| just | https://just.systems | Adopt | Cross-platform command runner with CC0-1.0 licensing; supplies the stable project command interface. |

Exact compatible versions are selected through Bun or Cargo during each story;
package-manager resolution and security advisories are reviewed before commit.

No application code or existing component library is present to extend.
