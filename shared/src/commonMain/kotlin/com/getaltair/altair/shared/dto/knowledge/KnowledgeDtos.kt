package com.getaltair.altair.shared.dto.knowledge

import kotlinx.serialization.Serializable

// === Request DTOs ===

/**
 * Request to create a new note.
 *
 * @property title The note's title (required)
 * @property content The note's markdown content (defaults to empty string)
 * @property folderId Optional parent folder ID
 * @property initiativeId Optional initiative to link this note to
 * @property tagIds List of tag IDs to apply to the note
 */
@Serializable
data class CreateNoteRequest(
    val title: String,
    val content: String = "",
    val folderId: String? = null,
    val initiativeId: String? = null,
    val tagIds: List<String> = emptyList()
)

/**
 * Request to update an existing note.
 *
 * All fields are optional; only provided fields will be updated.
 *
 * @property title New title for the note
 * @property content New markdown content
 * @property folderId New parent folder ID (null removes from folder)
 * @property initiativeId New initiative link (null removes link)
 */
@Serializable
data class UpdateNoteRequest(
    val title: String? = null,
    val content: String? = null,
    val folderId: String? = null,
    val initiativeId: String? = null
)

/**
 * Request to create a new folder.
 *
 * @property name The folder's display name
 * @property parentId Optional parent folder ID (null creates at root)
 */
@Serializable
data class CreateFolderRequest(
    val name: String,
    val parentId: String? = null
)

/**
 * Request to update an existing folder.
 *
 * All fields are optional; only provided fields will be updated.
 *
 * @property name New display name
 * @property parentId New parent folder ID (null moves to root)
 * @property order Display order within parent folder
 */
@Serializable
data class UpdateFolderRequest(
    val name: String? = null,
    val parentId: String? = null,
    val order: Int? = null
)

/**
 * Request to create a new tag.
 *
 * @property name The tag's display name
 * @property color Optional hex color code (e.g., "#3B82F6")
 */
@Serializable
data class CreateTagRequest(
    val name: String,
    val color: String? = null
)

/**
 * Request to update an existing tag.
 *
 * All fields are optional; only provided fields will be updated.
 *
 * @property name New display name
 * @property color New hex color code
 */
@Serializable
data class UpdateTagRequest(
    val name: String? = null,
    val color: String? = null
)

/**
 * Request to set all tags for a note.
 *
 * This replaces the note's current tags with the provided list.
 *
 * @property tagIds List of tag IDs to apply (empty list removes all tags)
 */
@Serializable
data class SetNoteTagsRequest(
    val tagIds: List<String>
)

// === Response DTOs ===

/**
 * Complete note details for viewing/editing.
 *
 * @property id Unique note identifier
 * @property title The note's title
 * @property content The note's markdown content
 * @property folderId Parent folder ID (null if at root)
 * @property folderPath Human-readable folder path (e.g., "Work/Projects")
 * @property initiativeId Linked initiative ID (null if not linked)
 * @property tags Tags applied to this note
 * @property attachments File attachments on this note
 * @property linkCount Number of outgoing links from this note
 * @property backlinkCount Number of incoming links to this note
 * @property createdAt ISO-8601 timestamp
 * @property updatedAt ISO-8601 timestamp
 */
@Serializable
data class NoteResponse(
    val id: String,
    val title: String,
    val content: String,
    val folderId: String?,
    val folderPath: String?,
    val initiativeId: String?,
    val tags: List<TagResponse>,
    val attachments: List<AttachmentResponse>,
    val linkCount: Int,
    val backlinkCount: Int,
    val createdAt: String,
    val updatedAt: String
)

/**
 * Abbreviated note for list views.
 *
 * @property id Unique note identifier
 * @property title The note's title
 * @property preview First ~200 characters of content
 * @property folderId Parent folder ID (null if at root)
 * @property tagCount Number of tags on this note
 * @property updatedAt ISO-8601 timestamp
 */
@Serializable
data class NoteSummaryResponse(
    val id: String,
    val title: String,
    val preview: String,
    val folderId: String?,
    val tagCount: Int,
    val updatedAt: String
)

/**
 * Folder details for hierarchy views.
 *
 * @property id Unique folder identifier
 * @property name The folder's display name
 * @property parentId Parent folder ID (null if at root)
 * @property path Full path from root (e.g., "Work/Projects/Active")
 * @property noteCount Number of direct child notes
 * @property childCount Number of direct child folders
 * @property children Nested child folders (optional, for tree views)
 */
@Serializable
data class FolderResponse(
    val id: String,
    val name: String,
    val parentId: String?,
    val path: String,
    val noteCount: Int,
    val childCount: Int,
    val children: List<FolderResponse>? = null
)

/**
 * Tag details with usage statistics.
 *
 * @property id Unique tag identifier
 * @property name The tag's display name
 * @property color Hex color code (e.g., "#3B82F6")
 * @property usageCount Number of notes using this tag
 */
@Serializable
data class TagResponse(
    val id: String,
    val name: String,
    val color: String?,
    val usageCount: Int
)

/**
 * File attachment metadata.
 *
 * @property id Unique attachment identifier
 * @property filename Original filename
 * @property mimeType File MIME type (e.g., "image/png")
 * @property sizeBytes File size in bytes
 * @property downloadUrl Presigned temporary download URL
 */
@Serializable
data class AttachmentResponse(
    val id: String,
    val filename: String,
    val mimeType: String,
    val sizeBytes: Long,
    val downloadUrl: String
)

/**
 * Note-to-note link with context.
 *
 * @property id Unique link identifier
 * @property sourceId Source note ID
 * @property sourceTitle Source note title
 * @property targetId Target note ID
 * @property targetTitle Target note title
 * @property context Surrounding text where link appears (optional)
 */
@Serializable
data class NoteLinkResponse(
    val id: String,
    val sourceId: String,
    val sourceTitle: String,
    val targetId: String,
    val targetTitle: String,
    val context: String?
)

/**
 * Knowledge graph representation.
 *
 * @property nodes All notes and source documents as graph nodes
 * @property edges All links between nodes
 */
@Serializable
data class GraphResponse(
    val nodes: List<GraphNode>,
    val edges: List<GraphEdge>
)

/**
 * Node in the knowledge graph.
 *
 * @property id Node identifier (matches note or document ID)
 * @property title Node display label
 * @property type Node type: "note" or "source_document"
 * @property linkCount Total incoming + outgoing link count
 */
@Serializable
data class GraphNode(
    val id: String,
    val title: String,
    val type: String,
    val linkCount: Int
)

/**
 * Edge in the knowledge graph.
 *
 * @property source Source node ID
 * @property target Target node ID
 */
@Serializable
data class GraphEdge(
    val source: String,
    val target: String
)

/**
 * Search results with pagination info.
 *
 * @property notes Matching notes (abbreviated)
 * @property totalCount Total matching notes (may exceed returned count)
 * @property query Original search query
 */
@Serializable
data class SearchResultResponse(
    val notes: List<NoteSummaryResponse>,
    val totalCount: Int,
    val query: String
)
