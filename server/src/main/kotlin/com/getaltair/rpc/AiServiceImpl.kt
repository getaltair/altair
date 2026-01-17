package com.getaltair.rpc

import com.getaltair.altair.rpc.AiService
import com.getaltair.altair.rpc.CompletionRequest
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlin.random.Random

/**
 * Stub implementation of AiService for infrastructure validation.
 *
 * Returns placeholder data to verify RPC transport works correctly.
 * Real implementation will integrate with AI providers in Phase 5+.
 */
class AiServiceImpl : AiService {
    override suspend fun embed(texts: List<String>): List<List<Float>> {
        // Stub: Return random embeddings (typical small model embedding dimensions)
        return texts.map {
            List(EMBEDDING_DIMENSIONS) { Random.nextFloat() * 2 - 1 }
        }
    }

    override suspend fun transcribe(
        audioData: ByteArray,
        format: String,
    ): String {
        // Stub: Return placeholder transcription
        return "[Transcription placeholder - received ${audioData.size} bytes of $format audio]"
    }

    override fun complete(request: CompletionRequest): Flow<String> =
        flow {
            // Stub: Stream a simple response token by token
            val maxPromptPreview = 50
            val response = "This is a stub response to: ${request.prompt.take(maxPromptPreview)}..."
            val tokens = response.split(" ")
            for (token in tokens) {
                emit("$token ")
                delay(TOKEN_GENERATION_DELAY_MS)
            }
        }

    private companion object {
        const val EMBEDDING_DIMENSIONS = 384
        const val TOKEN_GENERATION_DELAY_MS = 50L
    }
}
