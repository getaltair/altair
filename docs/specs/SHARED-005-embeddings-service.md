# Feature SHARED-005: Embeddings Service

## What it does

Provides a centralized local embeddings service using Sentence Transformers for semantic search, content similarity, and AI-powered features across all Altair applications with privacy-first, offline-capable text vectorization.

## User Journey

GIVEN any Altair app needs to convert text to vectors for semantic operations
WHEN the app sends text to the embeddings service
THEN it receives high-quality embeddings suitable for similarity search and clustering

## Functional Requirements

- Local sentence transformer models (all-MiniLM-L6-v2 default)
- Multi-language support with language detection
- Batch embedding generation
- Model management and switching
- Embedding cache for performance
- Similarity calculation functions
- Clustering support
- Dimensionality reduction options
- Model download and updates
- Resource monitoring
- Queue management for large batches
- Incremental index updates
- Vector normalization
- Custom model support

## UI/UX Requirements

### Components

```dart
// Embeddings service monitoring components
EmbeddingServiceStatus
ModelSelector
EmbeddingQueueMonitor
ResourceUsageWidget
ModelDownloadProgress
CacheStatistics
PerformanceMetrics
EmbeddingVisualizer
ServiceHealthIndicator
BatchProgressBar
```

### Visual Design

- **Layout:**
  - Status bar indicator: 200px width
  - Model selector dropdown: 280px
  - Queue monitor: floating panel 400x300px
  - Resource graph: 300x200px
  
- **Colors:**
  ```dart
  serviceActive: Color(0xFF10B981), // Green for active
  serviceProcessing: Color(0xFF3B82F6), // Blue for processing
  serviceError: Color(0xFFEF4444), // Red for error
  cacheHit: Color(0xFF6366F1), // Indigo for cache hit
  modelLoading: Color(0xFFF59E0B), // Amber for loading
  ```
  
- **Typography:**
  - Status text: 12px regular
  - Model name: 14px semibold
  - Queue count: 16px monospace
  - Performance metrics: 12px monospace
  
- **Iconography:**
  - Model: brain icon 18px
  - Processing: spinner 16px
  - Cache: database icon 16px
  - Queue: layers icon 16px
  
- **Borders/Shadows:**
  - Status panel: 2px border
  - Active processing: pulsing glow
  - Model selector: 3px on focus

### User Interactions

- **Input Methods:**
  - Automatic service start
  - Model selection via dropdown
  - Clear cache button
  - Queue priority adjustment
  
- **Keyboard Shortcuts:**
  - `Ctrl+Shift+M`: Model switcher
  - `Ctrl+Shift+C`: Clear cache
  - `Ctrl+Shift+Q`: Queue monitor
  
- **Gestures:**
  - Drag to reorder queue
  - Swipe to dismiss notifications
  
- **Feedback:**
  - Processing spinner
  - Cache hit indicator
  - Queue position updates
  - Completion notifications

### State Management

```dart
// Riverpod providers
final embeddingServiceProvider = Provider<EmbeddingService>((ref) {
  return EmbeddingService();
});

final embeddingServiceStatusProvider = StreamProvider<ServiceStatus>((ref) {
  return ref.read(embeddingServiceProvider).statusStream;
});

final currentModelProvider = StateProvider<EmbeddingModel>((ref) {
  return EmbeddingModel.miniLM;
});

final embeddingQueueProvider = StateNotifierProvider<QueueNotifier, List<EmbeddingJob>>(
  (ref) => QueueNotifier(),
);

final embeddingCacheStatsProvider = Provider<CacheStats>((ref) {
  return ref.read(embeddingServiceProvider).getCacheStats();
});

final resourceMonitorProvider = StreamProvider<ResourceUsage>((ref) {
  return ref.read(embeddingServiceProvider).resourceStream;
});
```

### Responsive Behavior

- **Desktop (>1200px):**
  - Full monitoring dashboard
  - Detailed metrics visible
  - Multiple model support
  
- **Tablet (768-1199px):**
  - Simplified status view
  - Essential metrics only
  
