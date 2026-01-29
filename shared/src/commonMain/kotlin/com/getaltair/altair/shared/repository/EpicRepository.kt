package com.getaltair.altair.shared.repository

import com.getaltair.altair.shared.domain.common.EpicStatus
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Epic

/**
 * Repository interface for Epic operations.
 *
 * This repository provides CRUD operations for Epics, which serve as organizational
 * containers for related Quests within the Guidance module. All operations return
 * [AltairResult] for consistent error handling.
 *
 * ## Purpose
 *
 * Epics group thematically related Quests (e.g., "Implement user authentication",
 * "Learn Kotlin Multiplatform"). They provide hierarchical organization and can
 * optionally be linked to Initiatives for cross-module project/area alignment.
 *
 * ## Lifecycle States
 *
 * Epics progress through three states:
 *
 * - **ACTIVE** - Accepting new quests, work in progress
 * - **COMPLETED** - All associated quests finished, no further work expected
 * - **ARCHIVED** - Preserved for reference, no longer active
 *
 * ## Soft Delete
 *
 * Epics are soft-deleted via [softDelete], setting the `deletedAt` timestamp
 * while preserving data for audit purposes. Deleted epics are excluded from
 * standard query results.
 *
 * ## Initiative Association
 *
 * Epics can optionally link to an Initiative (cross-module organization system).
 * Use [getByInitiative] to retrieve all epics associated with a specific initiative.
 *
 * ## Error Handling
 *
 * All methods return [AltairResult] with typed errors:
 *
 * - [AltairError.NotFoundError.EpicNotFound] - Epic ID doesn't exist
 * - [AltairError.ValidationError] - Invalid input data
 * - [AltairError.StorageError] - Database operation failed
 *
 * ## Example Usage
 *
 * ```kotlin
 * // Create an epic for a project
 * val epic = Epic(
 *     id = Ulid.generate(),
 *     userId = currentUserId,
 *     title = "Build REST API",
 *     description = "Implement backend services for mobile app",
 *     status = EpicStatus.ACTIVE,
 *     initiativeId = projectId,
 *     createdAt = Clock.System.now(),
 *     updatedAt = Clock.System.now(),
 *     completedAt = null,
 *     deletedAt = null
 * )
 *
 * epicRepository.create(epic).fold(
 *     ifLeft = { error -> println("Error: ${error.message}") },
 *     ifRight = { created -> println("Created epic: ${created.title}") }
 * )
 *
 * // Complete an epic when all quests are done
 * epicRepository.complete(epicId).map { completedEpic ->
 *     println("Epic completed: ${completedEpic.title}")
 * }
 * ```
 */
interface EpicRepository {
    /**
     * Retrieve an epic by its unique identifier.
     *
     * @param id Unique identifier of the epic to retrieve
     * @return [AltairResult] containing the [Epic] if found, or [AltairError.NotFoundError.EpicNotFound]
     */
    suspend fun getById(id: Ulid): AltairResult<Epic>

    /**
     * Retrieve all non-deleted epics for a specific user.
     *
     * @param userId User identifier
     * @return [AltairResult] containing list of epics (empty list if none found)
     */
    suspend fun getAllForUser(userId: Ulid): AltairResult<List<Epic>>

    /**
     * Retrieve all epics associated with a specific initiative.
     *
     * Use this to find all epics under a project or area of responsibility.
     *
     * @param initiativeId Initiative identifier
     * @return [AltairResult] containing list of epics linked to this initiative
     */
    suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Epic>>

    /**
     * Retrieve all epics with a specific status for a user.
     *
     * Useful for filtering active, completed, or archived epics.
     *
     * @param userId User identifier
     * @param status Status to filter by (ACTIVE, COMPLETED, ARCHIVED)
     * @return [AltairResult] containing filtered list of epics
     */
    suspend fun getByStatus(userId: Ulid, status: EpicStatus): AltairResult<List<Epic>>

    /**
     * Create a new epic.
     *
     * The epic must have a valid title (non-blank, ≤200 chars).
     *
     * @param epic Epic entity to create
     * @return [AltairResult] containing the created epic with timestamps set
     */
    suspend fun create(epic: Epic): AltairResult<Epic>

    /**
     * Update an existing epic.
     *
     * **Note:** For status changes to COMPLETED, prefer [complete] method to
     * ensure proper timestamp handling. For ARCHIVED status, use [archive].
     *
     * @param epic Epic entity with updated fields
     * @return [AltairResult] containing the updated epic
     */
    suspend fun update(epic: Epic): AltairResult<Epic>

    /**
     * Complete an epic (transition to COMPLETED status).
     *
     * Use this when all associated quests are finished and no further work
     * is expected on this epic.
     *
     * On success:
     * - Status changes to COMPLETED
     * - `completedAt` timestamp is set
     * - `updatedAt` timestamp is updated
     *
     * @param id Epic identifier
     * @return [AltairResult] containing the completed epic
     */
    suspend fun complete(id: Ulid): AltairResult<Epic>

    /**
     * Archive an epic (transition to ARCHIVED status).
     *
     * Use this to preserve an epic for reference while removing it from
     * active views. Archived epics are typically no longer worked on but
     * may not be fully "complete" in the traditional sense.
     *
     * On success:
     * - Status changes to ARCHIVED
     * - `updatedAt` timestamp is updated
     *
     * @param id Epic identifier
     * @return [AltairResult] containing the archived epic
     */
    suspend fun archive(id: Ulid): AltairResult<Epic>

    /**
     * Soft-delete an epic.
     *
     * Sets the `deletedAt` timestamp while preserving the epic data.
     * Deleted epics are excluded from standard query results.
     *
     * **Note:** Soft-deleting an epic does NOT cascade to its quests.
     * Quests remain accessible and can be reassigned to other epics or
     * left without an epic.
     *
     * @param id Epic identifier
     * @return [AltairResult] containing Unit on success
     */
    suspend fun softDelete(id: Ulid): AltairResult<Unit>
}
