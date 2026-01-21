package com.getaltair.altair.domain.model.guidance

import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlin.time.Instant
import kotlinx.datetime.LocalDate
import kotlinx.serialization.Serializable

/**
 * Daily energy allocation for a user.
 *
 * EnergyBudget tracks how much energy a user has available for a given day
 * and how much has been spent on completed Quests. This helps with workload
 * management and prevents overcommitment.
 */
@Serializable
data class EnergyBudget(
    val id: Ulid,
    val userId: Ulid,
    val date: LocalDate,
    val totalBudget: Int,
    val spentEnergy: Int,
    override val createdAt: Instant,
    override val updatedAt: Instant,
) : Timestamped {
    init {
        require(totalBudget >= 0) { "Total budget must be non-negative" }
        require(spentEnergy >= 0) { "Spent energy must be non-negative" }
    }

    val remainingEnergy: Int get() = (totalBudget - spentEnergy).coerceAtLeast(0)
    val isOverBudget: Boolean get() = spentEnergy > totalBudget
}
