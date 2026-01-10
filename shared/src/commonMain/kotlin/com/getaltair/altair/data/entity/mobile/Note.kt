package com.getaltair.altair.data.entity.mobile

/**
 * Note entity for mobile platforms (Android/iOS).
 *
 * Represents a user note in the application.
 * Follows ADR-002 entity conventions with ULID identifiers and soft delete.
 *
 * @property id Unique identifier in ULID format
 * @property title Note title (required)
 * @property content Optional note content/body
 * @property folderId Optional parent folder ID for organization
 * @property createdAt ISO 8601 timestamp when note was created
 * @property updatedAt ISO 8601 timestamp when note was last updated
 * @property deletedAt ISO 8601 timestamp when note was soft-deleted (null if active)
 * @property syncVersion Version counter for sync tracking
 */
data class Note(
    val id: String,
    val title: String,
    val content: String? = null,
    val folderId: String? = null,
    val createdAt: String,
    val updatedAt: String,
    val deletedAt: String? = null,
    val syncVersion: Long = 0,
) {
    /**
     * Returns true if this note has been soft-deleted.
     */
    val isDeleted: Boolean
        get() = deletedAt != null

    /**
     * Returns true if this note has content.
     */
    val hasContent: Boolean
        get() = !content.isNullOrBlank()
}

/**
 * Extension function to convert SQLDelight-generated Note to domain Note entity.
 */
fun com.getaltair.altair.database.Note.toDomain(): Note = Note(
    id = id,
    title = title,
    content = content,
    folderId = folder_id,
    createdAt = created_at,
    updatedAt = updated_at,
    deletedAt = deleted_at,
    syncVersion = sync_version,
)
