package com.getaltair.rpc

import com.getaltair.altair.rpc.AiService
import com.getaltair.altair.rpc.CompletionRequest
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import org.slf4j.LoggerFactory
import kotlin.random.Random

/**
 * Stub implementation of AiService for infrastructure validation.
 *
 * Returns placeholder data to verify RPC transport works correctly.
 * Real implementation will integrate with AI providers in Phase 5+.
 */
class AiServiceImpl : AiService {
    private val logger = LoggerFactory.getLogger(AiServiceImpl::class.java)

    override suspend fun embed(texts: List<String>): List<List<Float>> {
        logger.debug("STUB: AiService.embed() generating fake embeddings for {} texts", texts.size)
        return texts.map {
            List(EMBEDDING_DIMENSIONS) { Random.nextFloat() * 2 - 1 }
        }
    }

    override suspend fun transcribe(
        audioData: ByteArray,
        format: String,
    ): String {
        logger.debug("STUB: AiService.transcribe() received {} bytes of {} audio", audioData.size, format)
        return "[Transcription placeholder - received ${audioData.size} bytes of $format audio]"
    }

    override fun complete(request: CompletionRequest): Flow<String> =
        flow {
            val promptPreview = request.prompt.take(LOG_PROMPT_PREVIEW_LENGTH)
            logger.debug("STUB: AiService.complete() streaming response for prompt: {}...", promptPreview)
            val response = "This is a stub response to: ${request.prompt.take(RESPONSE_PROMPT_PREVIEW_LENGTH)}..."
            val tokens = response.split(" ")
            for (token in tokens) {
                emit("$token ")
                delay(TOKEN_GENERATION_DELAY_MS)
            }
        }

    private companion object {
        const val EMBEDDING_DIMENSIONS = 384
        const val TOKEN_GENERATION_DELAY_MS = 50L
        const val LOG_PROMPT_PREVIEW_LENGTH = 30
        const val RESPONSE_PROMPT_PREVIEW_LENGTH = 50
    }
}
