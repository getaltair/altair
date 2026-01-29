package com.getaltair.altair.shared.dto.ai

import kotlinx.serialization.Serializable

// ==================== Transcription DTOs ====================

/**
 * Request to transcribe audio to text using speech-to-text AI.
 *
 * @property audioBase64 Base64-encoded audio data
 * @property mimeType MIME type of the audio (e.g., "audio/webm", "audio/mp4")
 */
@Serializable
data class TranscribeRequest(
    val audioBase64: String,
    val mimeType: String
)

/**
 * Response from audio transcription.
 *
 * @property text Transcribed text content
 * @property confidence Confidence score from 0.0 to 1.0
 * @property durationMs Duration of the audio in milliseconds
 */
@Serializable
data class TranscribeResponse(
    val text: String,
    val confidence: Float,
    val durationMs: Long
)

// ==================== Embedding DTOs ====================

/**
 * Request to generate vector embedding for text.
 *
 * @property text Text content to embed
 */
@Serializable
data class EmbeddingRequest(
    val text: String
)

/**
 * Response with generated vector embedding.
 *
 * @property embedding Vector representation of the input text
 * @property dimensions Number of dimensions in the embedding vector
 */
@Serializable
data class EmbeddingResponse(
    val embedding: List<Float>,
    val dimensions: Int
)

// ==================== Semantic Search DTOs ====================

/**
 * Request to perform semantic search across knowledge base.
 *
 * @property query Natural language search query
 * @property entityTypes Types of entities to search: "note", "source_document"
 * @property limit Maximum number of results to return (default: 10)
 */
@Serializable
data class SemanticSearchRequest(
    val query: String,
    val entityTypes: List<String>,
    val limit: Int = 10
)

/**
 * Response with semantic search results.
 *
 * @property results List of matching entities sorted by relevance
 */
@Serializable
data class SemanticSearchResponse(
    val results: List<SearchResult>
)

/**
 * Individual search result entry.
 *
 * @property entityType Type of entity ("note" or "source_document")
 * @property entityId Unique identifier of the entity
 * @property title Entity title
 * @property preview Text preview/excerpt from the entity
 * @property score Relevance score from 0.0 to 1.0
 */
@Serializable
data class SearchResult(
    val entityType: String,
    val entityId: String,
    val title: String,
    val preview: String,
    val score: Float
)

// ==================== Completion DTOs ====================

/**
 * Request for AI text completion/generation.
 *
 * @property prompt The prompt to complete
 * @property context Additional context strings to provide to the model
 * @property maxTokens Maximum number of tokens to generate (default: 500)
 */
@Serializable
data class CompletionRequest(
    val prompt: String,
    val context: List<String> = emptyList(),
    val maxTokens: Int = 500
)

/**
 * Response with AI-generated text completion.
 *
 * @property text Generated text
 * @property tokensUsed Number of tokens consumed by this request
 */
@Serializable
data class CompletionResponse(
    val text: String,
    val tokensUsed: Int
)

// ==================== Quest Breakdown DTOs ====================

/**
 * Request for AI to suggest Quest breakdown into Checkpoints.
 *
 * @property questTitle Title of the Quest to break down
 * @property questDescription Optional description providing additional context
 */
@Serializable
data class SuggestQuestBreakdownRequest(
    val questTitle: String,
    val questDescription: String?
)

/**
 * Response with suggested Checkpoint breakdown.
 *
 * @property checkpoints List of suggested checkpoints in recommended order
 */
@Serializable
data class SuggestQuestBreakdownResponse(
    val checkpoints: List<SuggestedCheckpoint>
)

/**
 * Individual suggested checkpoint.
 *
 * @property title Suggested checkpoint title/description
 * @property estimatedEnergy Estimated energy cost for this checkpoint (nullable)
 */
@Serializable
data class SuggestedCheckpoint(
    val title: String,
    val estimatedEnergy: Int?
)

// ==================== Summarization DTOs ====================

/**
 * Request to generate AI summary of a Note.
 *
 * @property noteId ID of the Note to summarize
 * @property maxLength Maximum length of the summary in characters (default: 200)
 */
@Serializable
data class SummarizeNoteRequest(
    val noteId: String,
    val maxLength: Int = 200
)

/**
 * Response with generated summary.
 *
 * @property summary Generated summary text
 */
@Serializable
data class SummarizeNoteResponse(
    val summary: String
)
