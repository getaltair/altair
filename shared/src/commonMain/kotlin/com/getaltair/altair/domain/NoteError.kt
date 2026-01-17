package com.getaltair.altair.domain

import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Errors specific to Note operations in the Knowledge module.
 *
 * These errors extend [DomainError] to enable exhaustive when-matching
 * for Note-specific error handling while maintaining compatibility
 * with generic error handlers.
 */
@Serializable
sealed interface NoteError : DomainError {
    /**
     * The requested note could not be found.
     *
     * @property id The ULID of the note that was not found
     */
    @Serializable
    @SerialName("note_not_found")
    data class NotFound(
        val id: Ulid,
    ) : NoteError {
        override fun toUserMessage(): String = "The requested note could not be found."
    }

    /**
     * A note with the given title already exists in the same folder.
     *
     * @property title The conflicting title (must not be blank)
     * @property folderId The folder where the conflict exists (null for root)
     */
    @Serializable
    @SerialName("note_title_conflict")
    data class TitleConflict(
        val title: String,
        val folderId: Ulid?,
    ) : NoteError {
        init {
            require(title.isNotBlank()) { "Title must not be blank" }
        }

        override fun toUserMessage(): String = "A note with the title \"$title\" already exists in this folder."
    }

    /**
     * A wiki-style link in the note content references an invalid target.
     *
     * @property linkText The wiki link text that could not be resolved (must not be blank)
     * @property noteId The note containing the invalid link
     */
    @Serializable
    @SerialName("note_invalid_wiki_link")
    data class InvalidWikiLink(
        val linkText: String,
        val noteId: Ulid,
    ) : NoteError {
        init {
            require(linkText.isNotBlank()) { "Link text must not be blank" }
        }

        override fun toUserMessage(): String = "The link \"$linkText\" could not be resolved to an existing note."
    }

    /**
     * Creating the link would introduce a circular reference chain.
     *
     * @property sourceNoteId The note where the link would originate
     * @property targetNoteId The note being linked to (must differ from sourceNoteId)
     */
    @Serializable
    @SerialName("note_circular_link")
    data class CircularLink(
        val sourceNoteId: Ulid,
        val targetNoteId: Ulid,
    ) : NoteError {
        init {
            require(sourceNoteId != targetNoteId) { "Source and target note IDs must differ" }
        }

        override fun toUserMessage(): String = "Cannot create this link because it would create a circular reference."
    }

    /**
     * The folder specified for the note does not exist.
     *
     * @property folderId The ULID of the folder that was not found
     */
    @Serializable
    @SerialName("note_folder_not_found")
    data class FolderNotFound(
        val folderId: Ulid,
    ) : NoteError {
        override fun toUserMessage(): String = "The specified folder could not be found."
    }

    /**
     * The link between two notes was not found.
     *
     * @property sourceNoteId The source note of the link
     * @property targetNoteId The target note of the link
     */
    @Serializable
    @SerialName("note_link_not_found")
    data class LinkNotFound(
        val sourceNoteId: Ulid,
        val targetNoteId: Ulid,
    ) : NoteError {
        override fun toUserMessage(): String = "The link between these notes could not be found."
    }
}
