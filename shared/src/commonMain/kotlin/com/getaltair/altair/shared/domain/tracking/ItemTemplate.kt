package com.getaltair.altair.shared.domain.tracking

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a template for creating consistent items with predefined structure.
 *
 * ItemTemplates enable:
 * - Consistent item creation (e.g., "Book", "Tool", "Medication")
 * - Predefined custom fields via FieldDefinitions
 * - Visual identity (icon for UI representation)
 * - Bulk categorization and filtering
 *
 * Example:
 * - "Book" template with fields: ISBN, Author, Publisher, Year
 * - "Medication" template with fields: Dosage, Expiration, Prescription Number
 * - "Power Tool" template with fields: Model, Voltage, Warranty Expiration
 *
 * When creating an Item from a template, the user fills in values for the
 * template's FieldDefinitions, creating CustomField instances.
 *
 * Invariants:
 * - Name is required and max 100 characters
 *
 * @property id Unique identifier for this template
 * @property userId Owner of this template
 * @property name Human-readable template name (required, max 100 chars)
 * @property description Optional detailed description
 * @property icon Optional icon identifier for UI representation
 * @property createdAt When this template was created
 */
@Serializable
data class ItemTemplate(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val icon: String?,
    val createdAt: Instant
) {
    init {
        require(name.length <= 100) { "Name max 100 chars" }
        require(name.isNotBlank()) { "Name required" }
    }
}
