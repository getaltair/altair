package com.getaltair.altair.api

/**
 * Sealed class representing API errors for client-side error handling.
 * Maps HTTP error responses to domain-specific error types.
 */
sealed class ApiError {
    abstract val message: String

    /**
     * Network connectivity or communication error.
     */
    data class NetworkError(override val message: String) : ApiError()

    /**
     * 401 Unauthorized - authentication required or token invalid.
     */
    data class Unauthorized(override val message: String) : ApiError()

    /**
     * 403 Forbidden - authenticated but insufficient permissions.
     */
    data class Forbidden(override val message: String) : ApiError()

    /**
     * 404 Not Found - requested resource does not exist.
     */
    data class NotFound(override val message: String) : ApiError()

    /**
     * 409 Conflict - resource state conflict (e.g., duplicate username).
     */
    data class Conflict(override val message: String) : ApiError()

    /**
     * 400 Bad Request - validation errors with field-specific details.
     */
    data class ValidationError(
        override val message: String,
        val details: Map<String, String>? = null
    ) : ApiError()

    /**
     * 5xx Server Error - unexpected server-side error.
     */
    data class ServerError(
        val statusCode: Int,
        override val message: String
    ) : ApiError()

    /**
     * 501 Not Implemented - feature not yet available.
     */
    data class NotImplemented(override val message: String) : ApiError()
}
