package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Schedule
import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalTime
import kotlinx.serialization.Serializable

/**
 * A recurring template that spawns Quest instances on a schedule.
 *
 * Routines define habits and recurring tasks. When triggered by their schedule,
 * they generate Quest instances that the user can complete. This separates the
 * template definition from the actual work instances.
 */
@Serializable
data class Routine(
    val id: Ulid,
    val userId: Ulid,
    val title: String,
    val description: String?,
    val energyCost: Int,
    val schedule: Schedule,
    val scheduledTime: LocalTime?,
    val initiativeId: Ulid?,
    val isActive: Boolean,
    val lastSpawnedAt: Instant?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped, SoftDeletable {
    init {
        require(title.isNotBlank()) { "Routine title must not be blank" }
        require(title.length <= 200) { "Routine title must be at most 200 characters" }
        require(energyCost in 1..5) { "Energy cost must be between 1 and 5" }
    }
}