- **Mobile (<768px):**
  - Minimal status indicator
  - Background operation only

### Accessibility Requirements

- **Screen Reader:**
  - Service status announced
  - Queue updates communicated
  - Model changes announced
  
- **Keyboard Navigation:**
  - Tab through controls
  - Clear focus indicators
  
- **Color Contrast:**
  - Status indicators accessible
  - Alternative text indicators
  
- **Motion:**
  - Optional processing animations
  - Static status available
  
- **Font Sizing:**
  - Scalable metrics display
  - Clear status text

### ADHD-Specific UI Requirements

- **Cognitive Load:**
  - Automatic operation default
  - Hidden unless issues
  - Smart model selection
  - No manual configuration needed
  
- **Focus Management:**
  - Non-intrusive status
  - Background processing
  - Silent operation
  
- **Forgiveness:**
  - Auto-retry on failure
  - Cache recovery
  - Queue persistence
  
- **Visual Hierarchy:**
  - Clear status indicator
  - Critical errors prominent
  - Performance hidden by default
  
- **Immediate Feedback:**
  - Instant cache hits
  - Quick status updates
  - Real-time queue progress

## Non-Functional Requirements

### Performance Targets

- Embedding generation: <100ms for 512 tokens
- Batch processing: 100 docs/second
- Cache hit rate: >80%
- Model loading: <5 seconds
- Memory usage: <500MB for base model
- Similarity search: <10ms for 10k vectors

### Technical Constraints

- Python service with Rust FFI option
- Model size: 25-250MB depending on quality
- CPU-only operation (no GPU required)
- Cross-platform compatibility
- IPC communication with Flutter

### Security Requirements

- Local-only processing
- No external API calls
- Secure IPC channels
- Model integrity verification
- Cache encryption option

## Implementation Details

### Code Structure

```
// Service structure (Python/Rust hybrid)
embeddings-service/
├── src/
│   ├── models/
│   │   ├── loader.py
│   │   ├── minilm.py
│   │   └── multilingual.py
│   ├── cache/
│   │   ├── lru_cache.py
│   │   └── persistent_cache.py
│   ├── queue/
│   │   ├── job_queue.py
│   │   └── priority_queue.py
│   ├── api/
│   │   ├── grpc_server.py
│   │   └── ipc_server.py
│   └── monitoring/
│       ├── metrics.py
│       └── health_check.py
├── rust_ffi/
│   ├── src/
│   │   ├── lib.rs
│   │   └── embeddings.rs
│   └── Cargo.toml
└── requirements.txt

// Flutter integration
lib/shared/services/embeddings/
├── embedding_service.dart
├── embedding_client.dart
├── models/
│   ├── embedding_model.dart
│   └── embedding_result.dart
└── providers/
    └── embedding_provider.dart
```

### Rust Backend Service

