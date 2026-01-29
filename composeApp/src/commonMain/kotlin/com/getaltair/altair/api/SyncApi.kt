package com.getaltair.altair.api

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.dto.auth.ErrorResponse
import com.getaltair.altair.shared.dto.sync.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*

/**
 * Client API for sync operations.
 * Communicates with server /api/sync endpoints.
 *
 * Note: Sync implementation is deferred to Phase 12.
 * These methods will receive NotImplemented errors from server.
 */
class SyncApi(private val httpClient: HttpClient) {

    /**
     * Pull changes from server since the given timestamp.
     *
     * @param request Pull request with last sync timestamp
     * @return Either error or pull response with changes
     */
    suspend fun pull(request: SyncPullRequest): Either<ApiError, SyncPullResponse<Any>> =
        post("/api/sync/pull", request)

    /**
     * Push local changes to server.
     *
     * @param request Push request with local changes
     * @return Either error or push response
     */
    suspend fun push(request: SyncPushRequest<Any>): Either<ApiError, SyncPushResponse> =
        post("/api/sync/push", request)

    /**
     * Get current sync status for the authenticated user.
     *
     * @return Either error or sync status
     */
    suspend fun getStatus(): Either<ApiError, SyncStatusResponse> =
        get("/api/sync/status")

    // Helper methods for HTTP operations

    /**
     * Perform GET request and parse response.
     */
    private suspend inline fun <reified T> get(path: String): Either<ApiError, T> = try {
        val response = httpClient.get(path)
        handleResponse(response)
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

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
