package com.getaltair.altair.data.entity.mobile

/**
 * Quest entity for mobile platforms (Android/iOS).
 *
 * Represents a quest or task in the application.
 * Follows ADR-002 entity conventions with ULID identifiers and soft delete.
 *
 * @property id Unique identifier in ULID format
 * @property title Quest title (required)
 * @property description Optional quest description
 * @property status Quest status (pending, in_progress, completed, etc.)
 * @property epicId Optional parent epic ID for grouping quests
 * @property createdAt ISO 8601 timestamp when quest was created
 * @property updatedAt ISO 8601 timestamp when quest was last updated
 * @property deletedAt ISO 8601 timestamp when quest was soft-deleted (null if active)
 * @property syncVersion Version counter for sync tracking
 */
data class Quest(
    val id: String,
    val title: String,
    val description: String? = null,
    val status: String = STATUS_PENDING,
    val epicId: String? = null,
    val createdAt: String,
    val updatedAt: String,
    val deletedAt: String? = null,
    val syncVersion: Long = 0,
) {
    /**
     * Returns true if this quest has been soft-deleted.
     */
    val isDeleted: Boolean
        get() = deletedAt != null

    /**
     * Returns true if this quest is completed.
     */
    val isCompleted: Boolean
        get() = status == STATUS_COMPLETED

    companion object {
        const val STATUS_PENDING = "pending"
        const val STATUS_IN_PROGRESS = "in_progress"
        const val STATUS_COMPLETED = "completed"
        const val STATUS_CANCELLED = "cancelled"
    }
}

/**
 * Extension function to convert SQLDelight-generated Quest to domain Quest entity.
 */
fun com.getaltair.altair.database.Quest.toDomain(): Quest = Quest(
    id = id,
    title = title,
    description = description,
    status = status,
    epicId = epic_id,
    createdAt = created_at,
    updatedAt = updated_at,
    deletedAt = deleted_at,
    syncVersion = sync_version,
)
