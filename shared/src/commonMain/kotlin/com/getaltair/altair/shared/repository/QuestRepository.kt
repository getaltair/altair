package com.getaltair.altair.shared.repository

import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Checkpoint
import com.getaltair.altair.shared.domain.guidance.EnergyBudget
import com.getaltair.altair.shared.domain.guidance.Quest
import kotlinx.datetime.LocalDate

/**
 * Repository interface for Quest operations with WIP (Work-in-Progress) enforcement.
 *
 * This repository provides complete CRUD operations for Quests, including status
 * transitions, checkpoint management, and energy budget tracking. All operations
 * return [AltairResult] for consistent error handling.
 *
 * ## WIP=1 Enforcement
 *
 * By default, only **one Quest can be ACTIVE at a time per user** (WIP=1 limit).
 * The [start] method enforces this constraint and returns [AltairError.ConflictError.WipLimitExceeded]
 * if a user attempts to start a quest while another is already active.
 *
 * **Rationale:** WIP limits reduce context switching and improve task completion
 * rates, particularly beneficial for neurodivergent users with ADHD.
 *
 * ## Energy Budget System
 *
 * Quests have an energy cost (1-5) representing cognitive/emotional load. The
 * repository tracks daily energy budgets to enable capacity-aware planning.
 *
 * - [getEnergyBudget] - Retrieve budget for a specific date
 * - [setDailyBudget] - Update available energy for a given day
 *
 * ## Soft Delete
 *
 * Quests are soft-deleted via [softDelete], setting the `deletedAt` timestamp
 * while preserving data for audit purposes. Deleted quests are excluded from
 * standard query results. Use [restore] to un-delete a quest.
 *
 * ## Status Transitions
 *
 * Status changes should use dedicated transition methods rather than direct
 * [update] calls to ensure proper timestamp handling and constraint enforcement:
 *
 * - [start] - BACKLOG → ACTIVE (WIP check enforced)
 * - [complete] - ACTIVE → COMPLETED
 * - [abandon] - Any status → ABANDONED
 * - [backlog] - ACTIVE → BACKLOG (release WIP slot)
 *
 * ## Checkpoints
 *
 * Quests can have multiple checkpoints for granular progress tracking:
 *
 * - [getCheckpoints] - Retrieve all checkpoints for a quest
 * - [addCheckpoint] - Create a new checkpoint
 * - [updateCheckpoint] - Modify checkpoint (e.g., mark complete)
 * - [deleteCheckpoint] - Remove a checkpoint
 * - [reorderCheckpoints] - Update display order of checkpoints
 *
 * ## Error Handling
 *
 * All methods return [AltairResult] with typed errors:
 *
 * - [AltairError.NotFoundError.QuestNotFound] - Quest ID doesn't exist
 * - [AltairError.ConflictError.WipLimitExceeded] - Too many active quests
 * - [AltairError.ValidationError] - Invalid input data
 * - [AltairError.StorageError] - Database operation failed
 *
 * ## Example Usage
 *
 * ```kotlin
 * // Start a quest (WIP check enforced)
 * questRepository.start(questId).fold(
 *     ifLeft = { error ->
 *         when (error) {
 *             is AltairError.ConflictError.WipLimitExceeded ->
 *                 println("Cannot start: another quest is already active")
 *             else -> println("Error: ${error.message}")
 *         }
 *     },
 *     ifRight = { quest -> println("Started: ${quest.title}") }
 * )
 *
 * // Check daily energy budget
 * questRepository.getEnergyBudget(userId, LocalDate.now()).map { budget ->
 *     println("Energy: ${budget.spent}/${budget.budget} (${budget.remaining} remaining)")
 * }
 * ```
 */
interface QuestRepository {
    /**
     * Retrieve a quest by its unique identifier.
     *
     * @param id Unique identifier of the quest to retrieve
     * @return [AltairResult] containing the [Quest] if found, or [AltairError.NotFoundError.QuestNotFound]
     */
    suspend fun getById(id: Ulid): AltairResult<Quest>

