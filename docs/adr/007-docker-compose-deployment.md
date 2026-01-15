# ADR-007: Docker Compose Deployment

| Field        | Value           |
| ------------ | --------------- |
| **Status**   | Accepted        |
| **Date**     | 2026-01-09      |
| **Deciders** | Robert Hamilton |

## Context

Altair's self-hosted server architecture (ADR-001) requires a deployment strategy that:

1. Works for non-technical users who want self-hosting simplicity
2. Bundles all required services (application server, database, optional AI)
3. Runs on typical VPS or home server hardware
4. Supports easy updates and backup

The target users are technically capable enough to run `docker compose up -d` but shouldn't need to
configure Kubernetes or manage multiple separate services manually.

## Decision

Deploy the Altair server stack using **Docker Compose** with three services:

```yaml
version: "3.8"
services:
  altair-server:
    image: ghcr.io/getaltair/altair-server:latest
    ports:
      - "8080:8080" # HTTP/gRPC
    environment:
      - DATABASE_URL=surrealdb://surrealdb:8000
      - OLLAMA_URL=http://ollama:11434 # Optional
    depends_on:
      - surrealdb

  surrealdb:
    image: surrealdb/surrealdb:latest
    user: root
    command: start --user root --pass changeme surrealkv:/data/altair.db
    volumes:
      - surrealdb_data:/data

  ollama: # Optional, for local completion
    image: ollama/ollama:latest
    volumes:
      - ollama_models:/root/.ollama
    profiles:
      - with-ollama # Only starts with --profile with-ollama

volumes:
  surrealdb_data:
  ollama_models:
```

Users run:

- `docker compose up -d` — Server + SurrealDB only
- `docker compose --profile with-ollama up -d` — Includes Ollama for local AI completion

## Consequences

### Positive

- **Single command deployment**: `docker compose up -d` starts everything
- **Isolated dependencies**: No system-level installs for SurrealDB or Ollama
- **Easy updates**: `docker compose pull && docker compose up -d`
- **Portable**: Works on any Docker-capable host (VPS, NAS, Raspberry Pi 4+, etc.)
- **Backup simplicity**: Volume mounts are the only stateful data
- **Optional Ollama**: Users can skip local AI if using cloud providers

### Negative

- **Docker required**: Users must have Docker and Docker Compose installed
- **Resource overhead**: Container runtime adds ~50-100MB memory overhead
- **Complexity for scaling**: Docker Compose is single-host; no built-in HA
- **Networking**: Users may need to configure reverse proxy for HTTPS

### Neutral

- Default configuration works for family-scale usage (2-5 users, 5-15 devices)
- Production deployments can add Traefik/Nginx for TLS termination

## Resource Requirements

### Minimum (without Ollama)

- 1 vCPU
- 1GB RAM
- 10GB storage

### Recommended (with Ollama)

- 2+ vCPU
- 4GB RAM (8GB for larger models)
- 50GB storage (models can be large)
- GPU passthrough optional but helpful

## Service Details

### altair-server

Ktor application serving:

- kotlinx-rpc endpoints for sync, AI, and auth
- Embedded all-MiniLM-L6-v2 for embeddings
- Embedded whisper-small for transcription
- Health check endpoint at `/health`

Built as a single JAR with embedded JVM runtime using jlink.

### surrealdb

Official SurrealDB image running in file-backed mode. Stores all user data, sync state, and vector
embeddings.

### ollama (optional)

Official Ollama image for local LLM inference. Users can pull models like `llama3.1:8b` or
`mistral:7b` for completion tasks without cloud API dependency.

## Alternatives Considered

### Alternative 1: Kubernetes / Helm

Container orchestration for cloud-native deployment.

**Rejected because:**

- Overkill for single-user/family self-hosting
- Steep learning curve for non-DevOps users
- Most users don't have Kubernetes clusters at home

### Alternative 2: Native Binaries

Distribute compiled binaries for each OS.

**Rejected because:**

- Must bundle SurrealDB separately or as subprocess
- OS-specific packaging (deb, rpm, msi, etc.)
- Dependency hell for native libraries (whisper.cpp, ONNX)
- Updates harder to manage

### Alternative 3: Snap / Flatpak

Linux sandboxed application formats.

**Rejected because:**

- Linux-only; doesn't help Windows/macOS server users
- Sandboxing complicates database file access
- Less familiar than Docker for self-hosters

### Alternative 4: Cloud Hosting Only

Offer managed hosting instead of self-hosting.

**Rejected because:**

- Core requirement is self-hosted for privacy and data ownership
- Could be offered as additional option, not replacement

## References

- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [SurrealDB Docker Image](https://hub.docker.com/r/surrealdb/surrealdb)
- [Ollama Docker Image](https://hub.docker.com/r/ollama/ollama)
- [ADR-001: Kotlin Multiplatform Architecture](./001-single-tauri-application.md)
- [ADR-006: Server-Centralized AI Services](./006-server-centralized-ai.md)
