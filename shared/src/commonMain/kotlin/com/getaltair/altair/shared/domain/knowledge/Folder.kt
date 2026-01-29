package com.getaltair.altair.shared.domain.knowledge

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A hierarchical folder for organizing notes.
 *
 * Folders provide tree-structured organization for notes in the Knowledge domain.
 * Each folder can contain:
 * - Multiple notes (via Note.folderId)
 * - Multiple child folders (via Folder.parentId)
 *
 * ## Hierarchy
 * Folders form a tree structure where:
 * - Root folders have [parentId] = null
 * - Child folders have [parentId] = parent folder's ID
 * - Circular references must be prevented at service layer
 *
 * ## Ordering
 * The [order] field allows custom sort order within a parent folder or at root level.
 * Lower values appear first. Folders at the same level can be reordered by updating their order values.
 *
 * ## Example Structure
 * ```
 * Projects (order=0, parentId=null)
 *   ├─ Work (order=0, parentId=Projects.id)
 *   └─ Personal (order=1, parentId=Projects.id)
 * Reference (order=1, parentId=null)
 * ```
 *
 * @property id Unique identifier for this folder
 * @property userId Owner of this folder
 * @property name Human-readable folder name (max 100 characters, required)
 * @property parentId Parent folder ID (null for root-level folders)
 * @property order Sort order within parent level (lower values appear first)
 * @property createdAt When this folder was created
 */
@Serializable
data class Folder(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val parentId: Ulid?,
    val order: Int,
    val createdAt: Instant
) {
    init {
        require(name.length <= MAX_NAME_LENGTH) {
            "Name max $MAX_NAME_LENGTH chars, got ${name.length}"
        }
        require(name.isNotBlank()) { "Name required" }
        require(order >= 0) { "Order must be non-negative, got $order" }
    }

    /**
     * Whether this is a root-level folder (no parent).
     */
    val isRoot: Boolean get() = parentId == null

    companion object {
        /** Maximum allowed folder name length in characters */
        const val MAX_NAME_LENGTH = 100
    }
}
