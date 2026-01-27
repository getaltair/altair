# ADR-006: Server-Centralized AI Services

| Field          | Value                                         |
| -------------- | --------------------------------------------- |
| **Status**     | Accepted                                      |
| **Date**       | 2026-01-09                                    |
| **Deciders**   | Robert Hamilton                               |
| **Supersedes** | ADR-004 (AI Provider Adapters, local-default) |

## Context

Altair uses AI for three capabilities:

1. **Embeddings**: Semantic search, auto-discovery, similar note finding
2. **Transcription**: Voice note capture, audio-to-text
3. **Completion**: AI-assisted writing, summarization, query answering

Originally, ADR-004 specified desktop-bundled AI (ONNX embeddings, Whisper transcription bundled
with the desktop app). With the shift to mobile support and a self-hosted server architecture
(ADR-001), we need to reconsider where AI processing happens.

## Decision

**Centralize AI services on the self-hosted server**:

| Capability    | Location | Implementation                                           |
| ------------- | -------- | -------------------------------------------------------- |
| Embeddings    | Server   | all-MiniLM-L6-v2 via ort (Kotlin ONNX Runtime)           |
| Transcription | Server   | whisper-small via whisper.cpp bindings                   |
| Completion    | Server   | Proxy to user-configured provider (Ollama, OpenAI, etc.) |

Clients (desktop and mobile) send requests to the server's AI service endpoints. The server performs
inference locally for embeddings/transcription and proxies completion requests to the configured
provider.

**Desktop offline fallback**: When the server is unreachable, desktop clients can optionally run
local embedding and transcription models. Mobile clients require server connectivity for AI
features.

## Consequences

### Positive

- **Mobile simplicity**: No 50-100MB model bundles on mobile; thin client, server does heavy lifting
- **Instant model updates**: Update models server-side; all clients benefit immediately
- **Better hardware**: Server can have GPU; mobile/laptop battery not drained
- **Consistent results**: Same model version across all clients; no version skew
- **Completion flexibility**: Server can run Ollama with local GPU or proxy to cloud providers
- **Privacy preserved**: Data transits to user's own server, not third-party cloud

### Negative

- **Server required for AI**: Mobile AI features need connectivity (acceptable for "quick capture"
  scope)
- **Latency**: Network round-trip for each AI request (mitigated by batching embeddings)
- **Server resources**: Self-hosted server needs sufficient RAM/CPU for inference
- **Complexity**: Server must handle concurrent AI requests; may need queuing

### Neutral

- Desktop can cache embedding results locally to reduce server calls
- Transcription typically done immediately after recording; latency acceptable

## AI Service Architecture

### Server Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Altair Server (Ktor)                     │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                   AiService (kotlinx-rpc)              │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌────────────────┐  │ │
│  │  │  Embedding  │  │Transcription│  │   Completion   │  │ │
│  │  │   Provider  │  │   Provider  │  │     Router     │  │ │
│  │  └──────┬──────┘  └──────┬──────┘  └───────┬────────┘  │ │
│  └─────────┼────────────────┼─────────────────┼───────────┘ │
│            │                │                 │             │
│  ┌─────────▼──────┐  ┌──────▼──────┐  ┌───────▼────────┐   │
│  │ all-MiniLM-L6  │  │whisper-small│  │ Provider API   │   │
│  │   (ONNX/ort)   │  │(whisper.cpp)│  │ (Ollama/Cloud) │   │
│  └────────────────┘  └─────────────┘  └────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Completion Providers

Users configure their preferred completion provider in server settings:

| Provider   | Use Case                             | Configuration            |
| ---------- | ------------------------------------ | ------------------------ |
| Ollama     | Local GPU inference, privacy-focused | `http://localhost:11434` |
| Anthropic  | Claude models (Opus, Sonnet, Haiku)  | API key                  |
| OpenAI     | GPT models                           | API key                  |
| OpenRouter | Multi-provider access                | API key                  |

### Request Flow

```
Mobile/Desktop Client
        │
        │ kotlinx-rpc (gRPC)
        ▼
   Altair Server
        │
        ├─► Embeddings: Process locally (ort)
        │
        ├─► Transcription: Process locally (whisper.cpp)
        │
        └─► Completion: Route to configured provider
                │
                ├─► Ollama (local): HTTP to localhost:11434
                └─► Cloud: HTTPS to provider API
```

## Alternatives Considered

### Alternative 1: Local-Only AI (Original ADR-004)

Bundle models in desktop app; no server AI.

**Rejected because:**

- Mobile would need 50-100MB model bundles or no AI features
- Model updates require app releases
- No completion capability without separate cloud config per client
- Battery drain on laptops during inference

### Alternative 2: Cloud-Only AI

Use only cloud providers (OpenAI, Anthropic) for all AI.

**Rejected because:**

- Privacy: User data transits third-party servers
- Cost: Embedding/transcription costs add up for heavy users
- Dependency: Requires internet and API keys for basic features

### Alternative 3: Hybrid Client-Side

Desktop runs local models; mobile proxies through desktop.

**Rejected because:**

- Requires desktop to be online when mobile needs AI
- Complex peer-to-peer networking
- Battery drain on desktop when acting as mobile's AI backend

## References

- [ADR-001: Kotlin Multiplatform Architecture](./001-single-tauri-application.md)
- [ADR-005: kotlinx-rpc Communication](./005-kotlinx-rpc-communication.md)
- [ADR-007: Docker Compose Deployment](./007-docker-compose-deployment.md)
- [all-MiniLM-L6-v2 Model](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2)
- [whisper.cpp](https://github.com/ggerganov/whisper.cpp)
- FR-K-125: Semantic search
- FR-T-038: Voice capture and transcription
