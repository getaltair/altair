package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.knowledge.Tag
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Tag entities.
 *
 * Tags provide flat-namespace labels for categorizing Notes.
 * A Note can have multiple Tags, enabling cross-cutting categorization.
 */
interface TagRepository : Repository<Tag, DomainError> {
    /**
     * Finds a tag by its name.
     *
     * Tag names are unique per user.
     *
     * @param name The tag name to search for
     * @return Either an error if not found, or the tag
     */
    suspend fun findByName(name: String): Either<DomainError, Tag>

    /**
     * Finds or creates a tag with the given name.
     *
     * If a tag with the name exists, returns it. Otherwise, creates a new one.
     *
     * @param name The tag name
     * @param color Optional color for new tags
     * @return Either an error on failure, or the found/created tag
     */
    suspend fun findOrCreate(
        name: String,
        color: String? = null,
    ): Either<DomainError, Tag>

    /**
     * Finds all tags applied to a specific note.
     *
     * @param noteId The ULID of the note
     * @return A Flow emitting tags on the note
     */
    fun findByNote(noteId: Ulid): Flow<List<Tag>>

    /**
     * Applies a tag to a note.
     *
     * If the note already has this tag, this is a no-op.
     *
     * @param noteId The ULID of the note
     * @param tagId The ULID of the tag
     * @return Either an error on failure, or Unit on success
     */
    suspend fun tagNote(
        noteId: Ulid,
        tagId: Ulid,
    ): Either<DomainError, Unit>

    /**
     * Removes a tag from a note.
     *
     * @param noteId The ULID of the note
     * @param tagId The ULID of the tag
     * @return Either an error on failure, or Unit on success
     */
    suspend fun untagNote(
        noteId: Ulid,
        tagId: Ulid,
    ): Either<DomainError, Unit>

    /**
     * Returns tags with their usage counts, sorted by popularity.
     *
     * @param limit Maximum number of tags to return
     * @return A Flow emitting tags with their note counts
     */
    fun findMostUsed(limit: Int = 20): Flow<List<Pair<Tag, Int>>>
}
