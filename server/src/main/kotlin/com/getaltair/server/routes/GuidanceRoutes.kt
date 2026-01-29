@file:Suppress("DEPRECATION")

package com.getaltair.server.routes

import arrow.core.getOrElse
import com.getaltair.altair.shared.domain.common.EpicStatus
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.guidance.Checkpoint
import com.getaltair.altair.shared.domain.guidance.Epic
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.dto.auth.ErrorResponse
import com.getaltair.altair.shared.dto.guidance.*
import com.getaltair.altair.shared.repository.EpicRepository
import com.getaltair.altair.shared.repository.QuestRepository
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime

/**
 * Guidance module routes for Quest, Epic, Checkpoint, and Energy Budget management.
 *
 * All routes require JWT authentication. Routes are organized by entity:
 * - /api/guidance/quests - Quest CRUD and status transitions
 * - /api/guidance/quests/{id}/checkpoints - Checkpoint management
 * - /api/guidance/checkpoints/{id} - Individual checkpoint operations
 * - /api/guidance/epics - Epic CRUD
 * - /api/guidance/energy/{date} - Energy budget management
 * - /api/guidance/today - Today view aggregation
 */
fun Route.guidanceRoutes(questRepository: QuestRepository, epicRepository: EpicRepository) {
    route("/api/guidance") {
        authenticate("jwt") {

            // ==================== Quest Operations ====================

            /**
             * GET /api/guidance/quests
             * Retrieve all quests for the authenticated user.
             * Optional query parameter: status (BACKLOG, ACTIVE, COMPLETED, ABANDONED)
             */
            get("/quests") {
                val statusParam = call.request.queryParameters["status"]
                val userId = call.userId

                val result = if (statusParam != null) {
                    val status = try {
                        QuestStatus.valueOf(statusParam.uppercase())
                    } catch (e: IllegalArgumentException) {
                        call.respond(
                            HttpStatusCode.BadRequest,
                            ErrorResponse("INVALID_STATUS", "Invalid status: $statusParam")
                        )
                        return@get
                    }
                    questRepository.getByStatus(userId, status)
                } else {
                    questRepository.getAllForUser(userId)
                }

                result.fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { quests ->
                        val response = quests.map { quest ->
                            // Get checkpoint counts for summary
                            val checkpoints = questRepository.getCheckpoints(quest.id)
                                .getOrElse { emptyList() }
                            QuestSummaryResponse(
                                id = quest.id.toString(),
                                title = quest.title,
                                energyCost = quest.energyCost,
                                status = quest.status,
                                epicId = quest.epicId?.toString(),
                                checkpointCount = checkpoints.size,
                                completedCheckpointCount = checkpoints.count { it.completed }
                            )
                        }
                        call.respond(HttpStatusCode.OK, response)
                    }
                )
            }

            /**
             * POST /api/guidance/quests
             * Create a new quest.
             */
            post("/quests") {
                val request = call.receive<CreateQuestRequest>()
                val userId = call.userId

                val epicId = request.epicId?.let { epicIdStr ->
                    val parsed = Ulid.parse(epicIdStr)
                    if (parsed == null) {
                        call.respond(
                            HttpStatusCode.BadRequest,
                            ErrorResponse("INVALID_EPIC_ID", "Invalid epicId format: $epicIdStr")
                        )
                        return@post
                    }
                    parsed
                }

                val quest = Quest(
                    id = Ulid.generate(),
                    userId = userId,
                    title = request.title,
                    description = request.description,
                    energyCost = request.energyCost,
                    status = QuestStatus.BACKLOG,
                    epicId = epicId,
                    routineId = null,
                    createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    startedAt = null,
                    completedAt = null,
                    deletedAt = null
                )

                questRepository.create(quest).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { created ->
                        val response = created.toResponse(emptyList())
                        call.respond(HttpStatusCode.Created, response)
                    }
                )
            }

            /**
             * GET /api/guidance/quests/{id}
             * Retrieve detailed quest information including checkpoints.
             */
            get("/quests/{id}") {
                call.pathParamAsUlid("id") { questId ->
                    questRepository.getById(questId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { quest ->
                            val checkpoints = questRepository.getCheckpoints(quest.id)
                                .getOrElse { emptyList() }
                            val response = quest.toResponse(checkpoints)
                            call.respond(HttpStatusCode.OK, response)
                        }
                    )
                }
            }

            /**
             * PUT /api/guidance/quests/{id}
             * Update quest fields.
             */
            put("/quests/{id}") {
                call.pathParamAsUlid("id") { questId ->
                    val request = call.receive<UpdateQuestRequest>()

                    questRepository.getById(questId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { existing ->
                            val epicId = request.epicId?.let { epicIdStr ->
                                val parsed = Ulid.parse(epicIdStr)
                                if (parsed == null) {
                                    call.respond(
                                        HttpStatusCode.BadRequest,
                                        ErrorResponse("INVALID_EPIC_ID", "Invalid epicId format: $epicIdStr")
                                    )
                                    return@pathParamAsUlid
                                }
                                parsed
                            }

                            val updated = existing.copy(
                                title = request.title ?: existing.title,
                                description = request.description ?: existing.description,
                                energyCost = request.energyCost ?: existing.energyCost,
                                epicId = epicId ?: existing.epicId,
                                updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                            )

                            questRepository.update(updated).fold(
                                ifLeft = { error -> call.respondError(error) },
                                ifRight = { savedQuest ->
                                    val checkpoints = questRepository.getCheckpoints(savedQuest.id)
                                        .getOrElse { emptyList() }
                                    val response = savedQuest.toResponse(checkpoints)
                                    call.respond(HttpStatusCode.OK, response)
                                }
                            )
                        }
                    )
                }
            }

            /**
             * DELETE /api/guidance/quests/{id}
             * Soft-delete a quest.
             */
            delete("/quests/{id}") {
                call.pathParamAsUlid("id") { questId ->
                    questRepository.softDelete(questId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { call.respond(HttpStatusCode.NoContent) }
                    )
                }
            }

            // ==================== Quest Status Transitions ====================

            /**
             * POST /api/guidance/quests/{id}/start
             * Start a quest (BACKLOG → ACTIVE).
             * WIP=1 enforcement: returns 409 Conflict if another quest is active.
             */
            post("/quests/{id}/start") {
                call.pathParamAsUlid("id") { questId ->
                    questRepository.start(questId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { quest ->
                            val checkpoints = questRepository.getCheckpoints(quest.id)
                                .getOrElse { emptyList() }
                            call.respond(HttpStatusCode.OK, quest.toResponse(checkpoints))
                        }
                    )
                }
            }

            /**
             * POST /api/guidance/quests/{id}/complete
             * Complete a quest (ACTIVE → COMPLETED).
             */
            post("/quests/{id}/complete") {
                call.pathParamAsUlid("id") { questId ->
                    questRepository.complete(questId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { quest ->
                            val checkpoints = questRepository.getCheckpoints(quest.id)
                                .getOrElse { emptyList() }
                            call.respond(HttpStatusCode.OK, quest.toResponse(checkpoints))
                        }
                    )
                }
            }

            /**
             * POST /api/guidance/quests/{id}/abandon
             * Abandon a quest (any → ABANDONED).
             */
            post("/quests/{id}/abandon") {
                call.pathParamAsUlid("id") { questId ->
                    questRepository.abandon(questId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { quest ->
                            val checkpoints = questRepository.getCheckpoints(quest.id)
                                .getOrElse { emptyList() }
                            call.respond(HttpStatusCode.OK, quest.toResponse(checkpoints))
                        }
                    )
                }
            }

            /**
             * POST /api/guidance/quests/{id}/backlog
             * Return quest to backlog (ACTIVE → BACKLOG).
             */
            post("/quests/{id}/backlog") {
                call.pathParamAsUlid("id") { questId ->
                    questRepository.backlog(questId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { quest ->
                            val checkpoints = questRepository.getCheckpoints(quest.id)
                                .getOrElse { emptyList() }
                            call.respond(HttpStatusCode.OK, quest.toResponse(checkpoints))
                        }
                    )
                }
            }

            // ==================== Checkpoint Operations ====================

            /**
             * GET /api/guidance/quests/{id}/checkpoints
             * Retrieve all checkpoints for a quest in display order.
             */
            get("/quests/{questId}/checkpoints") {
                call.pathParamAsUlid("questId") { questId ->
                    questRepository.getCheckpoints(questId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { checkpoints ->
                            val response = checkpoints.map { it.toResponse() }
                            call.respond(HttpStatusCode.OK, response)
                        }
                    )
                }
            }

            /**
             * POST /api/guidance/quests/{id}/checkpoints
             * Add a new checkpoint to a quest.
             */
            post("/quests/{questId}/checkpoints") {
                call.pathParamAsUlid("questId") { questId ->
                    val request = call.receive<CreateCheckpointRequest>()

                    // Calculate order if not provided
                    val existingCheckpoints = questRepository.getCheckpoints(questId)
                        .getOrElse { emptyList() }
                    val order = request.order ?: (existingCheckpoints.maxOfOrNull { it.order } ?: -1) + 1

                    val checkpoint = Checkpoint(
                        id = Ulid.generate(),
                        questId = questId,
                        title = request.title,
                        completed = false,
                        order = order,
                        completedAt = null
                    )

                    questRepository.addCheckpoint(checkpoint).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { created ->
                            call.respond(HttpStatusCode.Created, created.toResponse())
                        }
                    )
                }
            }

            /**
             * POST /api/guidance/quests/{id}/checkpoints/reorder
             * Reorder checkpoints within a quest.
             */
            post("/quests/{questId}/checkpoints/reorder") {
                call.pathParamAsUlid("questId") { questId ->
                    val request = call.receive<ReorderCheckpointsRequest>()

                    val orderUlids = request.order.map { idStr ->
                        val parsed = Ulid.parse(idStr)
                        if (parsed == null) {
                            call.respond(
                                HttpStatusCode.BadRequest,
                                ErrorResponse("INVALID_CHECKPOINT_ID", "Invalid checkpoint ID: $idStr")
                            )
                            return@pathParamAsUlid
                        }
                        parsed
                    }

                    questRepository.reorderCheckpoints(questId, orderUlids).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { call.respond(HttpStatusCode.NoContent) }
                    )
                }
            }

            /**
             * PUT /api/guidance/checkpoints/{id}
             * Update a checkpoint (title, completed status, order).
             */
            put("/checkpoints/{id}") {
                call.pathParamAsUlid("id") { checkpointId ->
                    val request = call.receive<UpdateCheckpointRequest>()

                    // TODO: Implement checkpoint update via repository method
                    // For now, return not implemented
                    call.respond(
                        HttpStatusCode.NotImplemented,
                        ErrorResponse("NOT_IMPLEMENTED", "Direct checkpoint update not yet implemented")
                    )
                }
            }

            /**
             * DELETE /api/guidance/checkpoints/{id}
             * Delete a checkpoint (hard delete).
             */
            delete("/checkpoints/{id}") {
                call.pathParamAsUlid("id") { checkpointId ->
                    questRepository.deleteCheckpoint(checkpointId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { call.respond(HttpStatusCode.NoContent) }
                    )
                }
            }

            // ==================== Epic Operations ====================

            /**
             * GET /api/guidance/epics
             * Retrieve all epics for the authenticated user.
             */
            get("/epics") {
                val userId = call.userId

                epicRepository.getAllForUser(userId).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { epics ->
                        val response = epics.map { epic ->
                            // Get quest counts for summary
                            val quests = questRepository.getByEpic(epic.id)
                                .getOrElse { emptyList() }
                            epic.toResponse(quests)
                        }
                        call.respond(HttpStatusCode.OK, response)
                    }
                )
            }

            /**
             * POST /api/guidance/epics
             * Create a new epic.
             */
            post("/epics") {
                val request = call.receive<CreateEpicRequest>()
                val userId = call.userId

                val initiativeId = request.initiativeId?.let { initIdStr ->
                    val parsed = Ulid.parse(initIdStr)
                    if (parsed == null) {
                        call.respond(
                            HttpStatusCode.BadRequest,
                            ErrorResponse("INVALID_INITIATIVE_ID", "Invalid initiativeId format: $initIdStr")
                        )
                        return@post
                    }
                    parsed
                }

                val epic = Epic(
                    id = Ulid.generate(),
                    userId = userId,
                    title = request.title,
                    description = request.description,
                    status = EpicStatus.ACTIVE,
                    initiativeId = initiativeId,
                    createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    completedAt = null,
                    deletedAt = null
                )

                epicRepository.create(epic).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { created ->
                        val response = created.toResponse(emptyList())
                        call.respond(HttpStatusCode.Created, response)
                    }
                )
            }

            /**
             * GET /api/guidance/epics/{id}
             * Retrieve detailed epic information.
             */
            get("/epics/{id}") {
                call.pathParamAsUlid("id") { epicId ->
                    epicRepository.getById(epicId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { epic ->
                            val quests = questRepository.getByEpic(epic.id)
                                .getOrElse { emptyList() }
                            val response = epic.toResponse(quests)
                            call.respond(HttpStatusCode.OK, response)
                        }
                    )
                }
            }

            /**
             * PUT /api/guidance/epics/{id}
             * Update epic fields.
             */
            put("/epics/{id}") {
                call.pathParamAsUlid("id") { epicId ->
                    val request = call.receive<UpdateEpicRequest>()

                    epicRepository.getById(epicId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { existing ->
                            val initiativeId = request.initiativeId?.let { initIdStr ->
                                val parsed = Ulid.parse(initIdStr)
                                if (parsed == null) {
                                    call.respond(
                                        HttpStatusCode.BadRequest,
                                        ErrorResponse("INVALID_INITIATIVE_ID", "Invalid initiativeId format: $initIdStr")
                                    )
                                    return@pathParamAsUlid
                                }
                                parsed
                            }

                            val updated = existing.copy(
                                title = request.title ?: existing.title,
                                description = request.description ?: existing.description,
                                initiativeId = initiativeId ?: existing.initiativeId,
                                updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                            )

                            epicRepository.update(updated).fold(
                                ifLeft = { error -> call.respondError(error) },
                                ifRight = { savedEpic ->
                                    val quests = questRepository.getByEpic(savedEpic.id)
                                        .getOrElse { emptyList() }
                                    val response = savedEpic.toResponse(quests)
                                    call.respond(HttpStatusCode.OK, response)
                                }
                            )
                        }
                    )
                }
            }

            /**
             * DELETE /api/guidance/epics/{id}
             * Soft-delete an epic.
             */
            delete("/epics/{id}") {
                call.pathParamAsUlid("id") { epicId ->
                    epicRepository.softDelete(epicId).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { call.respond(HttpStatusCode.NoContent) }
                    )
                }
            }

            // ==================== Energy Budget Operations ====================

            /**
             * GET /api/guidance/energy/{date}
             * Retrieve energy budget for a specific date.
             */
            get("/energy/{date}") {
                val dateStr = call.parameters["date"]
                if (dateStr == null) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        ErrorResponse("MISSING_DATE", "Date parameter is required")
                    )
                    return@get
                }

                val date = try {
                    LocalDate.parse(dateStr)
                } catch (e: Exception) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        ErrorResponse("INVALID_DATE", "Invalid date format: $dateStr (expected YYYY-MM-DD)")
                    )
                    return@get
                }

                val userId = call.userId

                questRepository.getEnergyBudget(userId, date).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { budget ->
                        val response = EnergyBudgetResponse(
                            date = date.toString(),
                            budget = budget.budget,
                            spent = budget.spent,
                            remaining = budget.remaining,
                            percentUsed = if (budget.budget > 0) budget.spent.toFloat() / budget.budget else 0f
                        )
                        call.respond(HttpStatusCode.OK, response)
                    }
                )
            }

            /**
             * PUT /api/guidance/energy/{date}
             * Set energy budget for a specific date.
             */
            put("/energy/{date}") {
                val dateStr = call.parameters["date"]
                if (dateStr == null) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        ErrorResponse("MISSING_DATE", "Date parameter is required")
                    )
                    return@put
                }

                val date = try {
                    LocalDate.parse(dateStr)
                } catch (e: Exception) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        ErrorResponse("INVALID_DATE", "Invalid date format: $dateStr (expected YYYY-MM-DD)")
                    )
                    return@put
                }

                val request = call.receive<SetEnergyBudgetRequest>()
                val userId = call.userId

                questRepository.setDailyBudget(userId, date, request.budget).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { budget ->
                        val response = EnergyBudgetResponse(
                            date = date.toString(),
                            budget = budget.budget,
                            spent = budget.spent,
                            remaining = budget.remaining,
                            percentUsed = if (budget.budget > 0) budget.spent.toFloat() / budget.budget else 0f
                        )
                        call.respond(HttpStatusCode.OK, response)
                    }
                )
            }

            // ==================== Today View ====================

            /**
             * GET /api/guidance/today
             * Retrieve comprehensive today view with active quest, energy budget,
             * ready quests, routine instances, and completed quests.
             */
            get("/today") {
                val userId = call.userId
                val today = Instant.fromEpochMilliseconds(System.currentTimeMillis()).toLocalDateTime(TimeZone.currentSystemDefault()).date

                // Get energy budget
                val energyBudget = questRepository.getEnergyBudget(userId, today)
                    .getOrElse {
                        call.respondError(it)
                        return@get
                    }

                // Get active quest (WIP=1)
                val activeQuest = questRepository.getActiveQuest(userId)
                    .getOrElse {
                        call.respondError(it)
                        return@get
                    }

                val activeQuestResponse = activeQuest?.let { quest ->
                    val checkpoints = questRepository.getCheckpoints(quest.id).getOrElse { emptyList() }
                    quest.toResponse(checkpoints)
                }

                // Get quests by status
                val allQuests = questRepository.getAllForUser(userId).getOrElse { emptyList() }

                val readyQuests = allQuests
                    .filter { it.status == QuestStatus.BACKLOG }
                    .map { quest ->
                        val checkpoints = questRepository.getCheckpoints(quest.id).getOrElse { emptyList() }
                        QuestSummaryResponse(
                            id = quest.id.toString(),
                            title = quest.title,
                            energyCost = quest.energyCost,
                            status = quest.status,
                            epicId = quest.epicId?.toString(),
                            checkpointCount = checkpoints.size,
                            completedCheckpointCount = checkpoints.count { it.completed }
                        )
                    }

                // Routine instances - quests that have routineId set
                val routineInstances = allQuests
                    .filter { it.routineId != null && it.status != QuestStatus.COMPLETED && it.status != QuestStatus.ABANDONED }
                    .map { quest ->
                        val checkpoints = questRepository.getCheckpoints(quest.id).getOrElse { emptyList() }
                        QuestSummaryResponse(
                            id = quest.id.toString(),
                            title = quest.title,
                            energyCost = quest.energyCost,
                            status = quest.status,
                            epicId = quest.epicId?.toString(),
                            checkpointCount = checkpoints.size,
                            completedCheckpointCount = checkpoints.count { it.completed }
                        )
                    }

                // Get quests completed today
                val completedToday = questRepository.getTodayQuests(userId, today)
                    .getOrElse { emptyList() }
                    .filter { it.status == QuestStatus.COMPLETED }
                    .map { quest ->
                        val checkpoints = questRepository.getCheckpoints(quest.id).getOrElse { emptyList() }
                        QuestSummaryResponse(
                            id = quest.id.toString(),
                            title = quest.title,
                            energyCost = quest.energyCost,
                            status = quest.status,
                            epicId = quest.epicId?.toString(),
                            checkpointCount = checkpoints.size,
                            completedCheckpointCount = checkpoints.count { it.completed }
                        )
                    }

                val response = TodayViewResponse(
                    date = today.toString(),
                    energyBudget = EnergyBudgetResponse(
                        date = today.toString(),
                        budget = energyBudget.budget,
                        spent = energyBudget.spent,
                        remaining = energyBudget.remaining,
                        percentUsed = if (energyBudget.budget > 0) energyBudget.spent.toFloat() / energyBudget.budget else 0f
                    ),
                    activeQuest = activeQuestResponse,
                    readyQuests = readyQuests,
                    routineInstances = routineInstances,
                    completedToday = completedToday
                )

                call.respond(HttpStatusCode.OK, response)
            }
        }
    }
}

