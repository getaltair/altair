package com.getaltair.altair.api

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.dto.ai.*
import com.getaltair.altair.shared.dto.auth.ErrorResponse
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*

/**
 * Client API for AI services.
 * Communicates with server /api/ai endpoints.
 *
 * Note: AI implementation is deferred to Phase 13.
 * These methods will receive NotImplemented errors from server.
 */
class AiApi(private val httpClient: HttpClient) {

    /**
     * Transcribe audio to text using Whisper.
     *
     * @param request Transcription request with audio data
     * @return Either error or transcription result
     */
    suspend fun transcribe(request: TranscribeRequest): Either<ApiError, TranscribeResponse> =
        post("/api/ai/transcribe", request)

    /**
     * Generate text embeddings for semantic search.
     *
     * @param request Embedding request with text
     * @return Either error or embedding vector
     */
    suspend fun embed(request: EmbeddingRequest): Either<ApiError, EmbeddingResponse> =
        post("/api/ai/embed", request)

    /**
     * Perform semantic search across notes.
     *
     * @param request Search request with query text
     * @return Either error or search results
     */
    suspend fun semanticSearch(request: SemanticSearchRequest): Either<ApiError, SemanticSearchResponse> =
        post("/api/ai/search", request)

    /**
     * LLM completion for various use cases.
     *
     * @param request Completion request with prompt
     * @return Either error or completion result
     */
    suspend fun complete(request: CompletionRequest): Either<ApiError, CompletionResponse> =
        post("/api/ai/complete", request)

    /**
     * Get AI-suggested quest breakdown into subtasks.
     *
     * @param request Breakdown request with quest description
     * @return Either error or suggested breakdown
     */
    suspend fun suggestBreakdown(request: SuggestQuestBreakdownRequest): Either<ApiError, SuggestQuestBreakdownResponse> =
        post("/api/ai/suggest-breakdown", request)

    /**
     * Summarize note content using AI.
     *
     * @param request Summarization request with note text
     * @return Either error or summary
     */
    suspend fun summarize(request: SummarizeNoteRequest): Either<ApiError, SummarizeNoteResponse> =
        post("/api/ai/summarize", request)

    // Helper methods for HTTP operations

    /**
     * Perform POST request with body and parse response.
     */
    private suspend inline fun <reified R, reified T> post(
        path: String,
        body: R
    ): Either<ApiError, T> = try {
        val response = httpClient.post(path) {
            contentType(ContentType.Application.Json)
            setBody(body)
        }
        handleResponse(response)
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Handle HTTP response, parsing success or error.
     */
    private suspend inline fun <reified T> handleResponse(
        response: HttpResponse
    ): Either<ApiError, T> =
        if (response.status.isSuccess()) {
            response.body<T>().right()
        } else {
            parseError(response).left()
        }

    /**
     * Parse HTTP error response into typed ApiError.
     */
    private suspend fun parseError(response: HttpResponse): ApiError = try {
        val error = response.body<ErrorResponse>()
        when (response.status) {
            HttpStatusCode.Unauthorized -> ApiError.Unauthorized(error.message)
            HttpStatusCode.Forbidden -> ApiError.Forbidden(error.message)
            HttpStatusCode.NotFound -> ApiError.NotFound(error.message)
            HttpStatusCode.Conflict -> ApiError.Conflict(error.message)
            HttpStatusCode.BadRequest -> ApiError.ValidationError(error.message, error.details)
            HttpStatusCode.NotImplemented -> ApiError.NotImplemented(error.message)
            else -> ApiError.ServerError(response.status.value, error.message)
        }
    } catch (e: Exception) {
        ApiError.ServerError(response.status.value, response.status.description)
    }
}
