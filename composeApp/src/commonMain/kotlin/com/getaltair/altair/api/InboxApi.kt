package com.getaltair.altair.api

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.dto.system.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*

/**
 * Client API for Universal Inbox operations.
 *
 * Provides access to quick capture and triage endpoints.
 * All operations require authentication via JWT token in httpClient.
 */
class InboxApi(private val httpClient: HttpClient) {
    private val baseUrl = "/api/inbox"

    /**
     * Retrieves all inbox items for the authenticated user.
     *
     * @return Success with list of inbox items, or Failure
     */
    suspend fun getInboxItems(): Either<AltairError, List<InboxItemResponse>> = try {
        val response = httpClient.get(baseUrl)

        if (response.status.isSuccess()) {
            response.body<List<InboxItemResponse>>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Creates a new inbox item for quick capture.
     *
     * @param request Inbox item creation request
     * @return Success with created inbox item, or Failure
     */
    suspend fun createInboxItem(request: CreateInboxItemRequest): Either<AltairError, InboxItemResponse> = try {
        val response = httpClient.post(baseUrl) {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<InboxItemResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Retrieves a specific inbox item by ID.
     *
     * @param id Inbox item identifier
     * @return Success with inbox item details, or Failure
     */
    suspend fun getInboxItem(id: String): Either<AltairError, InboxItemResponse> = try {
        val response = httpClient.get("$baseUrl/$id")

        if (response.status.isSuccess()) {
            response.body<InboxItemResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Soft-deletes an inbox item.
     *
     * @param id Inbox item identifier
     * @return Success on deletion, or Failure
     */
    suspend fun deleteInboxItem(id: String): Either<AltairError, Unit> = try {
        val response = httpClient.delete("$baseUrl/$id")

        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Triages an inbox item into a Quest, Note, Item, or SourceDocument.
     *
     * Creates the target entity and soft-deletes the inbox item atomically.
     *
     * @param id Inbox item identifier
     * @param request Triage request with target type and parameters
     * @return Success with triage response containing new entity ID, or Failure
     */
    suspend fun triageItem(id: String, request: TriageRequest): Either<AltairError, TriageResponse> = try {
        val response = httpClient.post("$baseUrl/triage/$id") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<TriageResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    // === Error Handling ===

    private suspend fun handleError(status: HttpStatusCode, errorBody: String): AltairError {
        return when (status) {
            HttpStatusCode.NotFound -> AltairError.ValidationError.ConstraintViolation("Inbox item not found")
            HttpStatusCode.Unauthorized -> AltairError.AuthError.Unauthorized
            HttpStatusCode.BadRequest -> AltairError.ValidationError.ConstraintViolation(errorBody)
            HttpStatusCode.Conflict -> AltairError.ConflictError.DuplicateEntity("InboxItem", "unknown", errorBody)
            else -> AltairError.NetworkError.ServerError(status.value, errorBody)
        }
    }
}
