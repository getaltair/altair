package com.getaltair.server.routes

import com.getaltair.altair.shared.dto.ai.*
import com.getaltair.altair.shared.dto.auth.ErrorResponse
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*

/**
 * Registers AI service routes under /api/ai.
 * All routes require JWT authentication.
 *
 * Endpoints:
 * - POST /transcribe - Audio to text transcription
 * - POST /embed - Generate text embeddings
 * - POST /search - Semantic search
 * - POST /complete - LLM completion
 * - POST /suggest-breakdown - Quest breakdown suggestions
 * - POST /summarize - Note summarization
 *
 * Note: Actual AI implementation is deferred to Phase 13.
 * These routes currently return NotImplemented responses.
 */
fun Route.aiRoutes() {
    route("/api/ai") {
        authenticate("jwt") {
            /**
             * POST /api/ai/transcribe
             * Transcribe audio to text using Whisper.
             */
            post("/transcribe") {
                val request = call.receive<TranscribeRequest>()
                // TODO: Phase 13 will implement AI services
                call.respond(HttpStatusCode.NotImplemented, ErrorResponse(
                    code = "AI_NOT_IMPLEMENTED",
                    message = "AI services will be implemented in Phase 13"
                ))
            }

            /**
             * POST /api/ai/embed
             * Generate text embeddings for semantic search.
             */
            post("/embed") {
                val request = call.receive<EmbeddingRequest>()
                call.respond(HttpStatusCode.NotImplemented, ErrorResponse(
                    code = "AI_NOT_IMPLEMENTED",
                    message = "AI services will be implemented in Phase 13"
                ))
            }

            /**
             * POST /api/ai/search
             * Perform semantic search across notes.
             */
            post("/search") {
                val request = call.receive<SemanticSearchRequest>()
                call.respond(HttpStatusCode.NotImplemented, ErrorResponse(
                    code = "AI_NOT_IMPLEMENTED",
                    message = "AI services will be implemented in Phase 13"
                ))
            }

            /**
             * POST /api/ai/complete
             * LLM completion for various use cases.
             */
            post("/complete") {
                val request = call.receive<CompletionRequest>()
                call.respond(HttpStatusCode.NotImplemented, ErrorResponse(
                    code = "AI_NOT_IMPLEMENTED",
                    message = "AI services will be implemented in Phase 13"
                ))
            }

            /**
             * POST /api/ai/suggest-breakdown
             * AI-suggested quest breakdown into subtasks.
             */
            post("/suggest-breakdown") {
                val request = call.receive<SuggestQuestBreakdownRequest>()
                call.respond(HttpStatusCode.NotImplemented, ErrorResponse(
                    code = "AI_NOT_IMPLEMENTED",
                    message = "AI services will be implemented in Phase 13"
                ))
            }

            /**
             * POST /api/ai/summarize
             * Summarize note content.
             */
            post("/summarize") {
                val request = call.receive<SummarizeNoteRequest>()
                call.respond(HttpStatusCode.NotImplemented, ErrorResponse(
                    code = "AI_NOT_IMPLEMENTED",
                    message = "AI services will be implemented in Phase 13"
                ))
            }
        }
    }
}
