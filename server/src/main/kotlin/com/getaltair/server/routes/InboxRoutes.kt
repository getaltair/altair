@file:Suppress("DEPRECATION")

package com.getaltair.server.routes

import arrow.core.getOrElse
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.domain.knowledge.Note
import com.getaltair.altair.shared.domain.system.InboxItem
import com.getaltair.altair.shared.domain.tracking.Item
import com.getaltair.altair.shared.dto.system.*
import com.getaltair.altair.shared.repository.InboxRepository
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.datetime.Instant

/**
 * Configures Universal Inbox routes for quick capture and triage.
 *
 * All routes require JWT authentication.
 * Supports type-agnostic capture with triage to Quest, Note, Item, or SourceDocument.
 */
fun Route.inboxRoutes(inboxRepository: InboxRepository) {
    route("/api/inbox") {
        authenticate("jwt") {
            get {
                val userId = call.userId
                inboxRepository.getAllForUser(userId).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { items -> call.respond(items.map { it.toResponse() }) }
                )
            }

            post {
                val userId = call.userId
                val request = call.receive<CreateInboxItemRequest>()

                val item = InboxItem(
                    id = Ulid.generate(),
                    userId = userId,
                    content = request.content,
                    source = request.source,
                    attachmentIds = emptyList(),
                    createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    deletedAt = null
                )

                inboxRepository.create(item).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { created -> call.respond(HttpStatusCode.Created, created.toResponse()) }
                )
            }

            get("/{id}") {
                call.pathParamAsUlid("id") { id ->
                    inboxRepository.getById(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { item -> call.respond(item.toResponse()) }
                    )
                }
            }

            delete("/{id}") {
                call.pathParamAsUlid("id") { id ->
                    inboxRepository.softDelete(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { call.respond(HttpStatusCode.NoContent) }
                    )
                }
            }

            post("/triage/{id}") {
                call.pathParamAsUlid("id") { id ->
                    val request = call.receive<TriageRequest>()

                    val result = when (request.targetType.lowercase()) {
                        "quest" -> {
                            val energyCost = request.energyCost
                                ?: run {
                                    call.respond(HttpStatusCode.BadRequest, "energyCost required for Quest")
                                    return@pathParamAsUlid
                                }

                            val quest = Quest(
                                id = Ulid.generate(),
                                userId = call.userId,
                                title = request.title,
                                description = null,
                                energyCost = energyCost,
                                status = QuestStatus.BACKLOG,
                                epicId = null,
                                routineId = null,
                                createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                                updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                                startedAt = null,
                                completedAt = null,
                                deletedAt = null
                            )

                            inboxRepository.triageToQuest(id, quest).map { entityId ->
                                TriageResponse(targetType = "quest", entityId = entityId.toString())
                            }
                        }

                        "note" -> {
                            val note = Note(
                                id = Ulid.generate(),
                                userId = call.userId,
                                title = request.title,
                                content = "",
                                folderId = request.folderId?.let { Ulid.parse(it) },
                                initiativeId = request.initiativeId?.let { Ulid.parse(it) },
                                embedding = null,
                                createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                                updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                                deletedAt = null
                            )

                            inboxRepository.triageToNote(id, note).map { entityId ->
                                TriageResponse(targetType = "note", entityId = entityId.toString())
                            }
                        }

                        "item" -> {
                            val item = Item(
                                id = Ulid.generate(),
                                userId = call.userId,
                                name = request.title,
                                description = null,
                                quantity = 1,
                                templateId = null,
                                locationId = request.locationId?.let { Ulid.parse(it) },
                                containerId = null,
                                initiativeId = request.initiativeId?.let { Ulid.parse(it) },
                                imageKey = null,
                                createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                                updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                                deletedAt = null
                            )

                            inboxRepository.triageToItem(id, item).map { entityId ->
                                TriageResponse(targetType = "item", entityId = entityId.toString())
                            }
                        }

                        else -> {
                            call.respond(HttpStatusCode.BadRequest, "Invalid targetType: ${request.targetType}")
                            return@pathParamAsUlid
                        }
                    }

                    result.fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { response -> call.respond(HttpStatusCode.Created, response) }
                    )
                }
            }
        }
    }
}

// === DTO Mappers ===

/**
 * Converts InboxItem domain model to InboxItemResponse DTO.
 */
private fun InboxItem.toResponse(): InboxItemResponse = InboxItemResponse(
    id = id.toString(),
    content = content,
    source = source,
    attachments = emptyList(), // TODO: Fetch attachment metadata
    createdAt = createdAt.toString()
)
