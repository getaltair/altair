package com.getaltair.altair.data.entity

/**
 * Test entity for verifying database connectivity and repository operations.
 *
 * Follows ADR-002 entity conventions:
 * - ID field: ULID string format
 * - Timestamps: ISO 8601 strings (createdAt, updatedAt)
 * - Soft delete: deletedAt field (null when active)
 * - Sync tracking: syncVersion integer field
 *
 * @property id Unique identifier in ULID format
 * @property name Name field for testing
 * @property value Integer value for testing
 * @property createdAt ISO 8601 timestamp when entity was created
 * @property updatedAt ISO 8601 timestamp when entity was last updated
 * @property deletedAt ISO 8601 timestamp when entity was soft-deleted (null if active)
 * @property syncVersion Version counter for sync tracking
 */
data class TestEntity(
    val id: String,
    val name: String,
    val value: Int,
    val createdAt: String,
    val updatedAt: String,
    val deletedAt: String? = null,
    val syncVersion: Int = 0
) {
    /**
     * Returns true if this entity has been soft-deleted.
     */
    val isDeleted: Boolean
        get() = deletedAt != null

    companion object {
        /**
         * SurrealDB table name for this entity.
         */
        const val TABLE_NAME = "test_entity"
    }
}
