package com.getaltair.altair.domain.common

import kotlin.time.Instant

/**
 * Interface for entities that track creation and modification timestamps.
 *
 * All domain entities that need audit trails should implement this interface.
 * Timestamps are managed by the repository layer, not by domain logic.
 */
interface Timestamped {
    /** When this entity was first created */
    val createdAt: Instant

    /** When this entity was last modified */
    val updatedAt: Instant
}
