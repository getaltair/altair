package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.QuestError
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
import kotlinx.coroutines.flow.Flow
import kotlinx.datetime.LocalDate

/**
 * Repository for Quest entities.
 *
 * Quests are the core unit of work in the Guidance module. They have
 * an energy cost and lifecycle status.
 */
interface QuestRepository : Repository<Quest, QuestError> {
    /**
     * Finds all quests with the specified status.
     *
     * @param status The status to filter by
     * @return A Flow emitting quests matching the status
     */
    fun findByStatus(status: QuestStatus): Flow<List<Quest>>

    /**
     * Finds all active (in-progress) quests.
     *
     * This is a convenience method equivalent to `findByStatus(QuestStatus.ACTIVE)`.
     *
     * @return A Flow emitting active quests
     */
    fun findActive(): Flow<List<Quest>>

    /**
     * Finds quests scheduled for a specific date.
     *
     * @param date The scheduled date to filter by
     * @return A Flow emitting quests scheduled for the date
     */
    fun findByScheduledDate(date: LocalDate): Flow<List<Quest>>

    /**
     * Finds quests due on or before a specific date.
     *
     * @param date The due date threshold
     * @return A Flow emitting quests with due dates on or before the date
     */
    fun findDueByDate(date: LocalDate): Flow<List<Quest>>

    /**
     * Finds quests belonging to a specific epic.
     *
     * @param epicId The ULID of the epic
     * @return A Flow emitting quests in the epic
     */
    fun findByEpic(epicId: Ulid): Flow<List<Quest>>

    /**
     * Finds quests spawned from a specific routine.
     *
     * @param routineId The ULID of the routine
     * @return A Flow emitting quests spawned from the routine
     */
    fun findByRoutine(routineId: Ulid): Flow<List<Quest>>

    /**
     * Finds quests associated with a specific initiative.
     *
     * @param initiativeId The ULID of the initiative
     * @return A Flow emitting quests for the initiative
     */
    fun findByInitiative(initiativeId: Ulid): Flow<List<Quest>>

    /**
     * Transitions a quest to a new status.
     *
     * This method enforces valid state transitions:
     * - BACKLOG -> ACTIVE (requires WIP limit check)
     * - ACTIVE -> COMPLETED | ABANDONED
     * - Invalid transitions return [QuestError.InvalidStatusTransition]
     *
     * @param id The ULID of the quest
     * @param newStatus The target status
     * @return Either an error on invalid transition, or the updated quest
     */
    suspend fun transitionStatus(
        id: Ulid,
        newStatus: QuestStatus,
    ): Either<QuestError, Quest>

    /**
     * Counts the number of currently active quests.
     *
     * Used for WIP (work-in-progress) limit enforcement.
     *
     * @return Either an error on failure, or the count of active quests
     */
    suspend fun countActive(): Either<QuestError, Int>
}
