package com.getaltair.altair.shared.domain.tracking

import com.getaltair.altair.shared.domain.common.FieldType
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.serialization.Serializable

/**
 * Defines a custom field structure within an ItemTemplate.
 *
 * FieldDefinitions specify the schema for CustomFields that can be created
 * when instantiating items from a template.
 *
 * Features:
 * - Type validation (TEXT, NUMBER, DATE, BOOLEAN, URL, ENUM)
 * - Required field enforcement
 * - Default values for convenience
 * - Enum options for predefined choices
 * - Ordering for consistent UI presentation
 *
 * Example: "Book" template might define:
 * 1. ISBN (TEXT, required, order=1)
 * 2. Author (TEXT, required, order=2)
 * 3. Publisher (TEXT, optional, order=3)
 * 4. Year (NUMBER, optional, order=4)
 * 5. Format (ENUM, optional, enumOptions=["Hardcover", "Paperback", "eBook"], order=5)
 *
 * Invariants:
 * - Name is required and max 100 characters
 * - Order must be non-negative
 * - ENUM type must have enumOptions
 * - Non-ENUM types must have null enumOptions
 *
 * @property id Unique identifier for this field definition
 * @property templateId The ItemTemplate this definition belongs to
 * @property name Human-readable field name (required, max 100 chars)
 * @property fieldType Data type for values created from this definition
 * @property required Whether a value is required when creating items
 * @property defaultValue Optional default value as string
 * @property enumOptions Required list of options for ENUM type (null for other types)
 * @property order Display order (0-based) for consistent UI presentation
 */
@Serializable
data class FieldDefinition(
    val id: Ulid,
    val templateId: Ulid,
    val name: String,
    val fieldType: FieldType,
    val required: Boolean,
    val defaultValue: String?,
    val enumOptions: List<String>?,
    val order: Int
) {
    init {
        require(name.length <= 100) { "Name max 100 chars" }
        require(name.isNotBlank()) { "Name required" }
        require(order >= 0) { "Order must be non-negative" }
        if (fieldType == FieldType.ENUM) {
            require(!enumOptions.isNullOrEmpty()) { "Enum fields must have options" }
        }
    }
}