    /**
     * Retrieve all non-deleted quests for a specific user.
     *
     * @param userId User identifier
     * @return [AltairResult] containing list of quests (empty list if none found)
     */
    suspend fun getAllForUser(userId: Ulid): AltairResult<List<Quest>>

    /**
     * Retrieve all quests with a specific status for a user.
     *
     * Useful for filtering backlog, active, completed, or abandoned quests.
     *
     * @param userId User identifier
     * @param status Status to filter by (BACKLOG, ACTIVE, COMPLETED, ABANDONED)
     * @return [AltairResult] containing filtered list of quests
     */
    suspend fun getByStatus(userId: Ulid, status: QuestStatus): AltairResult<List<Quest>>

    /**
     * Retrieve all quests associated with a specific epic.
     *
     * @param epicId Epic identifier
     * @return [AltairResult] containing list of quests in this epic
     */
    suspend fun getByEpic(epicId: Ulid): AltairResult<List<Quest>>

    /**
     * Retrieve the currently active quest for a user (WIP=1).
     *
     * Since only one quest can be active at a time, this returns a single
     * quest or null if no quest is currently active.
     *
     * @param userId User identifier
     * @return [AltairResult] containing the active [Quest] or null if none active
     */
    suspend fun getActiveQuest(userId: Ulid): AltairResult<Quest?>

    /**
     * Retrieve all quests scheduled for a specific date.
     *
     * This includes quests with explicit date associations and can be used
     * for daily planning views and energy budget calculations.
     *
     * @param userId User identifier
     * @param date Date to filter by
     * @return [AltairResult] containing list of quests for this date
     */
    suspend fun getTodayQuests(userId: Ulid, date: LocalDate): AltairResult<List<Quest>>

    /**
     * Create a new quest.
     *
     * The quest must have valid title (non-blank, ≤200 chars) and energy cost (1-5).
     *
     * @param quest Quest entity to create
     * @return [AltairResult] containing the created quest with timestamps set
     */
    suspend fun create(quest: Quest): AltairResult<Quest>

    /**
     * Update an existing quest.
     *
     * **Note:** For status changes, prefer dedicated transition methods
     * ([start], [complete], [abandon], [backlog]) to ensure proper constraint
     * enforcement and timestamp handling.
     *
     * @param quest Quest entity with updated fields
     * @return [AltairResult] containing the updated quest
     */
    suspend fun update(quest: Quest): AltairResult<Quest>

    /**
     * Start a quest (transition to ACTIVE status).
     *
     * **WIP=1 Enforcement:** This method checks if another quest is already
     * active for this user. If so, it returns [AltairError.ConflictError.WipLimitExceeded].
     *
     * On success:
     * - Status changes to ACTIVE
     * - `startedAt` timestamp is set
     * - `updatedAt` timestamp is updated
     *
     * @param id Quest identifier
     * @return [AltairResult] containing the updated quest, or [WipLimitExceeded] if limit reached
     */
    suspend fun start(id: Ulid): AltairResult<Quest>

    /**
     * Complete a quest (transition to COMPLETED status).
     *
     * On success:
     * - Status changes to COMPLETED
     * - `completedAt` timestamp is set
     * - `updatedAt` timestamp is updated
     * - Energy cost is added to daily energy budget's `spent` value
     *
     * @param id Quest identifier
     * @return [AltairResult] containing the completed quest
     */
    suspend fun complete(id: Ulid): AltairResult<Quest>

    /**
     * Abandon a quest (transition to ABANDONED status).
     *
     * Use this when a quest is no longer relevant or should not be pursued.
     * Abandoned quests don't count toward energy budget.
     *
     * On success:
     * - Status changes to ABANDONED
     * - `updatedAt` timestamp is updated
     *
     * @param id Quest identifier
     * @return [AltairResult] containing the abandoned quest
     */
    suspend fun abandon(id: Ulid): AltairResult<Quest>

