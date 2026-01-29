package com.getaltair.altair.api

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.dto.guidance.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*

/**
 * Client API for Guidance module operations.
 *
 * Provides complete CRUD operations for:
 * - Quests (create, read, update, delete, status transitions)
 * - Checkpoints (create, read, update, delete, reorder)
 * - Epics (create, read, update, delete)
 * - Energy budgets (read, set)
 * - Today view aggregation
 *
 * All methods return [Either<ApiError, T>] for functional error handling.
 * Requires authenticated HttpClient with JWT bearer token configured.
 *
 * ## Example Usage
 *
 * ```kotlin
 * val guidanceApi = GuidanceApi(authenticatedHttpClient)
 *
 * // Create a quest
 * guidanceApi.createQuest(
 *     CreateQuestRequest(
 *         title = "Implement login screen",
 *         description = "Build UI and wire to auth API",
 *         energyCost = 3
 *     )
 * ).fold(
 *     ifLeft = { error -> println("Error: ${error.message}") },
 *     ifRight = { quest -> println("Created: ${quest.title}") }
 * )
 *
 * // Start a quest (WIP=1 enforcement)
 * guidanceApi.startQuest(questId).fold(
 *     ifLeft = { error ->
 *         when (error) {
 *             is ApiError.Conflict -> println("Another quest is already active")
 *             else -> println("Error: ${error.message}")
 *         }
 *     },
 *     ifRight = { quest -> println("Started: ${quest.title}") }
 * )
 * ```
 *
 * @property httpClient Authenticated HTTP client with JWT bearer token
 */
class GuidanceApi(private val httpClient: HttpClient) {

    // ==================== Quest Operations ====================

