package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.InitiativeStatus
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlinx.serialization.Serializable

/**
 * Cross-module organizational container for Projects and Areas.
 *
 * Initiatives provide hierarchical organization for all domain entities across
 * the three Altair modules (Guidance, Knowledge, Tracking).
 *
 * ## Project vs. Area (PARA Method)
 * - **Project** ([ongoing]=false): Finite goal with completion criteria and [targetDate]
 * - **Area** ([ongoing]=true): Ongoing responsibility with no completion state
 *
 * ## Hierarchy
 * - Initiatives can have parent initiatives ([parentId])
 * - Both projects and areas can contain child initiatives
 * - Maximum depth is not enforced at the domain layer
 *
 * ## Focus System
 * - Only ONE initiative per user can be [focused] = true
 * - Focused initiative gets prioritized in UI and workflows
 * - Enforcement happens at the service layer
 *
 * ## Lifecycle
 * - Projects can reach [InitiativeStatus.COMPLETED]
 * - Areas cannot be completed (enforced in init)
 * - Soft deletion via [deletedAt]
 *
 * @property id Unique identifier for this initiative
 * @property userId Owner of this initiative
 * @property name Display name (max 200 characters)
 * @property description Optional markdown description
 * @property parentId Parent initiative for hierarchical organization (null = top-level)
 * @property ongoing True for Areas (no completion), False for Projects (completable)
 * @property targetDate Target completion date (ignored if ongoing=true)
 * @property status Current lifecycle status
 * @property focused Whether this is the user's currently focused initiative (one per user)
 * @property createdAt When this initiative was created
 * @property updatedAt When this initiative was last modified
 * @property deletedAt When this initiative was soft-deleted (null if active)
 */
@Serializable
data class Initiative(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val parentId: Ulid?,
    val ongoing: Boolean,
    val targetDate: LocalDate?,
    val status: InitiativeStatus,
    val focused: Boolean,
    val createdAt: Instant,
    val updatedAt: Instant,
    val deletedAt: Instant?
) {
    init {
        require(name.length <= 200) { "Name max 200 chars" }
        require(name.isNotBlank()) { "Name required" }
        if (status == InitiativeStatus.COMPLETED) {
            require(!ongoing) { "Areas cannot be completed, only projects" }
        }
    }

    /**
     * Whether this is a Project (finite, completable).
     */
    val isProject: Boolean get() = !ongoing

    /**
     * Whether this is an Area (ongoing, not completable).
     */
    val isArea: Boolean get() = ongoing

    /**
     * Whether this initiative has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null

    /**
     * Whether this is a top-level initiative (no parent).
     */
    val isTopLevel: Boolean get() = parentId == null
}
