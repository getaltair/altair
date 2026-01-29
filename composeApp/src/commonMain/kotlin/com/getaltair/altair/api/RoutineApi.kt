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
 * Client API for Routine operations.
 *
 * Provides access to recurring task template endpoints.
 * All operations require authentication via JWT token in httpClient.
 */
class RoutineApi(private val httpClient: HttpClient) {
    private val baseUrl = "/api/routines"

    /**
     * Retrieves all routines for the authenticated user.
     *
     * @return Success with list of routines, or Failure
     */
    suspend fun getRoutines(): Either<AltairError, List<RoutineResponse>> = try {
        val response = httpClient.get(baseUrl)

        if (response.status.isSuccess()) {
            response.body<List<RoutineResponse>>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Creates a new routine.
     *
     * @param request Routine creation request
     * @return Success with created routine, or Failure
     */
    suspend fun createRoutine(request: CreateRoutineRequest): Either<AltairError, RoutineResponse> = try {
        val response = httpClient.post(baseUrl) {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<RoutineResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Retrieves a specific routine by ID.
     *
     * @param id Routine identifier
     * @return Success with routine details, or Failure
     */
    suspend fun getRoutine(id: String): Either<AltairError, RoutineResponse> = try {
        val response = httpClient.get("$baseUrl/$id")

        if (response.status.isSuccess()) {
            response.body<RoutineResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Updates an existing routine.
     *
     * @param id Routine identifier
     * @param request Update request with fields to modify
     * @return Success with updated routine, or Failure
     */
    suspend fun updateRoutine(id: String, request: UpdateRoutineRequest): Either<AltairError, RoutineResponse> = try {
        val response = httpClient.put("$baseUrl/$id") {
            contentType(ContentType.Application.Json)
            setBody(request)
        }

        if (response.status.isSuccess()) {
            response.body<RoutineResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Soft-deletes a routine.
     *
     * @param id Routine identifier
     * @return Success on deletion, or Failure
     */
    suspend fun deleteRoutine(id: String): Either<AltairError, Unit> = try {
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
     * Activates a routine to resume Quest generation.
     *
     * @param id Routine identifier
     * @return Success with updated routine, or Failure
     */
    suspend fun activateRoutine(id: String): Either<AltairError, RoutineResponse> = try {
        val response = httpClient.post("$baseUrl/$id/activate")

        if (response.status.isSuccess()) {
            response.body<RoutineResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    /**
     * Deactivates a routine to pause Quest generation.
     *
     * @param id Routine identifier
     * @return Success with updated routine, or Failure
     */
    suspend fun deactivateRoutine(id: String): Either<AltairError, RoutineResponse> = try {
        val response = httpClient.post("$baseUrl/$id/deactivate")

        if (response.status.isSuccess()) {
            response.body<RoutineResponse>().right()
        } else {
            handleError(response.status, response.body()).left()
        }
    } catch (e: Exception) {
        AltairError.NetworkError.ConnectionFailed(e.message ?: "Network error").left()
    }

    // === Error Handling ===

    private suspend fun handleError(status: HttpStatusCode, errorBody: String): AltairError {
        return when (status) {
            HttpStatusCode.NotFound -> AltairError.NotFoundError.RoutineNotFound(errorBody)
            HttpStatusCode.Unauthorized -> AltairError.AuthError.Unauthorized
            HttpStatusCode.BadRequest -> AltairError.ValidationError.ConstraintViolation(errorBody)
            HttpStatusCode.Conflict -> AltairError.ConflictError.DuplicateEntity("Routine", "unknown", errorBody)
            else -> AltairError.NetworkError.ServerError(status.value, errorBody)
        }
    }
}
