@file:Suppress("DEPRECATION")

package com.getaltair.server.routes

import arrow.core.getOrElse
import com.getaltair.altair.shared.domain.common.Schedule
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.system.Routine
import com.getaltair.altair.shared.dto.system.*
import com.getaltair.altair.shared.repository.RoutineRepository
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.datetime.Instant
import kotlinx.datetime.DayOfWeek
import kotlinx.datetime.LocalTime

/**
 * Configures Routine routes for recurring task templates.
 *
 * All routes require JWT authentication.
 * Supports scheduled Quest generation and active/inactive management.
 */
fun Route.routineRoutes(routineRepository: RoutineRepository) {
    route("/api/routines") {
        authenticate("jwt") {
            get {
                val userId = call.userId
                routineRepository.getAllForUser(userId).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { routines -> call.respond(routines.map { it.toResponse() }) }
                )
            }

            post {
                val userId = call.userId
                val request = call.receive<CreateRoutineRequest>()

                // Parse schedule string to Schedule domain object
                val schedule = parseSchedule(request.schedule)
                if (schedule == null) {
                    call.respond(HttpStatusCode.BadRequest, "Invalid schedule format")
                    return@post
                }

                val routine = Routine(
                    id = Ulid.generate(),
                    userId = userId,
                    name = request.name,
                    description = request.description,
                    schedule = schedule,
                    timeOfDay = request.timeOfDay?.let { LocalTime.parse(it) },
                    energyCost = request.energyCost,
                    initiativeId = request.initiativeId?.let { Ulid.parse(it) },
                    active = true,
                    nextDue = null, // Will be calculated by scheduler
                    createdAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis()),
                    deletedAt = null
                )

                routineRepository.create(routine).fold(
                    ifLeft = { error -> call.respondError(error) },
                    ifRight = { created -> call.respond(HttpStatusCode.Created, created.toResponse()) }
                )
            }

            get("/{id}") {
                call.pathParamAsUlid("id") { id ->
                    routineRepository.getById(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { routine -> call.respond(routine.toResponse()) }
                    )
                }
            }

            put("/{id}") {
                call.pathParamAsUlid("id") { id ->
                    val request = call.receive<UpdateRoutineRequest>()

                    routineRepository.getById(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { existing ->
                            val schedule = request.schedule?.let { parseSchedule(it) } ?: existing.schedule
                            val newTimeOfDay = request.timeOfDay

                            val updated = existing.copy(
                                name = request.name ?: existing.name,
                                description = request.description ?: existing.description,
                                schedule = schedule,
                                timeOfDay = if (newTimeOfDay != null)
                                    LocalTime.parse(newTimeOfDay) else existing.timeOfDay,
                                energyCost = request.energyCost ?: existing.energyCost,
                                active = request.active ?: existing.active,
                                updatedAt = Instant.fromEpochMilliseconds(System.currentTimeMillis())
                            )

                            routineRepository.update(updated).fold(
                                ifLeft = { error -> call.respondError(error) },
                                ifRight = { routine -> call.respond(routine.toResponse()) }
                            )
                        }
                    )
                }
            }

            delete("/{id}") {
                call.pathParamAsUlid("id") { id ->
                    routineRepository.softDelete(id).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = { call.respond(HttpStatusCode.NoContent) }
                    )
                }
            }

            post("/{id}/activate") {
                call.pathParamAsUlid("id") { id ->
                    routineRepository.setActive(id, true).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = {
                            routineRepository.getById(id).fold(
                                ifLeft = { error -> call.respondError(error) },
                                ifRight = { routine -> call.respond(routine.toResponse()) }
                            )
                        }
                    )
                }
            }

            post("/{id}/deactivate") {
                call.pathParamAsUlid("id") { id ->
                    routineRepository.setActive(id, false).fold(
                        ifLeft = { error -> call.respondError(error) },
                        ifRight = {
                            routineRepository.getById(id).fold(
                                ifLeft = { error -> call.respondError(error) },
                                ifRight = { routine -> call.respond(routine.toResponse()) }
                            )
                        }
                    )
                }
            }
        }
    }
}

// === Helpers ===

/**
 * Parses schedule string to Schedule domain object.
 * Format examples: "daily", "weekly:1,3,5", "monthly:15", "monthly:first:1", "interval:7"
 */
private fun parseSchedule(scheduleStr: String): Schedule? {
    return Schedule.parse(scheduleStr)
}

// === DTO Mappers ===

/**
 * Converts Routine domain model to RoutineResponse DTO.
 */
private fun Routine.toResponse(): RoutineResponse = RoutineResponse(
    id = id.toString(),
    name = name,
    description = description,
    schedule = formatSchedule(schedule),
    scheduleDescription = formatScheduleDescription(schedule),
    timeOfDay = timeOfDay?.toString(),
    energyCost = energyCost,
    initiativeId = initiativeId?.toString(),
    active = active,
    nextDue = nextDue?.toString(),
    createdAt = createdAt.toString(),
    updatedAt = updatedAt.toString()
)

/**
 * Converts Schedule domain object to string format.
 */
private fun formatSchedule(schedule: Schedule): String = schedule.toSerializedString()

/**
 * Converts Schedule domain object to human-readable description.
 */
private fun formatScheduleDescription(schedule: Schedule): String = when (schedule) {
    is Schedule.Daily -> "Every day"
    is Schedule.Weekly -> {
        val dayNames = schedule.daysOfWeek.map { dayOfWeek ->
            when (dayOfWeek) {
                DayOfWeek.MONDAY -> "Monday"
                DayOfWeek.TUESDAY -> "Tuesday"
                DayOfWeek.WEDNESDAY -> "Wednesday"
                DayOfWeek.THURSDAY -> "Thursday"
                DayOfWeek.FRIDAY -> "Friday"
                DayOfWeek.SATURDAY -> "Saturday"
                DayOfWeek.SUNDAY -> "Sunday"
            }
        }
        "Every ${dayNames.joinToString(", ")}"
    }
    is Schedule.MonthlyDate -> "Day ${schedule.dayOfMonth} of each month"
    is Schedule.MonthlyRelative -> {
        val weekStr = when (schedule.week) {
            com.getaltair.altair.shared.domain.common.RelativeWeek.FIRST -> "first"
            com.getaltair.altair.shared.domain.common.RelativeWeek.SECOND -> "second"
            com.getaltair.altair.shared.domain.common.RelativeWeek.THIRD -> "third"
            com.getaltair.altair.shared.domain.common.RelativeWeek.FOURTH -> "fourth"
            com.getaltair.altair.shared.domain.common.RelativeWeek.LAST -> "last"
        }
        val dayName = when (schedule.dayOfWeek) {
            DayOfWeek.MONDAY -> "Monday"
            DayOfWeek.TUESDAY -> "Tuesday"
            DayOfWeek.WEDNESDAY -> "Wednesday"
            DayOfWeek.THURSDAY -> "Thursday"
            DayOfWeek.FRIDAY -> "Friday"
            DayOfWeek.SATURDAY -> "Saturday"
            DayOfWeek.SUNDAY -> "Sunday"
        }
        "Every $weekStr $dayName"
    }
    is Schedule.Interval -> "Every ${schedule.days} days"
}
