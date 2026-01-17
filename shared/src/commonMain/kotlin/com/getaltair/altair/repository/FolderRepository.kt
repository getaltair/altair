package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.knowledge.Folder
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Folder entities.
 *
 * Folders provide hierarchical organization for Notes. They can be
 * nested to create folder trees.
 */
interface FolderRepository : Repository<Folder, DomainError> {
    /**
     * Finds all root-level folders (folders without a parent).
     *
     * @return A Flow emitting root folders ordered by sortOrder
     */
    fun findRoots(): Flow<List<Folder>>

    /**
     * Finds all child folders of a specific parent.
     *
     * @param parentId The ULID of the parent folder
     * @return A Flow emitting child folders ordered by sortOrder
     */
    fun findByParent(parentId: Ulid): Flow<List<Folder>>

    /**
     * Returns the full folder hierarchy as a tree structure.
     *
     * @return A Flow emitting the folder tree (roots with nested children)
     */
    fun findTree(): Flow<List<FolderNode>>

    /**
     * Gets the path from root to a specific folder.
     *
     * @param id The ULID of the folder
     * @return Either an error on failure, or the path (list of folders from root)
     */
    suspend fun getPath(id: Ulid): Either<DomainError, List<Folder>>

    /**
     * Moves a folder to a new parent.
     *
     * @param id The ULID of the folder to move
     * @param newParentId The ULID of the new parent (null for root)
     * @return Either an error (e.g., circular reference), or the updated folder
     */
    suspend fun move(
        id: Ulid,
        newParentId: Ulid?,
    ): Either<DomainError, Folder>

    /**
     * Reorders folders within a parent.
     *
     * @param parentId The parent folder ID (null for root level)
     * @param orderedIds The folder IDs in the desired order
     * @return Either an error on failure, or Unit on success
     */
    suspend fun reorder(
        parentId: Ulid?,
        orderedIds: List<Ulid>,
    ): Either<DomainError, Unit>
}

/**
 * A folder with its nested children for tree representation.
 */
data class FolderNode(
    val folder: Folder,
    val children: List<FolderNode>,
)
