package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.EpicError
import com.getaltair.altair.domain.model.guidance.Epic
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.EpicStatus
import kotlinx.coroutines.flow.Flow

/**
 * Aggregated statistics for an Epic's child quests.
 *
 * All values must be non-negative, and completed values must not exceed totals.
 */
data class EpicProgress(
    /** Total number of quests in the epic (must be >= 0) */
    val totalQuests: Int,
    /** Number of completed quests (must be >= 0 and <= totalQuests) */
    val completedQuests: Int,
    /** Total energy cost of all quests (must be >= 0) */
    val totalEnergy: Int,
    /** Energy spent on completed quests (must be >= 0 and <= totalEnergy) */
    val spentEnergy: Int,
) {
    init {
        require(totalQuests >= 0) { "Total quests must be non-negative" }
        require(completedQuests >= 0) { "Completed quests must be non-negative" }
        require(completedQuests <= totalQuests) { "Completed quests cannot exceed total quests" }
        require(totalEnergy >= 0) { "Total energy must be non-negative" }
        require(spentEnergy >= 0) { "Spent energy must be non-negative" }
        require(spentEnergy <= totalEnergy) { "Spent energy cannot exceed total energy" }
    }

    /** Completion percentage (0-100) */
    val completionPercent: Int
        get() = if (totalQuests > 0) completedQuests * 100 / totalQuests else 0
}

/**
 * Repository for Epic entities.
 *
 * Epics are large goals broken down into smaller Quests. They provide
 * high-level organization for related work.
 */
interface EpicRepository : Repository<Epic, EpicError> {
    /**
     * Finds all epics with the specified status.
     *
     * @param status The status to filter by
     * @return A Flow emitting epics matching the status
     */
    fun findByStatus(status: EpicStatus): Flow<List<Epic>>

    /**
     * Finds epics associated with a specific initiative.
     *
     * @param initiativeId The ULID of the initiative
     * @return A Flow emitting epics for the initiative
     */
    fun findByInitiative(initiativeId: Ulid): Flow<List<Epic>>

    /**
     * Calculates progress statistics for an epic based on its quests.
     *
     * @param id The ULID of the epic
     * @return Either an error on failure, or the progress statistics
     */
    suspend fun getProgress(id: Ulid): Either<EpicError, EpicProgress>

    /**
     * Finds epics with their associated progress in a single query.
     *
     * This is more efficient than calling `findAll()` and `getProgress()`
     * separately for each epic.
     *
     * @return A Flow emitting pairs of epics and their progress
     */
    fun findAllWithProgress(): Flow<List<Pair<Epic, EpicProgress>>>
}
