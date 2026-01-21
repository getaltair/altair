package com.getaltair.altair.domain.model.tracking

import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlin.time.Instant
import kotlinx.serialization.Serializable

/**
 * A user-defined attribute value on an Item.
 *
 * CustomFields store the actual values for fields defined by
 * FieldDefinitions. The value is stored as a string and parsed
 * according to the field type.
 */
@Serializable
data class CustomField(
    val id: Ulid,
    val userId: Ulid,
    val itemId: Ulid,
    val fieldDefinitionId: Ulid,
    val value: String,
    override val createdAt: Instant,
    override val updatedAt: Instant,
) : Timestamped {
    init {
        require(value.length <= 5000) { "Custom field value must be at most 5000 characters" }
    }
}