```rust
// rust-backend/services/embeddings/src/lib.rs
use candle_core::{Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;
use std::sync::{Arc, Mutex};
use lru::LruCache;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct EmbeddingService {
    model: Arc<BertModel>,
    tokenizer: Arc<Tokenizer>,
    cache: Arc<Mutex<LruCache<String, Vec<f32>>>>,
    config: EmbeddingConfig,
}

#[derive(Serialize, Deserialize)]
struct EmbeddingConfig {
    model_name: String,
    max_length: usize,
    pooling_strategy: PoolingStrategy,
    normalize: bool,
    cache_size: usize,
}

#[derive(Serialize, Deserialize)]
enum PoolingStrategy {
    Mean,
    Max,
    CLS,
}

impl EmbeddingService {
    pub fn new(model_path: &str) -> Result<Self, Error> {
        // Load model
        let device = Device::Cpu;
        let config = Config::from_file(format!("{}/config.json", model_path))?;
        let model = BertModel::load(&config, format!("{}/model.safetensors", model_path), &device)?;
        
        // Load tokenizer
        let tokenizer = Tokenizer::from_file(format!("{}/tokenizer.json", model_path))?;
        
        // Initialize cache
        let cache = LruCache::new(10000);
        
        Ok(Self {
            model: Arc::new(model),
            tokenizer: Arc::new(tokenizer),
            cache: Arc::new(Mutex::new(cache)),
            config: EmbeddingConfig::default(),
        })
    }
    
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, Error> {
        // Check cache
        if let Some(cached) = self.get_from_cache(text) {
            return Ok(cached);
        }
        
        // Tokenize
        let encoding = self.tokenizer.encode(text, true)?;
        let tokens = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();
        
        // Convert to tensors
        let token_ids = Tensor::from_slice(tokens, &[1, tokens.len()], &Device::Cpu)?;
        let mask = Tensor::from_slice(attention_mask, &[1, attention_mask.len()], &Device::Cpu)?;
        
        // Forward pass
        let output = self.model.forward(&token_ids, &mask)?;
        
        // Pool embeddings
        let pooled = self.pool_embeddings(&output, &mask)?;
        
        // Normalize if configured
        let final_embedding = if self.config.normalize {
            self.normalize_vector(&pooled)?
        } else {
            pooled
        };
        
        // Cache result
        self.cache_embedding(text, &final_embedding);
        
        Ok(final_embedding)
    }
    
    pub async fn embed_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Error> {
        let mut embeddings = Vec::new();
        
        // Process in chunks for memory efficiency
        for chunk in texts.chunks(32) {
            let chunk_embeddings = self.process_chunk(chunk).await?;
            embeddings.extend(chunk_embeddings);
        }
        
        Ok(embeddings)
    }
    
    fn pool_embeddings(&self, output: &Tensor, mask: &Tensor) -> Result<Vec<f32>, Error> {
        match self.config.pooling_strategy {
            PoolingStrategy::Mean => {
                // Mean pooling over sequence length
                let sum = output.sum_keepdim(1)?;
                let count = mask.sum_keepdim(1)?;
                let mean = sum.div(&count)?;
                Ok(mean.to_vec1()?)
            }
            PoolingStrategy::CLS => {
                // Use CLS token embedding
                let cls_embedding = output.i((.., 0, ..))?;
                Ok(cls_embedding.to_vec1()?)
            }
            PoolingStrategy::Max => {
                // Max pooling
                let max = output.max_keepdim(1)?;
                Ok(max.to_vec1()?)
            }
        }
    }
    
    fn normalize_vector(&self, vec: &[f32]) -> Result<Vec<f32>, Error> {
        let norm = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        Ok(vec.iter().map(|x| x / norm).collect())
    }
    
    pub fn cosine_similarity(&self, vec1: &[f32], vec2: &[f32]) -> f32 {
        let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        dot_product / (norm1 * norm2)
    }
    
    fn get_from_cache(&self, text: &str) -> Option<Vec<f32>> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(text).cloned()
    }
    
    fn cache_embedding(&self, text: &str, embedding: &[f32]) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(text.to_string(), embedding.to_vec());
    }
}

// gRPC service implementation
pub mod grpc {
    use tonic::{Request, Response, Status};
    use super::*;
    
    #[derive(Clone)]
    pub struct EmbeddingServiceImpl {
        service: EmbeddingService,
    }
    
    #[tonic::async_trait]
    impl embedding_proto::embedding_service_server::EmbeddingService for EmbeddingServiceImpl {
        async fn embed(
            &self,
            request: Request<embedding_proto::EmbedRequest>,
        ) -> Result<Response<embedding_proto::EmbedResponse>, Status> {
            let req = request.into_inner();
            
            match self.service.embed(&req.text).await {
                Ok(embedding) => {
                    Ok(Response::new(embedding_proto::EmbedResponse {
                        embedding,
                        model: self.service.config.model_name.clone(),
                        cached: false,
                    }))
                }
                Err(e) => Err(Status::internal(e.to_string())),
            }
        }
        
        async fn embed_batch(
            &self,
            request: Request<embedding_proto::EmbedBatchRequest>,
        ) -> Result<Response<embedding_proto::EmbedBatchResponse>, Status> {
            let req = request.into_inner();
            
            match self.service.embed_batch(req.texts).await {
                Ok(embeddings) => {
                    Ok(Response::new(embedding_proto::EmbedBatchResponse {
                        embeddings: embeddings.into_iter().map(|e| {
                            embedding_proto::Embedding { values: e }
                        }).collect(),
                    }))
                }
                Err(e) => Err(Status::internal(e.to_string())),
            }
        }
        
        async fn similarity(
            &self,
            request: Request<embedding_proto::SimilarityRequest>,
        ) -> Result<Response<embedding_proto::SimilarityResponse>, Status> {
            let req = request.into_inner();
            let score = self.service.cosine_similarity(&req.vec1, &req.vec2);
            
            Ok(Response::new(embedding_proto::SimilarityResponse {
                score,
            }))
        }
    }
}
```

