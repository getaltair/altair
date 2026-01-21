package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.Routine
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow
import kotlin.time.Instant
import kotlinx.datetime.LocalDate

/**
 * Repository for Routine entities.
 *
 * Routines are recurring templates that spawn Quest instances on a schedule.
 * They define habits and recurring tasks.
 */
interface RoutineRepository : Repository<Routine, DomainError> {
    /**
     * Finds all active routines.
     *
     * @return A Flow emitting active routines
     */
    fun findActive(): Flow<List<Routine>>

    /**
     * Finds routines that are due to spawn quests on or before the given date.
     *
     * A routine is due if:
     * - It is active
     * - Its schedule matches the date
     * - It hasn't spawned a quest for this occurrence yet
     *
     * @param date The date to check schedules against
     * @return Either an error on failure, or routines due for spawning
     */
    suspend fun findDueForDate(date: LocalDate): Either<DomainError, List<Routine>>

    /**
     * Finds routines associated with a specific initiative.
     *
     * @param initiativeId The ULID of the initiative
     * @return A Flow emitting routines for the initiative
     */
    fun findByInitiative(initiativeId: Ulid): Flow<List<Routine>>

    /**
     * Updates the last spawned timestamp for a routine.
     *
     * This should be called after a quest has been spawned from the routine.
     *
     * @param id The ULID of the routine
     * @param spawnedAt When the quest was spawned
     * @return Either an error on failure, or the updated routine
     */
    suspend fun updateLastSpawnedAt(
        id: Ulid,
        spawnedAt: Instant,
    ): Either<DomainError, Routine>
}
