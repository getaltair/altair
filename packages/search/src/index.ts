/**
 * @altair/search - Local embedding and hybrid search utilities
 *
 * Provides text embedding generation using ONNX (all-MiniLM-L6-v2) for semantic search,
 * plus utilities for hybrid search combining semantic similarity with keyword matching.
 */

// ============================================================================
// Configuration
// ============================================================================

/**
 * Embedding model configuration
 */
export interface EmbeddingConfig {
  /** Model name (default: 'all-MiniLM-L6-v2') */
  modelName?: string;
  /** Model file path (ONNX format) */
  modelPath?: string;
  /** Tokenizer vocabulary file path */
  vocabPath?: string;
  /** Maximum sequence length (default: 256 tokens) */
  maxLength?: number;
  /** Normalize embeddings to unit vectors (default: true) */
  normalize?: boolean;
}

/**
 * Default embedding configuration
 *
 * Uses all-MiniLM-L6-v2 model (~25MB), optimized for semantic similarity.
 */
export const defaultEmbeddingConfig: Partial<EmbeddingConfig> = {
  modelName: 'all-MiniLM-L6-v2',
  maxLength: 256,
  normalize: true,
};

// ============================================================================
// Embedding Operations
// ============================================================================

/**
 * Embedding vector (384 dimensions for all-MiniLM-L6-v2)
 */
export type EmbeddingVector = number[];

/**
 * Embedding result
 */
export interface EmbeddingResult {
  /** Text that was embedded */
  text: string;
  /** Embedding vector */
  vector: EmbeddingVector;
  /** Number of tokens in input */
  tokenCount: number;
  /** Model used for embedding */
  modelName: string;
  /** Generation timestamp */
  generatedAt: Date;
}

/**
 * Batch embedding result
 */
export interface BatchEmbeddingResult {
  /** Embedding results for each text */
  embeddings: EmbeddingResult[];
  /** Total processing time in milliseconds */
  processingTimeMs: number;
}

// ============================================================================
// Search Operations
// ============================================================================

/**
 * Search mode
 */
export enum SearchMode {
  /** Semantic search only (cosine similarity) */
  Semantic = 'semantic',
  /** Keyword search only (BM25 or similar) */
  Keyword = 'keyword',
  /** Hybrid search (combines semantic + keyword) */
  Hybrid = 'hybrid',
}

/**
 * Search options
 */
export interface SearchOptions {
  /** Search mode (default: hybrid) */
  mode?: SearchMode;
  /** Maximum number of results (default: 10) */
  limit?: number;
  /** Minimum similarity threshold (0-1, default: 0.5) */
  minSimilarity?: number;
  /** Weight for semantic score in hybrid mode (0-1, default: 0.7) */
  semanticWeight?: number;
  /** Weight for keyword score in hybrid mode (0-1, default: 0.3) */
  keywordWeight?: number;
  /** Boost factor for exact matches (default: 1.5) */
  exactMatchBoost?: number;
}

/**
 * Search result
 */
export interface SearchResult<T = unknown> {
  /** Matched item */
  item: T;
  /** Overall relevance score (0-1) */
  score: number;
  /** Semantic similarity score (0-1, null if mode=keyword) */
  semanticScore: number | null;
  /** Keyword match score (0-1, null if mode=semantic) */
  keywordScore: number | null;
  /** Matched text snippets (for highlighting) */
  highlights?: string[];
}

/**
 * Searchable item with text content
 */
export interface SearchableItem {
  /** Unique identifier */
  id: string;
  /** Text content to search */
  text: string;
  /** Pre-computed embedding vector (optional, will be generated if not provided) */
  embedding?: EmbeddingVector;
  /** Custom metadata */
  metadata?: Record<string, unknown>;
}

// ============================================================================
// Embedding Client Interface
// ============================================================================

/**
 * Embedding generation client
 *
 * Handles ONNX model loading and inference for text embeddings.
 */
export interface EmbeddingClient {
  /**
   * Generate embedding for a single text
   *
   * @param text - Input text
   * @returns Embedding result with vector
   */
  embed(text: string): Promise<EmbeddingResult>;

  /**
   * Generate embeddings for multiple texts in batch
   *
   * @param texts - Array of input texts
   * @returns Batch embedding result
   */
  embedBatch(texts: string[]): Promise<BatchEmbeddingResult>;

  /**
   * Get embedding dimension (384 for all-MiniLM-L6-v2)
   *
   * @returns Embedding vector dimension
   */
  getDimension(): number;

  /**
   * Get current model configuration
   *
   * @returns Embedding configuration
   */
  getConfig(): EmbeddingConfig;
}

// ============================================================================
// Search Client Interface
// ============================================================================

/**
 * Hybrid search client
 *
 * Combines semantic embeddings with keyword matching for robust search.
 */
export interface SearchClient<T = unknown> {
  /**
   * Add items to the search index
   *
   * @param items - Items to index
   */
  addItems(items: SearchableItem[]): Promise<void>;

  /**
   * Remove items from the search index
   *
   * @param ids - Item IDs to remove
   */
  removeItems(ids: string[]): Promise<void>;

  /**
   * Clear all items from the search index
   */
  clearIndex(): Promise<void>;

  /**
   * Search for items matching the query
   *
   * @param query - Search query text
   * @param options - Search options
   * @returns Ranked search results
   */
  search(query: string, options?: SearchOptions): Promise<SearchResult<T>[]>;

  /**
   * Get the total number of indexed items
   *
   * @returns Item count
   */
  getItemCount(): Promise<number>;
}