### Python Implementation (Alternative)

```python
# embeddings-service/src/models/sentence_transformer.py
from sentence_transformers import SentenceTransformer
from typing import List, Dict, Optional
import numpy as np
from functools import lru_cache
import asyncio
from concurrent.futures import ThreadPoolExecutor

class EmbeddingService:
    def __init__(self, model_name: str = 'all-MiniLM-L6-v2'):
        self.model = SentenceTransformer(model_name)
        self.executor = ThreadPoolExecutor(max_workers=4)
        self.cache = {}
        self.model_name = model_name
        
    @lru_cache(maxsize=10000)
    def _embed_cached(self, text: str) -> np.ndarray:
        """Cache embeddings at the function level"""
        return self.model.encode(text, normalize_embeddings=True)
    
    async def embed(self, text: str) -> List[float]:
        """Generate embedding for single text"""
        loop = asyncio.get_event_loop()
        embedding = await loop.run_in_executor(
            self.executor,
            self._embed_cached,
            text
        )
        return embedding.tolist()
    
    async def embed_batch(self, texts: List[str], batch_size: int = 32) -> List[List[float]]:
        """Generate embeddings for multiple texts"""
        embeddings = []
        
        for i in range(0, len(texts), batch_size):
            batch = texts[i:i + batch_size]
            batch_embeddings = await self._process_batch(batch)
            embeddings.extend(batch_embeddings)
        
        return embeddings
    
    async def _process_batch(self, texts: List[str]) -> List[List[float]]:
        """Process a batch of texts"""
        loop = asyncio.get_event_loop()
        embeddings = await loop.run_in_executor(
            self.executor,
            lambda: self.model.encode(texts, normalize_embeddings=True)
        )
        return embeddings.tolist()
    
    def cosine_similarity(self, vec1: List[float], vec2: List[float]) -> float:
        """Calculate cosine similarity between two vectors"""
        return np.dot(vec1, vec2)
    
    async def semantic_search(
        self,
        query: str,
        corpus_embeddings: List[List[float]],
        top_k: int = 10
    ) -> List[Dict]:
        """Perform semantic search"""
        query_embedding = await self.embed(query)
        
        # Calculate similarities
        similarities = [
            self.cosine_similarity(query_embedding, doc_emb)
            for doc_emb in corpus_embeddings
        ]
        
        # Get top-k results
        top_indices = np.argsort(similarities)[-top_k:][::-1]
        
        return [
            {'index': int(idx), 'score': float(similarities[idx])}
            for idx in top_indices
        ]
```

### gRPC Service Definition

