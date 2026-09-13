# Technical Stack

`docs/initial-build.md` is the source of truth for the accepted technical foundation.

| Area | Selection |
|---|---|
| Client | Vue 3, TypeScript, Vite, Pinia |
| Editor | CodeMirror 6 with Vim support |
| Server | Rust and Axum |
| Persistence | SQLx and PostgreSQL 18 |
| Retrieval | PostgreSQL full-text search, pg_trgm, and pgvector |
| Identity | Authentik through OIDC authorization code flow with PKCE |
| Attachments | Garage S3-compatible storage |
| Inference | Configured OpenAI-compatible `/v1/embeddings` endpoint |

Organize code by product domain. Keep external integrations behind focused infrastructure modules.
