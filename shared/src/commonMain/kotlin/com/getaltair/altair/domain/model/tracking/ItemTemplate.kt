package com.getaltair.altair.domain.model.tracking

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.Serializable
import kotlin.time.Instant

/**
 * A predefined schema for a category of Items.
 *
 * ItemTemplates define common field structures for similar items
 * (e.g., "Book" template with author, ISBN, genre fields). Items
 * created from a template inherit its FieldDefinitions.
 */
@Serializable
data class ItemTemplate(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val icon: String?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(name.isNotBlank()) { "ItemTemplate name must not be blank" }
        require(name.length <= 100) { "ItemTemplate name must be at most 100 characters" }
    }
}
