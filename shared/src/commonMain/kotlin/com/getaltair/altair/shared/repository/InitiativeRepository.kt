package com.getaltair.altair.shared.repository

import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.Initiative

/**
 * Repository interface for managing Initiatives (cross-module organizational structures).
 *
 * Initiatives represent Projects and Areas that span across Guidance, Knowledge, and Tracking.
 * They provide the organizational structure for tasks, notes, and items, with hierarchical
 * relationships and focus management for context-driven work.
 */
interface InitiativeRepository {
    /**
     * Retrieves an Initiative by its unique identifier.
     *
     * @param id The unique identifier of the Initiative
     * @return Success with the Initiative, or Failure if not found
     */
    suspend fun getById(id: Ulid): AltairResult<Initiative>

    /**
     * Retrieves all Initiatives for a specific user, including archived ones.
     *
     * @param userId The unique identifier of the user
     * @return Success with list of all Initiatives (may be empty), or Failure on error
     */
    suspend fun getAllForUser(userId: Ulid): AltairResult<List<Initiative>>

    /**
     * Retrieves all child Initiatives of a parent Initiative.
     *
     * @param parentId The unique identifier of the parent Initiative
     * @return Success with list of child Initiatives (may be empty), or Failure on error
     */
    suspend fun getChildren(parentId: Ulid): AltairResult<List<Initiative>>

    /**
     * Retrieves the currently focused Initiative for a user.
     * Only one Initiative can be focused at a time per user.
     *
     * @param userId The unique identifier of the user
     * @return Success with the focused Initiative (null if none), or Failure on error
     */
    suspend fun getFocused(userId: Ulid): AltairResult<Initiative?>

    /**
     * Creates a new Initiative.
     *
     * @param initiative The Initiative to create (ID should be generated)
     * @return Success with the created Initiative (with populated metadata), or Failure on error
     */
    suspend fun create(initiative: Initiative): AltairResult<Initiative>

    /**
     * Updates an existing Initiative.
     * Cannot change ID, userId, or timestamps managed by the system.
     *
     * @param initiative The Initiative with updated fields
     * @return Success with the updated Initiative, or Failure if not found or validation fails
     */
    suspend fun update(initiative: Initiative): AltairResult<Initiative>

    /**
     * Sets the focused Initiative for a user.
     * Unfocuses any previously focused Initiative automatically.
     *
     * @param userId The unique identifier of the user
     * @param initiativeId The Initiative to focus (null to clear focus)
     * @return Success on focus update, or Failure if Initiative doesn't exist or belongs to different user
     */
    suspend fun setFocused(userId: Ulid, initiativeId: Ulid?): AltairResult<Unit>

    /**
     * Soft deletes an Initiative by setting deletedAt timestamp.
     * Archived Initiatives are hidden from default views but remain queryable.
     *
     * @param id The unique identifier of the Initiative to archive
     * @return Success on deletion, or Failure if not found
     */
    suspend fun softDelete(id: Ulid): AltairResult<Unit>

    /**
     * Restores a soft-deleted Initiative by clearing deletedAt timestamp.
     *
     * @param id The unique identifier of the Initiative to restore
     * @return Success on restoration, or Failure if not found
     */
    suspend fun restore(id: Ulid): AltairResult<Unit>
}
