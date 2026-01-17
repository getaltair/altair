package com.getaltair.altair.domain.model.tracking

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.FieldType
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A field specification within an ItemTemplate.
 *
 * FieldDefinitions describe the custom attributes available for Items
 * of a particular template. They define the field name, type, and
 * constraints (like allowed values for enum fields).
 */
@Serializable
data class FieldDefinition(
    val id: Ulid,
    val userId: Ulid,
    val templateId: Ulid,
    val name: String,
    val fieldType: FieldType,
    val isRequired: Boolean,
    val defaultValue: String?,
    val enumOptions: List<String>?,
    val sortOrder: Int,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(name.isNotBlank()) { "FieldDefinition name must not be blank" }
        require(name.length <= 100) { "FieldDefinition name must be at most 100 characters" }
        require(sortOrder >= 0) { "Sort order must be non-negative" }
        if (fieldType == FieldType.ENUM) {
            require(!enumOptions.isNullOrEmpty()) {
                "Enum field must have at least one option"
            }
        } else {
            require(enumOptions.isNullOrEmpty()) {
                "enumOptions should only be provided for ENUM fields"
            }
        }
    }
}
