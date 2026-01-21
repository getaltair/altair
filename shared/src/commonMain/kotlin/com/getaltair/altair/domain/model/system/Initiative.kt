package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.common.ColorValidation
import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import kotlinx.serialization.Serializable
import kotlin.time.Instant

/**
 * A cross-cutting organizational unit that groups related work.
 *
 * Initiatives can be either projects (time-bound with completion) or areas
 * (ongoing responsibilities). They provide structure across all modules:
 * - Guidance: Epics and Quests can belong to an Initiative
 * - Knowledge: Notes can be tagged with an Initiative
 * - Tracking: Items can be associated with an Initiative
 */
@Serializable
data class Initiative(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val color: String?,
    val icon: String?,
    val status: InitiativeStatus,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(name.isNotBlank()) { "Initiative name must not be blank" }
        require(name.length <= 100) { "Initiative name must be at most 100 characters" }
        ColorValidation.requireValidHexColor(color, "Initiative color")
    }
}