```proto
// protos/embeddings.proto
syntax = "proto3";
package altair.shared.embeddings;

service EmbeddingService {
  rpc Embed(EmbedRequest) returns (EmbedResponse);
  rpc EmbedBatch(EmbedBatchRequest) returns (EmbedBatchResponse);
  rpc Similarity(SimilarityRequest) returns (SimilarityResponse);
  rpc SemanticSearch(SearchRequest) returns (SearchResponse);
  rpc GetStatus(StatusRequest) returns (StatusResponse);
  rpc SwitchModel(SwitchModelRequest) returns (SwitchModelResponse);
  rpc ClearCache(ClearCacheRequest) returns (ClearCacheResponse);
}

message EmbedRequest {
  string text = 1;
  bool use_cache = 2;
  EmbeddingOptions options = 3;
}

message EmbedResponse {
  repeated float embedding = 1;
  string model = 2;
  bool cached = 3;
  int32 dimensions = 4;
}

message EmbedBatchRequest {
  repeated string texts = 1;
  int32 batch_size = 2;
  bool use_cache = 3;
}

message EmbedBatchResponse {
  repeated Embedding embeddings = 1;
  int32 processed = 2;
  float processing_time = 3;
}

message Embedding {
  repeated float values = 1;
}

message SimilarityRequest {
  repeated float vec1 = 1;
  repeated float vec2 = 2;
}

message SimilarityResponse {
  float score = 1;
}

message SearchRequest {
  string query = 1;
  repeated Embedding corpus = 2;
  int32 top_k = 3;
}

message SearchResponse {
  repeated SearchResult results = 1;
}

message SearchResult {
  int32 index = 1;
  float score = 2;
}

message StatusResponse {
  bool is_running = 1;
  string current_model = 2;
  int32 cache_size = 3;
  int32 queue_length = 4;
  float memory_usage_mb = 5;
  float cpu_usage_percent = 6;
}

message EmbeddingOptions {
  bool normalize = 1;
  PoolingStrategy pooling = 2;
  int32 max_length = 3;
}

enum PoolingStrategy {
  MEAN = 0;
  MAX = 1;
  CLS = 2;
}

enum EmbeddingModel {
  MINILM_L6_V2 = 0;
  MINILM_L12_V2 = 1;
  MPNET_BASE_V2 = 2;
  MULTILINGUAL_E5 = 3;
  CUSTOM = 4;
}
```

### Flutter Client Integration

```dart
// lib/shared/services/embeddings/embedding_client.dart
import 'package:grpc/grpc.dart';

class EmbeddingClient {
  late ClientChannel _channel;
  late EmbeddingServiceClient _stub;
  final _cache = <String, List<double>>{};
  
  Future<void> connect({String host = 'localhost', int port = 50052}) async {
    _channel = ClientChannel(
      host,
      port: port,
      options: const ChannelOptions(
        credentials: ChannelCredentials.insecure(),
      ),
    );
    
    _stub = EmbeddingServiceClient(_channel);
  }
  
  Future<List<double>> embed(String text, {bool useCache = true}) async {
    // Check local cache first
    if (useCache && _cache.containsKey(text)) {
      return _cache[text]!;
    }
    
    final request = EmbedRequest()
      ..text = text
      ..useCache = useCache;
    
    final response = await _stub.embed(request);
    final embedding = response.embedding.toList();
    
    // Cache locally
    if (useCache) {
      _cache[text] = embedding;
    }
    
    return embedding;
  }
  
  Future<List<List<double>>> embedBatch(
    List<String> texts,
    {int batchSize = 32}
  ) async {
    final request = EmbedBatchRequest()
      ..texts.addAll(texts)
      ..batchSize = batchSize;
    
    final response = await _stub.embedBatch(request);
    
    return response.embeddings
        .map((e) => e.values.toList())
        .toList();
  }
  
  double cosineSimilarity(List<double> vec1, List<double> vec2) {
    double dotProduct = 0.0;
    for (int i = 0; i < vec1.length; i++) {
      dotProduct += vec1[i] * vec2[i];
    }
    return dotProduct; // Assumes normalized vectors
  }
  
  Future<ServiceStatus> getStatus() async {
    final response = await _stub.getStatus(StatusRequest());
    return ServiceStatus.fromProto(response);
  }
  
  void dispose() {
    _channel.shutdown();
  }
}
```

### SurrealDB Schema

