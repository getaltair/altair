# ADR-004: AI Provider Adapter Pattern

| Field             | Value                                                   |
| ----------------- | ------------------------------------------------------- |
| **Status**        | Superseded                                              |
| **Date**          | 2026-01-08                                              |
| **Deciders**      | Robert Hamilton                                         |
| **Superseded by** | [ADR-006](./006-server-centralized-ai.md) (Jan 9, 2026) |

> **Note**: This ADR described local-first AI with desktop-bundled models. With the shift to Kotlin
> Multiplatform and mobile support (ADR-001), AI services have been centralized on the self-hosted
> server. See ADR-006 for the current architecture. The adapter pattern concept remains relevant but
> is now implemented server-side.

## Context

Altair uses AI for several features:

- **Quest breakdown**: "Help me break this Epic into Quests" (FR-G-101)
- **Note summarization**: Auto-generate note summaries (FR-K-132)
- **Semantic search**: Vector embeddings for similarity (FR-K-112)
- **Voice transcription**: Convert audio to text (FR-K-094)
- **Auto-discovery**: Suggest relationships between entities (FR-K-125)

Users have different preferences and constraints:

- Some want **local-only** (privacy, offline, no API costs)
- Some prefer **cloud providers** (better quality, no GPU required)
- Corporate users may have **approved vendor lists**

We needed an architecture that supports multiple AI providers without coupling features to specific
implementations.

## Decision

Implement an **adapter pattern** with a common trait interface for AI capabilities. At runtime,
users select their preferred provider(s) from officially supported options:

| Provider               | Type  | Use Cases                                                 |
| ---------------------- | ----- | --------------------------------------------------------- |
| **ort (ONNX Runtime)** | Local | Default for embeddings; privacy, offline, no API costs    |
| **Whisper.cpp**        | Local | Default for transcription; privacy, offline, no API costs |
| **Ollama**             | Local | Completion and chat; privacy-focused, offline             |
| **Anthropic**          | Cloud | High-quality reasoning, Claude models                     |
| **OpenAI**             | Cloud | GPT models, embeddings, transcription                     |
| **OpenRouter**         | Cloud | Access to many providers via single API                   |

The adapter interface abstracts capabilities (completion, embedding, transcription), not specific
models.

### Default Configuration

**Embeddings are local by default** using `ort` with bundled ONNX models (e.g., `all-MiniLM-L6-v2`
or `nomic-embed-text`). This ensures:

- Privacy: User data never leaves their machine for semantic search
- Offline: Auto-discovery and semantic search work without internet
- Cost: No per-token API charges for high-frequency embedding operations

**Transcription is local by default** using `whisper.cpp` (Rust bindings via `whisper-rs`). This
ensures:

- Privacy: Voice notes and audio never leave the user's machine
- Offline: Transcription works without internet
- Cost: No per-minute API charges

Users can opt into cloud providers if they prefer higher quality and accept the tradeoffs.

## Consequences

### Positive

- **Privacy by default**: Embeddings stay local; user data not sent to cloud
- **User choice**: Privacy advocates use local; others can opt into cloud
- **Graceful degradation**: If cloud provider fails, local fallback available
- **Future-proof**: New providers added without changing feature code
- **Testable**: Mock adapters for unit tests
- **Cost control**: Users can choose based on pricing

### Negative

- **Bundled model size**: ONNX models add to application size (~50-100MB)
- **Configuration complexity**: Users must understand provider differences
- **Testing burden**: Must test against all supported providers
- **Quality variance**: Local models may produce different results than cloud

### Neutral

- Need to define clear capability requirements for each AI feature
- May need provider-specific prompt engineering for best results

## Design Details

### Trait Interface

```rust
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Generate text completion
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;

    /// Generate embeddings for semantic search
    async fn embed(&self, texts: Vec<String>) -> Result<Vec<Embedding>>;

    /// Transcribe audio to text
    async fn transcribe(&self, audio: AudioData) -> Result<Transcription>;

    /// Check which capabilities this provider supports
    fn capabilities(&self) -> ProviderCapabilities;
}

pub struct ProviderCapabilities {
    pub completion: bool,
    pub embedding: bool,
    pub transcription: bool,
    pub vision: bool,
}
```

### Default Provider Selection

```rust
pub struct AiService {
    completion_provider: Arc<dyn AiProvider>,
    embedding_provider: Arc<dyn AiProvider>,
    transcription_provider: Arc<dyn AiProvider>,
}

impl Default for AiService {
    fn default() -> Self {
        Self {
            // Completion requires user configuration (no sensible default)
            completion_provider: Arc::new(UnconfiguredProvider::new()),
            // Embeddings default to local ONNX
            embedding_provider: Arc::new(OrtEmbeddingProvider::default()),
            // Transcription defaults to local Whisper
            transcription_provider: Arc::new(WhisperProvider::default()),
        }
    }
}
```

### Local Embedding with ort (Default)

