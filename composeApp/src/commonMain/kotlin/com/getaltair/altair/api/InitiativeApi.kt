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
 * Client API for Initiative operations.
 *
 * Provides access to cross-module organizational structures (Projects and Areas).
 * All operations require authentication via JWT token in httpClient.
 */
class InitiativeApi(private val httpClient: HttpClient) {
    private val baseUrl = "/api/initiatives"

    /**
     * Retrieves all initiatives for the authenticated user.
     *
     * @return Success with list of initiatives, or Failure
     */
    suspend fun getInitiatives(): Either<AltairError, List<InitiativeResponse>> = try {
        val response = httpClient.get(baseUrl)

        if (response.status.isSuccess()) {
            response.body<List<InitiativeResponse>>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Creates a new initiative.
     *
     * @param request Initiative creation request
     * @return Success with created initiative, or Failure
     */
    suspend fun createInitiative(request: CreateInitiativeRequest): Either<AltairError, InitiativeResponse> = try {
        val response = httpClient.post(baseUrl) {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<InitiativeResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Retrieves a specific initiative by ID.
     *
     * @param id Initiative identifier
     * @return Success with initiative details, or Failure
     */
    suspend fun getInitiative(id: String): Either<AltairError, InitiativeResponse> = try {
        val response = httpClient.get("$baseUrl/$id")

        if (response.status.isSuccess()) {
            response.body<InitiativeResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Updates an existing initiative.
     *
     * @param id Initiative identifier
     * @param request Update request with fields to modify
     * @return Success with updated initiative, or Failure
     */
    suspend fun updateInitiative(id: String, request: UpdateInitiativeRequest): Either<AltairError, InitiativeResponse> = try {
        val response = httpClient.put("$baseUrl/$id") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<InitiativeResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Soft-deletes an initiative.
     *
     * @param id Initiative identifier
     * @return Success on deletion, or Failure
     */
    suspend fun deleteInitiative(id: String): Either<AltairError, Unit> = try {
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
     * Sets an initiative as the focused initiative for the user.
     *
     * Automatically unfocuses any previously focused initiative.
     *
     * @param id Initiative identifier
     * @return Success with updated initiative, or Failure
     */
    suspend fun focusInitiative(id: String): Either<AltairError, InitiativeResponse> = try {
        val response = httpClient.post("$baseUrl/$id/focus")

        if (response.status.isSuccess()) {
            response.body<InitiativeResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Removes focus from an initiative.
     *
     * @param id Initiative identifier
     * @return Success on unfocus, or Failure
     */
    suspend fun unfocusInitiative(id: String): Either<AltairError, Unit> = try {
        val response = httpClient.post("$baseUrl/$id/unfocus")

        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    // === Error Handling ===

    private suspend fun handleError(status: HttpStatusCode, errorBody: String): AltairError {
        return when (status) {
            HttpStatusCode.NotFound -> AltairError.NotFoundError.InitiativeNotFound(errorBody)
            HttpStatusCode.Unauthorized -> AltairError.AuthError.Unauthorized
            HttpStatusCode.BadRequest -> AltairError.ValidationError.ConstraintViolation(errorBody)
            HttpStatusCode.Conflict -> AltairError.ConflictError.DuplicateEntity("Initiative", "unknown", errorBody)
            else -> AltairError.NetworkError.ServerError(status.value, errorBody)
        }
    }
}
