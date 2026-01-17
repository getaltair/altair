package com.getaltair.altair.rpc

import kotlinx.coroutines.flow.Flow
import kotlinx.rpc.annotations.Rpc
import kotlinx.serialization.Serializable

/**
 * RPC service for AI-powered features.
 *
 * Provides server-centralized AI capabilities including embeddings for
 * semantic search, audio transcription, and streaming text completions.
 * All AI processing happens server-side to leverage GPU resources and
 * manage API costs.
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
 */
@Serializable
data class CompletionRequest(
    val prompt: String,
    val systemPrompt: String? = null,
    val maxTokens: Int = 1024,
    val temperature: Float = 0.7f,
    val context: List<ContextMessage> = emptyList(),
)

/**
 * A message in the conversation context for completions.
 */
@Serializable
data class ContextMessage(
    val role: String,
    val content: String,
)
