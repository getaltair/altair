package com.getaltair.altair.domain.common

import kotlin.time.Instant

/**
 * Interface for entities that support soft deletion.
 *
 * Soft delete means the record remains in the database but is marked as deleted.
 * This allows for:
 * - Recovery of accidentally deleted data
 * - Audit trails and compliance requirements
 * - Referential integrity with dependent records
 *
 * Repository implementations should filter out soft-deleted records by default.
 */
interface SoftDeletable {
    /** When this entity was soft-deleted, or null if not deleted */
    val deletedAt: Instant?

    /** Returns true if this entity has been soft-deleted */
    val isDeleted: Boolean get() = deletedAt != null
}