    /**
     * Return a quest to backlog (transition to BACKLOG status).
     *
     * Use this to release an ACTIVE quest back to the backlog, freeing up
     * the WIP slot for another quest.
     *
     * On success:
     * - Status changes to BACKLOG
     * - `updatedAt` timestamp is updated
     * - `startedAt` remains set (preserves history)
     *
     * @param id Quest identifier
     * @return [AltairResult] containing the backlogged quest
     */
    suspend fun backlog(id: Ulid): AltairResult<Quest>

    /**
     * Retrieve all checkpoints for a quest, ordered by their `order` field.
     *
     * @param questId Quest identifier
     * @return [AltairResult] containing ordered list of checkpoints
     */
    suspend fun getCheckpoints(questId: Ulid): AltairResult<List<Checkpoint>>

    /**
     * Add a new checkpoint to a quest.
     *
     * @param checkpoint Checkpoint entity to create
     * @return [AltairResult] containing the created checkpoint
     */
    suspend fun addCheckpoint(checkpoint: Checkpoint): AltairResult<Checkpoint>

    /**
     * Update an existing checkpoint (e.g., mark as completed).
     *
     * @param checkpoint Checkpoint entity with updated fields
     * @return [AltairResult] containing the updated checkpoint
     */
    suspend fun updateCheckpoint(checkpoint: Checkpoint): AltairResult<Checkpoint>

    /**
     * Delete a checkpoint.
     *
     * Unlike quests, checkpoints are hard-deleted (no soft delete).
     *
     * @param id Checkpoint identifier
     * @return [AltairResult] containing Unit on success
     */
    suspend fun deleteCheckpoint(id: Ulid): AltairResult<Unit>

    /**
     * Reorder checkpoints for a quest.
     *
     * Updates the `order` field of checkpoints based on the provided sequence.
     * The list should contain all checkpoint IDs for the quest in desired order.
     *
     * @param questId Quest identifier
     * @param order Ordered list of checkpoint IDs (first = lowest order value)
     * @return [AltairResult] containing Unit on success
     */
    suspend fun reorderCheckpoints(questId: Ulid, order: List<Ulid>): AltairResult<Unit>

    /**
     * Retrieve energy budget for a specific date.
     *
     * If no budget exists for this date, returns a default budget (5 units).
     *
     * @param userId User identifier
     * @param date Date to retrieve budget for
     * @return [AltairResult] containing the energy budget for this date
     */
    suspend fun getEnergyBudget(userId: Ulid, date: LocalDate): AltairResult<EnergyBudget>

    /**
     * Set or update the daily energy budget for a specific date.
     *
     * Budget must be in range 1-10. The `spent` value is auto-calculated from
     * completed quests and should not be set directly via this method.
     *
     * @param userId User identifier
     * @param date Date to set budget for
     * @param budget Energy units available (1-10)
     * @return [AltairResult] containing the updated energy budget
     */
    suspend fun setDailyBudget(userId: Ulid, date: LocalDate, budget: Int): AltairResult<EnergyBudget>

    /**
     * Soft-delete a quest.
     *
     * Sets the `deletedAt` timestamp while preserving the quest data.
     * Deleted quests are excluded from standard query results but can be
     * restored via [restore].
     *
     * @param id Quest identifier
     * @return [AltairResult] containing Unit on success
     */
    suspend fun softDelete(id: Ulid): AltairResult<Unit>

    /**
     * Restore a soft-deleted quest.
     *
     * Clears the `deletedAt` timestamp, making the quest visible in
     * standard queries again.
     *
     * @param id Quest identifier
     * @return [AltairResult] containing Unit on success
     */
    suspend fun restore(id: Ulid): AltairResult<Unit>
}