```rust
pub struct OrtEmbeddingProvider {
    session: ort::Session,
    tokenizer: Tokenizer,
}

impl Default for OrtEmbeddingProvider {
    fn default() -> Self {
        // Load bundled all-MiniLM-L6-v2 ONNX model
        let model_bytes = include_bytes!("../models/all-MiniLM-L6-v2.onnx");
        let session = ort::Session::builder()
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .commit_from_memory(model_bytes)
            .expect("Failed to load bundled embedding model");
        // ...
    }
}
```

### Fallback Chain

```rust
impl AiService {
    pub async fn embed_with_fallback(&self, texts: Vec<String>) -> Result<Vec<Embedding>> {
        // Local embedding should always work, but if user configured cloud primary:
        if let Ok(embeddings) = self.embedding_provider.embed(texts.clone()).await {
            return Ok(embeddings);
        }

        // Fall back to local if cloud fails
        if let Some(fallback) = &self.local_embedding_fallback {
            return fallback.embed(texts).await;
        }

        Err(AiError::AllProvidersFailed)
    }
}
```

## Officially Supported Providers

### ort / ONNX Runtime (Local) — Default for Embeddings

- **Embedding**: Bundled `all-MiniLM-L6-v2` (384 dimensions) or user-provided ONNX models
- **Completion**: Not supported
- **Transcription**: Not supported

**Pros**: Privacy, no API costs, works offline, fast inference **Cons**: Lower quality than cloud
embeddings; adds to binary size

### Whisper.cpp (Local) — Default for Transcription

- **Embedding**: Not supported
- **Completion**: Not supported
- **Transcription**: Bundled `whisper-small` or user-provided models (tiny, base, small, medium,
  large)

**Pros**: Privacy, no API costs, works offline, good accuracy with small model **Cons**: Adds to
binary size (~150MB for small model); slower than cloud on CPU-only machines

### Ollama (Local)

- **Completion**: Any model (llama3.2, mistral, qwen2.5, etc.)
- **Embedding**: nomic-embed-text, all-minilm, mxbai-embed-large, etc.
- **Transcription**: Not supported (use Whisper.cpp)

**Pros**: Privacy, no API costs, works offline, wide model selection **Cons**: Requires
installation; benefits from GPU; user manages models

### Anthropic (Cloud)

- **Completion**: Claude Opus 4.5, Claude Sonnet 4.5, Claude Haiku 4.5
- **Embedding**: Not offered (use local or OpenAI)
- **Transcription**: Not offered

**Pros**: Highest quality reasoning; excellent instruction following **Cons**: API costs; no
embeddings or transcription

### OpenAI (Cloud)

- **Completion**: GPT-5, GPT-5 mini, GPT-4.1 (for lower cost)
- **Embedding**: text-embedding-3-small (1536 dim), text-embedding-3-large (3072 dim)
- **Transcription**: gpt-4o-transcribe, gpt-4o-mini-transcribe

**Pros**: Full-featured; high-quality embeddings; good transcription **Cons**: API costs; privacy
concerns for some users

### OpenRouter (Cloud)

- **Completion**: Access to Claude, GPT-5, Llama, Mistral, Gemini, etc.
- **Embedding**: Via supported models
- **Transcription**: Limited

**Pros**: Single API for many providers; can switch models easily; unified billing **Cons**:
Additional abstraction layer; varying quality by model

## Alternatives Considered

### Alternative 1: Cloud Embeddings by Default

Default to OpenAI text-embedding-3-small for embeddings.

**Rejected because:**

- Privacy: User notes and queries sent to cloud by default
- Cost: Embedding is high-frequency (auto-discovery runs on every note edit)
- Offline: Semantic search would fail without internet
- Local-first philosophy: Contradicts Altair's core principle

### Alternative 2: Single Provider (OpenAI Only)

Just use OpenAI for everything.

**Rejected because:**

- Excludes privacy-conscious users
- No offline capability
- Vendor lock-in

### Alternative 3: Local Only (Ollama/ort)

Only support local inference.

**Rejected because:**

- Requires capable hardware (GPU) for completions
- Quality may not meet user expectations for complex reasoning
- Complex setup for non-technical users

### Alternative 4: Plugin-Based Providers

Let third parties implement provider plugins.

**Rejected because:**

- Security concerns (API keys handled by untrusted code)
- Support burden for provider-specific issues
- Can add later if demand exists

## References

- [ort crate](https://github.com/pykeio/ort) — ONNX Runtime for Rust
- [whisper-rs](https://github.com/tazz4843/whisper-rs) — Rust bindings for whisper.cpp
- [Ollama](https://ollama.ai/) — Local model runner
- [Anthropic Models](https://docs.anthropic.com/en/docs/about-claude/models) — Claude model
  documentation
- [OpenAI Models](https://platform.openai.com/docs/models) — GPT and embedding models
- FR-G-101 through FR-G-105: Guidance AI requirements
- FR-K-131 through FR-K-137: Knowledge AI requirements
