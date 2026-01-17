package com.getaltair.altair.dto.knowledge

import kotlinx.serialization.Serializable

/**
 * Request to create a new note.
 */
@Serializable
data class CreateNoteRequest(
    val title: String,
    val content: String = "",
    val folderId: String? = null,
    val initiativeId: String? = null,
    val isPinned: Boolean = false,
    val tagIds: List<String> = emptyList(),
)

/**
 * Request to update an existing note.
 */
@Serializable
data class UpdateNoteRequest(
    val title: String? = null,
    val content: String? = null,
    val folderId: String? = null,
    val initiativeId: String? = null,
    val isPinned: Boolean? = null,
)

/**
 * Response containing note data.
 */
@Serializable
data class NoteResponse(
    val id: String,
    val title: String,
    val content: String,
    val folderId: String?,
    val initiativeId: String?,
    val isPinned: Boolean,
    val createdAt: String,
    val updatedAt: String,
    val tagIds: List<String>,
    val forwardLinkCount: Int,
    val backlinkCount: Int,
)

/**
 * Request to create a new folder.
 */
@Serializable
data class CreateFolderRequest(
    val name: String,
    val parentId: String? = null,
)

/**
 * Request to update an existing folder.
 */
@Serializable
data class UpdateFolderRequest(
    val name: String? = null,
    val parentId: String? = null,
)

/**
 * Response containing folder data.
 */
@Serializable
data class FolderResponse(
    val id: String,
    val name: String,
    val parentId: String?,
    val sortOrder: Int,
    val createdAt: String,
    val updatedAt: String,
    val noteCount: Int,
    val childFolderCount: Int,
)

/**
 * Request to create a new tag.
 */
@Serializable
data class CreateTagRequest(
    val name: String,
    val color: String? = null,
)

/**
 * Request to update an existing tag.
 */
@Serializable
data class UpdateTagRequest(
    val name: String? = null,
    val color: String? = null,
)

/**
 * Response containing tag data.
 */
@Serializable
data class TagResponse(
    val id: String,
    val name: String,
    val color: String?,
    val createdAt: String,
    val updatedAt: String,
    val noteCount: Int,
)

/**
 * Request to add or remove tags from a note.
 */
@Serializable
data class UpdateNoteTagsRequest(
    val noteId: String,
    val addTagIds: List<String> = emptyList(),
    val removeTagIds: List<String> = emptyList(),
)

/**
 * Request to link two notes.
 */
@Serializable
data class LinkNotesRequest(
    val sourceNoteId: String,
    val targetNoteId: String,
    val context: String? = null,
)
