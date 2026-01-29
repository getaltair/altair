@file:Suppress("DEPRECATION")

package com.getaltair.server.routes

import arrow.core.getOrElse
import com.getaltair.altair.shared.domain.common.InitiativeStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.system.Initiative
import com.getaltair.altair.shared.dto.system.*
import com.getaltair.altair.shared.repository.InitiativeRepository
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate

/**
 * Configures Initiative routes for cross-module organizational structures.
 *
 * All routes require JWT authentication.
 * Supports hierarchical Projects and Areas with focus management.
 */
fun Route.initiativeRoutes(initiativeRepository: InitiativeRepository) {
    route("/api/initiatives") {
        authenticate("jwt") {
            get {
                val userId = call.userId
                initiativeRepository.getAllForUser(userId).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { initiatives -> call.respond(initiatives.map { it.toResponse() }) }
                )
            }

            post {
                val userId = call.userId
                val request = call.receive<CreateInitiativeRequest>()

                val initiative = Initiative(
                    id = Ulid.generate(),
                    userId = userId,
                    name = request.name,
                    description = request.description,
                    parentId = request.parentId?.let { Ulid.parse(it) },
                    ongoing = request.ongoing,
                    targetDate = request.targetDate?.let { LocalDate.parse(it) },
                    status = InitiativeStatus.ACTIVE,
                    focused = false,
                    createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    deletedAt = null
                )

                initiativeRepository.create(initiative).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { created -> call.respond(HttpStatusCode.Created, created.toResponse()) }
                )
            }

            get("/{id}") {
                call.pathParamAsUlid("id") { id ->
                    initiativeRepository.getById(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { initiative -> call.respond(initiative.toResponse()) }
                    )
                }
            }

            put("/{id}") {
                call.pathParamAsUlid("id") { id ->
                    val request = call.receive<UpdateInitiativeRequest>()

                    initiativeRepository.getById(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { existing ->
                            val newParentId = request.parentId
                            val newTargetDate = request.targetDate

                            val updated = existing.copy(
                                name = request.name ?: existing.name,
                                description = request.description ?: existing.description,
                                parentId = if (newParentId != null)
                                    Ulid.parse(newParentId) else existing.parentId,
                                ongoing = request.ongoing ?: existing.ongoing,
                                targetDate = if (newTargetDate != null)
                                    LocalDate.parse(newTargetDate) else existing.targetDate,
                                status = request.status ?: existing.status,
                                updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                            )

                            initiativeRepository.update(updated).fold(
                                ifLeft = { error -> call.respondError(error) },
                                ifRight = { initiative -> call.respond(initiative.toResponse()) }
                            )
                        }
                    )
                }
            }

            delete("/{id}") {
                call.pathParamAsUlid("id") { id ->
                    initiativeRepository.softDelete(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { call.respond(HttpStatusCode.NoContent) }
                    )
                }
            }

            post("/{id}/focus") {
                call.pathParamAsUlid("id") { id ->
                    val userId = call.userId
                    initiativeRepository.setFocused(userId, id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = {
                            initiativeRepository.getById(id).fold(
                                ifLeft = { error -> call.respondError(error) },
                                ifRight = { initiative -> call.respond(initiative.toResponse()) }
                            )
                        }
                    )
                }
            }

            post("/{id}/unfocus") {
                call.pathParamAsUlid("id") { id ->
                    val userId = call.userId

                    // Verify this is the focused initiative before unfocusing
                    initiativeRepository.getById(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { initiative ->
                            if (initiative.focused) {
                                initiativeRepository.setFocused(userId, null).fold(
                                    ifLeft = { error -> call.respondError(error) },
                                    ifRight = { call.respond(HttpStatusCode.NoContent) }
                                )
                            } else {
                                call.respond(HttpStatusCode.NoContent)
                            }
                        }
                    )
                }
            }
        }
    }
}

// === DTO Mappers ===

/**
 * Converts Initiative domain model to InitiativeResponse DTO.
 */
private fun Initiative.toResponse(): InitiativeResponse = InitiativeResponse(
    id = id.toString(),
    name = name,
    description = description,
    parentId = parentId?.toString(),
    ongoing = ongoing,
    targetDate = targetDate?.toString(),
    status = status,
    focused = focused,
    questCount = 0, // TODO: Query quest count
    noteCount = 0, // TODO: Query note count
    itemCount = 0, // TODO: Query item count
    children = null,
    createdAt = createdAt.toString(),
    updatedAt = updatedAt.toString()
)
