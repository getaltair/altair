package com.getaltair.altair.shared.domain.tracking

import com.getaltair.altair.shared.domain.common.FieldType
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.serialization.Serializable

/**
 * Represents a custom metadata field attached to an Item.
 *
 * CustomFields allow flexible item metadata:
 * - Can be created from a FieldDefinition (template-based)
 * - Can be ad-hoc fields (definitionId = null)
 * - Value stored as string, interpreted by fieldType
 *
 * Field Type Examples:
 * - TEXT: "Hardcover", "Red", "Like new"
 * - NUMBER: "24", "3.14"
 * - DATE: ISO-8601 format "2025-12-31"
 * - BOOLEAN: "true", "false"
 * - URL: "https://manual.example.com"
 * - ENUM: "Small" (from predefined options)
 *
 * Template vs Ad-hoc:
 * - Template-based: definitionId references FieldDefinition in ItemTemplate
 * - Ad-hoc: definitionId = null, user creates field on the fly
 *
 * Invariants:
 * - Name is required and max 100 characters
 *
 * @property id Unique identifier for this custom field instance
 * @property itemId The item this field is attached to
 * @property name Human-readable field name (required, max 100 chars)
 * @property fieldType Data type for value interpretation
 * @property value The field value as string (null = empty/not set)
 * @property definitionId Optional reference to FieldDefinition (null = ad-hoc)
 */
@Serializable
data class CustomField(
    val id: Ulid,
    val itemId: Ulid,
    val name: String,
    val fieldType: FieldType,
    val value: String?,
    val definitionId: Ulid?
) {
    init {
        require(name.length <= 100) { "Name max 100 chars" }
        require(name.isNotBlank()) { "Name required" }
    }

    /**
     * Returns true if this field was created from a template FieldDefinition.
     */
    val isFromTemplate: Boolean get() = definitionId != null
}
