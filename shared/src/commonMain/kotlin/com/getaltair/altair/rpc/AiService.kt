package com.getaltair.altair.rpc

import kotlinx.coroutines.flow.Flow
import kotlinx.rpc.annotations.Rpc
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * RPC service for AI-powered features.
 *
 * Provides server-centralized AI capabilities including embeddings for
 * semantic search, audio transcription, and streaming text completions.
 * All AI processing happens server-side to leverage GPU resources and
 * manage API costs.
 *
 * **Requires authentication.** All operations are scoped to the authenticated user
 * for usage tracking and rate limiting.
 *
 * ## Error Handling
 *
 * RPC services use exception-based error handling at the transport layer.
 * Callers should wrap RPC calls with Arrow's `Either.catch {}` to convert
 * exceptions to typed errors at the repository layer.
 */
@Rpc
interface AiService {
    /**
     * Generate embeddings for a list of text strings.
     *
     * Used for semantic search and similarity matching in the Knowledge module.
     *
     * @param texts List of text strings to embed
     * @return List of embedding vectors (one per input text)
     */
    suspend fun embed(texts: List<String>): List<List<Float>>

    /**
     * Transcribe audio data to text.
     *
     * Supports voice capture for quick notes and inbox items.
     *
     * @param audioData Raw audio bytes
     * @param format Audio format (e.g., "wav", "mp3", "m4a", "webm")
     * @return Transcribed text
     */
    suspend fun transcribe(
        audioData: ByteArray,
        format: String,
    ): String

    /**
     * Generate a text completion with streaming response.
     *
     * Returns tokens as they are generated for responsive UI updates.
     *
     * @param request Completion request with prompt and parameters
     * @return Flow of text tokens as they are generated
     */
    fun complete(request: CompletionRequest): Flow<String>
}

/**
 * Request for AI text completion.
 *
 * @property prompt The main prompt text to complete (must not be blank)
 * @property systemPrompt Optional system message to set AI behavior
 * @property maxTokens Maximum tokens to generate (must be positive, default: 1024)
 * @property temperature Creativity parameter from 0.0 to 2.0 (default: 0.7)
 * @property context Previous conversation messages for context
 */
@Serializable
data class CompletionRequest(
    val prompt: String,
    val systemPrompt: String? = null,
    val maxTokens: Int = 1024,
    val temperature: Float = 0.7f,
    val context: List<ContextMessage> = emptyList(),
) {
    init {
        require(prompt.isNotBlank()) { "Prompt must not be blank" }
        require(maxTokens > 0) { "maxTokens must be positive" }
        require(temperature in 0f..2f) { "temperature must be between 0 and 2" }
    }
}

/**
 * Role of a message author in a conversation context.
 */
@Serializable
enum class MessageRole {
    @SerialName("user")
    USER,

    @SerialName("assistant")
    ASSISTANT,

    @SerialName("system")
    SYSTEM,
}

/**
 * A message in the conversation context for completions.
 *
 * @property role Message author role (user, assistant, or system)
 * @property content The message content
 */
@Serializable
data class ContextMessage(
    val role: MessageRole,
    val content: String,
)