// ==================== DTO Mapper Extensions ====================

/**
 * Convert Quest domain entity to QuestResponse DTO.
 */
private fun Quest.toResponse(checkpoints: List<Checkpoint>): QuestResponse = QuestResponse(
    id = id.toString(),
    title = title,
    description = description,
    energyCost = energyCost,
    status = status,
    epicId = epicId?.toString(),
    routineId = routineId?.toString(),
    checkpoints = checkpoints.map { it.toResponse() },
    createdAt = createdAt.toString(),
    updatedAt = updatedAt.toString(),
    startedAt = startedAt?.toString(),
    completedAt = completedAt?.toString()
)

/**
 * Convert Checkpoint domain entity to CheckpointResponse DTO.
 */
private fun Checkpoint.toResponse(): CheckpointResponse = CheckpointResponse(
    id = id.toString(),
    title = title,
    completed = completed,
    order = order,
    completedAt = completedAt?.toString()
)

/**
 * Convert Epic domain entity to EpicResponse DTO.
 */
private fun Epic.toResponse(quests: List<Quest>): EpicResponse = EpicResponse(
    id = id.toString(),
    title = title,
    description = description,
    status = status,
    initiativeId = initiativeId?.toString(),
    questCount = quests.size,
    completedQuestCount = quests.count { it.status == QuestStatus.COMPLETED },
    createdAt = createdAt.toString(),
    completedAt = completedAt?.toString()
)
