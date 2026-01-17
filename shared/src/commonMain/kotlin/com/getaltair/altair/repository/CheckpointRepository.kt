package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.guidance.Checkpoint
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Checkpoint entities.
 *
 * Checkpoints are optional sub-steps within a Quest. They are simpler
 * than Quests (no energy cost) and are ordered within their parent Quest.
 */
interface CheckpointRepository : Repository<Checkpoint, DomainError> {
    /**
     * Finds all checkpoints for a specific quest, ordered by sortOrder.
     *
     * @param questId The ULID of the parent quest
     * @return A Flow emitting checkpoints in order
     */
    fun findByQuest(questId: Ulid): Flow<List<Checkpoint>>

    /**
     * Toggles the completion status of a checkpoint.
     *
     * If currently incomplete, marks it complete with current timestamp.
     * If currently complete, marks it incomplete and clears the timestamp.
     *
     * @param id The ULID of the checkpoint
     * @return Either an error on failure, or the updated checkpoint
     */
    suspend fun toggleComplete(id: Ulid): Either<DomainError, Checkpoint>

    /**
     * Reorders checkpoints within a quest.
     *
     * Updates the sortOrder of all checkpoints to match the provided order.
     *
     * @param questId The ULID of the parent quest
     * @param orderedIds The checkpoint IDs in the desired order
     * @return Either an error on failure, or Unit on success
     */
    suspend fun reorder(
        questId: Ulid,
        orderedIds: List<Ulid>,
    ): Either<DomainError, Unit>

    /**
     * Counts checkpoints for a quest, optionally filtered by completion status.
     *
     * @param questId The ULID of the quest
     * @param completed If provided, only count checkpoints with this completion status
     * @return Either an error on failure, or the count
     */
    suspend fun countByQuest(
        questId: Ulid,
        completed: Boolean? = null,
    ): Either<DomainError, Int>
}
