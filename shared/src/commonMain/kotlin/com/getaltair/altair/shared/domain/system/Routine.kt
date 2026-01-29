package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.Schedule
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalTime
import kotlinx.serialization.Serializable

/**
 * Recurring template that spawns Quest instances based on a schedule.
 *
 * Routines enable ADHD-friendly recurring task management by automatically
 * generating Quest instances at scheduled times, reducing cognitive load
 * and decision fatigue.
 *
 * ## Scheduling
 * - [schedule] defines recurrence pattern (daily, weekly, monthly, interval)
 * - [timeOfDay] specifies when the Quest should be generated (null = any time)
 * - [nextDue] is auto-calculated based on schedule and last spawn time
 *
 * ## Quest Generation
 * - Active routines spawn new Quest instances when [nextDue] is reached
 * - Each spawned Quest inherits [energyCost] and [initiativeId]
 * - Generation happens via background job in the service layer
 *
 * ## Energy Cost
 * - Pre-defined energy cost (1-5) for spawned Quests
 * - Helps with daily capacity planning
 * - Same value used for all Quest instances
 *
 * ## Lifecycle
 * - [active]=true: Routine spawns Quests on schedule
 * - [active]=false: Routine is paused, no new Quests generated
 * - Soft deletion via [deletedAt] preserves history
 *
 * @property id Unique identifier for this routine
 * @property userId Owner of this routine
 * @property name Display name of the routine (max 200 characters)
 * @property description Optional markdown description
 * @property schedule Recurrence pattern (daily, weekly, monthly, etc.)
 * @property timeOfDay Specific time when Quest should spawn (null = any time)
 * @property energyCost Energy cost for spawned Quests (1-5)
 * @property initiativeId Optional initiative to assign to spawned Quests
 * @property active Whether this routine is currently spawning Quests
 * @property nextDue Next scheduled spawn time (null if never spawned)
 * @property createdAt When this routine was created
 * @property updatedAt When this routine was last modified
 * @property deletedAt When this routine was soft-deleted (null if active)
 */
@Serializable
data class Routine(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val schedule: Schedule,
    val timeOfDay: LocalTime?,
    val energyCost: Int,
    val initiativeId: Ulid?,
    val active: Boolean,
    val nextDue: Instant?,
    val createdAt: Instant,
    val updatedAt: Instant,
    val deletedAt: Instant?
) {
    init {
        require(name.length <= 200) { "Name max 200 chars" }
        require(name.isNotBlank()) { "Name required" }
        require(energyCost in 1..5) { "Energy cost must be 1-5" }
    }

    /**
     * Whether this routine has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null

    /**
     * Whether this routine is currently generating Quests.
     * (Active AND not deleted)
     */
    val isActive: Boolean get() = active && !isDeleted
}
