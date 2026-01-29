package com.getaltair.altair.shared.repository

import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.Routine
import kotlinx.datetime.Instant

/**
 * Repository interface for managing Routines (recurring templates).
 *
 * Routines are templates that spawn Quest instances on a schedule (daily, weekly, etc.).
 * They support ADHD-friendly habit formation by automating the creation of recurring
 * tasks without requiring manual repetition.
 */
interface RoutineRepository {
    /**
     * Retrieves a Routine by its unique identifier.
     *
     * @param id The unique identifier of the Routine
     * @return Success with the Routine, or Failure if not found
     */
    suspend fun getById(id: Ulid): AltairResult<Routine>

    /**
     * Retrieves all Routines for a specific user, including inactive ones.
     *
     * @param userId The unique identifier of the user
     * @return Success with list of all Routines (may be empty), or Failure on error
     */
    suspend fun getAllForUser(userId: Ulid): AltairResult<List<Routine>>

    /**
     * Retrieves all active Routines for a specific user.
     * Active Routines are those with isActive=true and no deletedAt timestamp.
     *
     * @param userId The unique identifier of the user
     * @return Success with list of active Routines (may be empty), or Failure on error
     */
    suspend fun getActiveForUser(userId: Ulid): AltairResult<List<Routine>>

    /**
     * Retrieves all active Routines due before a given timestamp.
     * Used by the scheduler to determine which Routines should spawn instances.
     *
     * @param before The cutoff timestamp (typically "now")
     * @return Success with list of due Routines (may be empty), or Failure on error
     */
    suspend fun getDue(before: Instant): AltairResult<List<Routine>>

    /**
     * Creates a new Routine.
     *
     * @param routine The Routine to create (ID should be generated)
     * @return Success with the created Routine (with populated metadata), or Failure on error
     */
    suspend fun create(routine: Routine): AltairResult<Routine>

    /**
     * Updates an existing Routine.
     * Cannot change ID, userId, or timestamps managed by the system.
     *
     * @param routine The Routine with updated fields
     * @return Success with the updated Routine, or Failure if not found or validation fails
     */
    suspend fun update(routine: Routine): AltairResult<Routine>

    /**
     * Updates the nextDue timestamp for a Routine.
     * Called after spawning an instance to schedule the next occurrence.
     *
     * @param id The unique identifier of the Routine
     * @param nextDue The new nextDue timestamp based on recurrence pattern
     * @return Success on update, or Failure if not found
     */
    suspend fun updateNextDue(id: Ulid, nextDue: Instant): AltairResult<Unit>

    /**
     * Activates or deactivates a Routine.
     * Inactive Routines do not spawn instances even if due.
     *
     * @param id The unique identifier of the Routine
     * @param active Whether the Routine should be active
     * @return Success on update, or Failure if not found
     */
    suspend fun setActive(id: Ulid, active: Boolean): AltairResult<Unit>

    /**
     * Soft deletes a Routine by setting deletedAt timestamp.
     * Deleted Routines do not spawn instances and are hidden from default views.
     *
     * @param id The unique identifier of the Routine to delete
     * @return Success on deletion, or Failure if not found
     */
    suspend fun softDelete(id: Ulid): AltairResult<Unit>
}