```sql
-- Embeddings cache table
DEFINE TABLE embedding_cache SCHEMAFULL;
DEFINE FIELD text_hash ON embedding_cache TYPE string;
DEFINE FIELD embedding ON embedding_cache TYPE array<float>;
DEFINE FIELD model ON embedding_cache TYPE string;
DEFINE FIELD created_at ON embedding_cache TYPE datetime DEFAULT time::now();
DEFINE FIELD accessed_at ON embedding_cache TYPE datetime DEFAULT time::now();
DEFINE FIELD access_count ON embedding_cache TYPE int DEFAULT 1;

-- Index for fast lookup
DEFINE INDEX text_hash_idx ON embedding_cache FIELDS text_hash UNIQUE;

-- Model registry
DEFINE TABLE embedding_models SCHEMAFULL;
DEFINE FIELD name ON embedding_models TYPE string;
DEFINE FIELD path ON embedding_models TYPE string;
DEFINE FIELD dimensions ON embedding_models TYPE int;
DEFINE FIELD size_mb ON embedding_models TYPE float;
DEFINE FIELD is_active ON embedding_models TYPE bool DEFAULT false;

-- Function to get or create embedding
DEFINE FUNCTION fn::get_or_create_embedding($text: string, $model: string) {
  LET $hash = crypto::hash::sha256($text);
  LET $cached = SELECT * FROM embedding_cache WHERE text_hash = $hash LIMIT 1;
  
  IF $cached THEN
    UPDATE embedding_cache SET 
      accessed_at = time::now(),
      access_count += 1
    WHERE text_hash = $hash;
    RETURN $cached.embedding;
  ELSE
    RETURN null; -- Embedding service will handle generation
  END;
};
```

## Testing Requirements

### Unit Tests

```dart
// test/shared/services/embeddings/embedding_service_test.dart
void main() {
  group('EmbeddingService', () {
    test('generates consistent embeddings', () async {
      final service = EmbeddingService();
      final text = 'Test sentence for embedding';
      
      final embedding1 = await service.embed(text);
      final embedding2 = await service.embed(text);
      
      expect(embedding1, equals(embedding2));
      expect(embedding1.length, equals(384)); // MiniLM dimensions
    });
    
    test('calculates similarity correctly', () {
      final service = EmbeddingService();
      final vec1 = [0.5, 0.5, 0.5];
      final vec2 = [0.5, 0.5, 0.5];
      
      final similarity = service.cosineSimilarity(vec1, vec2);
      expect(similarity, closeTo(1.0, 0.01));
    });
    
    test('handles batch processing', () async {
      final service = EmbeddingService();
      final texts = List.generate(100, (i) => 'Text $i');
      
      final embeddings = await service.embedBatch(texts);
      expect(embeddings.length, equals(100));
    });
  });
}
```

### Integration Tests

```dart
// integration_test/embedding_service_test.dart
void main() {
  testWidgets('Embedding service integration', (tester) async {
    final service = EmbeddingService();
    await service.connect();
    
    // Test embedding generation
    final embedding = await service.embed('Test text');
    expect(embedding, isNotEmpty);
    
    // Test status monitoring
    final status = await service.getStatus();
    expect(status.isRunning, isTrue);
    expect(status.currentModel, equals('all-MiniLM-L6-v2'));
  });
}
```

### Performance Tests

```dart
// test/shared/services/embeddings/performance_test.dart
void main() {
  test('meets performance targets', () async {
    final service = EmbeddingService();
    
    final stopwatch = Stopwatch()..start();
    await service.embed('Test text for performance');
    stopwatch.stop();
    
    expect(stopwatch.elapsedMilliseconds, lessThan(100));
  });
}
```

### Accessibility Tests

- [ ] Service status communicated to screen reader
- [ ] Error states clearly indicated
- [ ] Keyboard navigation for controls
- [ ] Alternative status indicators

## Definition of Done

- [ ] Embedding service runs locally
- [ ] Multiple models supported
- [ ] Cache system functional
- [ ] Batch processing works
- [ ] Performance targets met (<100ms)
- [ ] gRPC communication established
- [ ] Resource monitoring active
- [ ] All tests passing (>80% coverage)
- [ ] Cross-platform compatible
- [ ] Documentation complete
- [ ] Code review approved
