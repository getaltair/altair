package com.getaltair.server.routes

import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.dto.auth.ErrorResponse
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.auth.jwt.*
import io.ktor.server.response.*

/**
 * Extension to extract user ID from JWT principal.
 * Throws if no valid JWT or userId claim present.
 */
val ApplicationCall.userId: Ulid
    get() {
        val principal = principal<JWTPrincipal>()
            ?: throw IllegalStateException("No JWT principal found")
        val userIdClaim = principal.payload.getClaim("userId").asString()
            ?: throw IllegalStateException("No userId claim in JWT")
        return Ulid.parse(userIdClaim)
            ?: throw IllegalStateException("Invalid userId format in JWT: $userIdClaim")
    }

/**
 * Extension to respond with appropriate HTTP status for AltairError.
 */
suspend fun ApplicationCall.respondError(error: AltairError) {
    val (status, response) = error.toHttpResponse()
    respond(status, response)
}

/**
 * Maps AltairError to HTTP status code and ErrorResponse.
 */
fun AltairError.toHttpResponse(): Pair<HttpStatusCode, ErrorResponse> = when (this) {
    is AltairError.NotFoundError -> HttpStatusCode.NotFound to ErrorResponse(
        code = "NOT_FOUND",
        message = message
    )
    is AltairError.ConflictError -> HttpStatusCode.Conflict to ErrorResponse(
        code = "CONFLICT",
        message = message
    )
    is AltairError.ValidationError -> HttpStatusCode.BadRequest to ErrorResponse(
        code = "VALIDATION_ERROR",
        message = message
    )
    is AltairError.AuthError -> HttpStatusCode.Unauthorized to ErrorResponse(
        code = "AUTH_ERROR",
        message = message
    )
    is AltairError.StorageError -> HttpStatusCode.InternalServerError to ErrorResponse(
        code = "STORAGE_ERROR",
        message = message
    )
    is AltairError.NetworkError -> HttpStatusCode.BadGateway to ErrorResponse(
        code = "NETWORK_ERROR",
        message = message
    )
}

/**
 * Safely parses a path parameter as Ulid, responding with 400 if invalid.
 */
suspend inline fun ApplicationCall.pathParamAsUlid(name: String, block: (Ulid) -> Unit) {
    val raw = parameters[name]
    if (raw == null) {
        respond(HttpStatusCode.BadRequest, ErrorResponse(
            code = "MISSING_PARAMETER",
            message = "Missing required path parameter: $name"
        ))
        return
    }
    val ulid = Ulid.parse(raw)
    if (ulid == null) {
        respond(HttpStatusCode.BadRequest, ErrorResponse(
            code = "INVALID_PARAMETER",
            message = "Invalid $name format: $raw"
        ))
    } else {
        block(ulid)
    }
}