// ============================================================================
// Utilities
// ============================================================================

/**
 * Calculate cosine similarity between two vectors
 *
 * @param a - First vector
 * @param b - Second vector
 * @returns Cosine similarity (0-1, higher = more similar)
 *
 * @example
 * ```ts
 * const similarity = cosineSimilarity(embedding1, embedding2);
 * // similarity = 0.87 (very similar)
 * ```
 */
export function cosineSimilarity(a: EmbeddingVector, b: EmbeddingVector): number {
  if (a.length !== b.length) {
    throw new Error(`Vector dimension mismatch: ${a.length} !== ${b.length}`);
  }

  let dotProduct = 0;
  let normA = 0;
  let normB = 0;

  for (let i = 0; i < a.length; i++) {
    const aVal = a[i] ?? 0;
    const bVal = b[i] ?? 0;
    dotProduct += aVal * bVal;
    normA += aVal * aVal;
    normB += bVal * bVal;
  }

  const magnitude = Math.sqrt(normA) * Math.sqrt(normB);
  return magnitude === 0 ? 0 : dotProduct / magnitude;
}

/**
 * Normalize a vector to unit length
 *
 * @param vector - Input vector
 * @returns Normalized vector
 *
 * @example
 * ```ts
 * const normalized = normalizeVector(embedding);
 * // Magnitude of normalized is 1.0
 * ```
 */
export function normalizeVector(vector: EmbeddingVector): EmbeddingVector {
  const magnitude = Math.sqrt(vector.reduce((sum, val) => sum + val * val, 0));
  if (magnitude === 0) return vector;
  return vector.map((val) => val / magnitude);
}

/**
 * Calculate BM25 score for keyword matching
 *
 * Simplified BM25 implementation for keyword relevance scoring.
 *
 * @param queryTerms - Query terms (lowercased, tokenized)
 * @param docTerms - Document terms (lowercased, tokenized)
 * @param avgDocLength - Average document length (in terms)
 * @param k1 - BM25 parameter (default: 1.5)
 * @param b - BM25 parameter (default: 0.75)
 * @returns BM25 score
 */
export function bm25Score(
  queryTerms: string[],
  docTerms: string[],
  avgDocLength: number,
  k1 = 1.5,
  b = 0.75
): number {
  const termFreq = new Map<string, number>();
  for (const term of docTerms) {
    termFreq.set(term, (termFreq.get(term) || 0) + 1);
  }

  let score = 0;
  const docLength = docTerms.length;

  for (const term of queryTerms) {
    const tf = termFreq.get(term) || 0;
    if (tf === 0) continue;

    const numerator = tf * (k1 + 1);
    const denominator = tf + k1 * (1 - b + (b * docLength) / avgDocLength);
    score += numerator / denominator;
  }

  return score;
}

/**
 * Tokenize text for keyword search
 *
 * Simple tokenization: lowercase, split on non-alphanumeric, remove stopwords.
 *
 * @param text - Input text
 * @returns Array of tokens
 *
 * @example
 * ```ts
 * const tokens = tokenize("Hello, world! This is a test.");
 * // tokens = ['hello', 'world', 'test']
 * ```
 */
export function tokenize(text: string): string[] {
  const stopwords = new Set([
    'a',
    'an',
    'and',
    'are',
    'as',
    'at',
    'be',
    'by',
    'for',
    'from',
    'has',
    'he',
    'in',
    'is',
    'it',
    'its',
    'of',
    'on',
    'that',
    'the',
    'to',
    'was',
    'will',
    'with',
  ]);

  return text
    .toLowerCase()
    .split(/\W+/)
    .filter((token) => token.length > 0 && !stopwords.has(token));
}

/**
 * Extract text snippets around keyword matches for highlighting
 *
 * @param text - Full text
 * @param queryTerms - Query terms to highlight
 * @param snippetLength - Number of characters around match (default: 100)
 * @returns Array of text snippets
 *
 * @example
 * ```ts
 * const highlights = extractHighlights(doc, ['search', 'algorithm'], 50);
 * // highlights = ['...semantic search algorithm...', '...hybrid search results...']
 * ```
 */
export function extractHighlights(
  text: string,
  queryTerms: string[],
  snippetLength = 100
): string[] {
  const highlights: string[] = [];
  const lowerText = text.toLowerCase();

  for (const term of queryTerms) {
    const index = lowerText.indexOf(term.toLowerCase());
    if (index === -1) continue;

    const start = Math.max(0, index - snippetLength / 2);
    const end = Math.min(text.length, index + term.length + snippetLength / 2);

    let snippet = text.slice(start, end);
    if (start > 0) snippet = '...' + snippet;
    if (end < text.length) snippet = snippet + '...';

    highlights.push(snippet);
  }

  return highlights;
}

/**
 * Combine semantic and keyword scores for hybrid search
 *
 * @param semanticScore - Semantic similarity score (0-1)
 * @param keywordScore - Keyword match score (0-1)
 * @param semanticWeight - Weight for semantic score (default: 0.7)
 * @param keywordWeight - Weight for keyword score (default: 0.3)
 * @returns Combined score (0-1)
 *
 * @example
 * ```ts
 * const score = combineScores(0.85, 0.6, 0.7, 0.3);
 * // score = 0.85 * 0.7 + 0.6 * 0.3 = 0.775
 * ```
 */
export function combineScores(
  semanticScore: number,
  keywordScore: number,
  semanticWeight = 0.7,
  keywordWeight = 0.3
): number {
  return semanticScore * semanticWeight + keywordScore * keywordWeight;
}