    /**
     * Retrieve all quests for the authenticated user.
     *
     * @param status Optional filter by quest status (BACKLOG, ACTIVE, COMPLETED, ABANDONED)
     * @return [Either] containing list of quest summaries or an [ApiError]
     */
    suspend fun getQuests(status: String? = null): Either<ApiError, List<QuestSummaryResponse>> {
        return try {
            val response = httpClient.get("/api/guidance/quests") {
                if (status != null) {
                    parameter("status", status)
                }
            }
            handleResponse<List<QuestSummaryResponse>>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Create a new quest.
     *
     * @param request Quest creation parameters
     * @return [Either] containing created quest or an [ApiError]
     */
    suspend fun createQuest(request: CreateQuestRequest): Either<ApiError, QuestResponse> {
        return try {
            val response = httpClient.post("/api/guidance/quests") {
                contentType(ContentType.Application.Json)
                setBody(request)
            }
            handleResponse<QuestResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Retrieve detailed quest information.
     *
     * @param id Quest identifier
     * @return [Either] containing quest details with checkpoints or an [ApiError]
     */
    suspend fun getQuest(id: String): Either<ApiError, QuestResponse> {
        return try {
            val response = httpClient.get("/api/guidance/quests/$id")
            handleResponse<QuestResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Update quest fields.
     *
     * Note: For status transitions, use dedicated methods like [startQuest], [completeQuest], etc.
     *
     * @param id Quest identifier
     * @param request Updated quest fields (all optional)
     * @return [Either] containing updated quest or an [ApiError]
     */
    suspend fun updateQuest(id: String, request: UpdateQuestRequest): Either<ApiError, QuestResponse> {
        return try {
            val response = httpClient.put("/api/guidance/quests/$id") {
                contentType(ContentType.Application.Json)
                setBody(request)
            }
            handleResponse<QuestResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Soft-delete a quest.
     *
     * The quest will be marked as deleted but data is preserved for audit.
     *
     * @param id Quest identifier
     * @return [Either] containing Unit on success or an [ApiError]
     */
    suspend fun deleteQuest(id: String): Either<ApiError, Unit> {
        return try {
            val response = httpClient.delete("/api/guidance/quests/$id")
            handleResponse<Unit>(response.status, Unit)
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Start a quest (transition to ACTIVE status).
     *
     * WIP=1 enforcement: Returns [ApiError.Conflict] if another quest is already active.
     *
     * @param id Quest identifier
     * @return [Either] containing started quest or an [ApiError]
     */
    suspend fun startQuest(id: String): Either<ApiError, QuestResponse> {
        return try {
            val response = httpClient.post("/api/guidance/quests/$id/start")
            handleResponse<QuestResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Complete a quest (transition to COMPLETED status).
     *
     * Energy cost is added to today's energy budget upon completion.
     *
     * @param id Quest identifier
     * @return [Either] containing completed quest or an [ApiError]
     */
    suspend fun completeQuest(id: String): Either<ApiError, QuestResponse> {
        return try {
            val response = httpClient.post("/api/guidance/quests/$id/complete")
            handleResponse<QuestResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Abandon a quest (transition to ABANDONED status).
     *
     * Use when a quest is no longer relevant or should not be pursued.
     *
     * @param id Quest identifier
     * @return [Either] containing abandoned quest or an [ApiError]
     */
    suspend fun abandonQuest(id: String): Either<ApiError, QuestResponse> {
        return try {
            val response = httpClient.post("/api/guidance/quests/$id/abandon")
            handleResponse<QuestResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Return a quest to backlog (transition to BACKLOG status).
     *
     * Frees up the WIP slot for another quest.
     *
     * @param id Quest identifier
     * @return [Either] containing backlogged quest or an [ApiError]
     */
    suspend fun backlogQuest(id: String): Either<ApiError, QuestResponse> {
        return try {
            val response = httpClient.post("/api/guidance/quests/$id/backlog")
            handleResponse<QuestResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    // ==================== Checkpoint Operations ====================

    /**
     * Retrieve all checkpoints for a quest.
     *
     * Checkpoints are returned in display order (sorted by order field).
     *
     * @param questId Quest identifier
     * @return [Either] containing ordered list of checkpoints or an [ApiError]
     */
    suspend fun getCheckpoints(questId: String): Either<ApiError, List<CheckpointResponse>> {
        return try {
            val response = httpClient.get("/api/guidance/quests/$questId/checkpoints")
            handleResponse<List<CheckpointResponse>>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Add a checkpoint to a quest.
     *
     * @param questId Quest identifier
     * @param request Checkpoint creation parameters
     * @return [Either] containing created checkpoint or an [ApiError]
     */
    suspend fun addCheckpoint(questId: String, request: CreateCheckpointRequest): Either<ApiError, CheckpointResponse> {
        return try {
            val response = httpClient.post("/api/guidance/quests/$questId/checkpoints") {
                contentType(ContentType.Application.Json)
                setBody(request)
            }
            handleResponse<CheckpointResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Update a checkpoint.
     *
     * Can be used to mark as complete, change title, or reorder.
     *
     * @param id Checkpoint identifier
     * @param request Updated checkpoint fields (all optional)
     * @return [Either] containing updated checkpoint or an [ApiError]
     */
    suspend fun updateCheckpoint(id: String, request: UpdateCheckpointRequest): Either<ApiError, CheckpointResponse> {
        return try {
            val response = httpClient.put("/api/guidance/checkpoints/$id") {
                contentType(ContentType.Application.Json)
                setBody(request)
            }
            handleResponse<CheckpointResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Delete a checkpoint.
     *
     * This is a hard delete - the checkpoint is permanently removed.
     *
     * @param id Checkpoint identifier
     * @return [Either] containing Unit on success or an [ApiError]
     */
    suspend fun deleteCheckpoint(id: String): Either<ApiError, Unit> {
        return try {
            val response = httpClient.delete("/api/guidance/checkpoints/$id")
            handleResponse<Unit>(response.status, Unit)
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Reorder checkpoints within a quest.
     *
     * The order list should contain all checkpoint IDs in the desired sequence.
     *
     * @param questId Quest identifier
     * @param request Ordered list of checkpoint IDs
     * @return [Either] containing Unit on success or an [ApiError]
     */
    suspend fun reorderCheckpoints(questId: String, request: ReorderCheckpointsRequest): Either<ApiError, Unit> {
        return try {
            val response = httpClient.post("/api/guidance/quests/$questId/checkpoints/reorder") {
                contentType(ContentType.Application.Json)
                setBody(request)
            }
            handleResponse<Unit>(response.status, Unit)
        } catch (e: Exception) {
            handleException(e)
        }
    }

    // ==================== Epic Operations ====================

    /**
     * Retrieve all epics for the authenticated user.
     *
     * @return [Either] containing list of epics with quest counts or an [ApiError]
     */
    suspend fun getEpics(): Either<ApiError, List<EpicResponse>> {
        return try {
            val response = httpClient.get("/api/guidance/epics")
            handleResponse<List<EpicResponse>>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Create a new epic.
     *
     * @param request Epic creation parameters
     * @return [Either] containing created epic or an [ApiError]
     */
    suspend fun createEpic(request: CreateEpicRequest): Either<ApiError, EpicResponse> {
        return try {
            val response = httpClient.post("/api/guidance/epics") {
                contentType(ContentType.Application.Json)
                setBody(request)
            }
            handleResponse<EpicResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Retrieve detailed epic information.
     *
     * @param id Epic identifier
     * @return [Either] containing epic details with quest counts or an [ApiError]
     */
    suspend fun getEpic(id: String): Either<ApiError, EpicResponse> {
        return try {
            val response = httpClient.get("/api/guidance/epics/$id")
            handleResponse<EpicResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Update epic fields.
     *
     * @param id Epic identifier
     * @param request Updated epic fields (all optional)
     * @return [Either] containing updated epic or an [ApiError]
     */
    suspend fun updateEpic(id: String, request: UpdateEpicRequest): Either<ApiError, EpicResponse> {
        return try {
            val response = httpClient.put("/api/guidance/epics/$id") {
                contentType(ContentType.Application.Json)
                setBody(request)
            }
            handleResponse<EpicResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Soft-delete an epic.
     *
     * The epic will be marked as deleted but data is preserved for audit.
     * Quests associated with this epic are NOT deleted.
     *
     * @param id Epic identifier
     * @return [Either] containing Unit on success or an [ApiError]
     */
    suspend fun deleteEpic(id: String): Either<ApiError, Unit> {
        return try {
            val response = httpClient.delete("/api/guidance/epics/$id")
            handleResponse<Unit>(response.status, Unit)
        } catch (e: Exception) {
            handleException(e)
        }
    }

    // ==================== Energy Budget Operations ====================

    /**
     * Retrieve energy budget for a specific date.
     *
     * @param date Date in YYYY-MM-DD format
     * @return [Either] containing energy budget details or an [ApiError]
     */
    suspend fun getEnergyBudget(date: String): Either<ApiError, EnergyBudgetResponse> {
        return try {
            val response = httpClient.get("/api/guidance/energy/$date")
            handleResponse<EnergyBudgetResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    /**
     * Set energy budget for a specific date.
     *
     * Budget must be in range 1-10. Spent value is auto-calculated from completed quests.
     *
     * @param date Date in YYYY-MM-DD format
     * @param request Energy budget parameters
     * @return [Either] containing updated energy budget or an [ApiError]
     */
    suspend fun setEnergyBudget(date: String, request: SetEnergyBudgetRequest): Either<ApiError, EnergyBudgetResponse> {
        return try {
            val response = httpClient.put("/api/guidance/energy/$date") {
                contentType(ContentType.Application.Json)
                setBody(request)
            }
            handleResponse<EnergyBudgetResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    // ==================== Today View ====================

    /**
     * Retrieve comprehensive today view.
     *
     * Aggregates active quest, energy budget, ready quests, routine instances,
     * and completed quests for the current day.
     *
     * @return [Either] containing today view data or an [ApiError]
     */
    suspend fun getTodayView(): Either<ApiError, TodayViewResponse> {
        return try {
            val response = httpClient.get("/api/guidance/today")
            handleResponse<TodayViewResponse>(response.status, response.body())
        } catch (e: Exception) {
            handleException(e)
        }
    }

    // ==================== Private Helper Methods ====================

    /**
     * Handle HTTP response status and convert to Either.
     */
    private fun <T> handleResponse(status: HttpStatusCode, body: T): Either<ApiError, T> {
        return when {
            status.value in 200..299 -> body.right()
            else -> ApiError.ServerError(
                statusCode = status.value,
                message = "Unexpected status: ${status.value}"
            ).left()
        }
    }

    /**
     * Handle exceptions and convert to ApiError.
     */
    private fun <T> handleException(e: Exception): Either<ApiError, T> {
        return when (e) {
            is io.ktor.client.plugins.ClientRequestException -> {
                when (e.response.status.value) {
                    400 -> ApiError.ValidationError("Invalid request: ${e.message}").left()
                    401 -> ApiError.Unauthorized("Authentication required").left()
                    403 -> ApiError.Forbidden("Access denied").left()
                    404 -> ApiError.NotFound("Resource not found").left()
                    409 -> ApiError.Conflict("Conflict: ${e.message}").left()
                    else -> ApiError.ServerError(e.response.status.value, e.message ?: "Client error").left()
                }
            }
            is io.ktor.client.plugins.ServerResponseException -> {
                ApiError.ServerError(
                    statusCode = e.response.status.value,
                    message = e.message ?: "Server error"
                ).left()
            }
            else -> ApiError.NetworkError(e.message ?: "Network error occurred").left()
        }
    }
}

