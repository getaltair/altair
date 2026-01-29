package com.getaltair.altair.shared.dto.tracking

import com.getaltair.altair.shared.domain.common.FieldType
import kotlinx.serialization.Serializable

// === Request DTOs ===

/**
 * Request to create a new inventory item.
 *
 * @property name Display name of the item
 * @property description Optional detailed description
 * @property quantity Initial quantity (defaults to 1)
 * @property templateId Optional template to inherit field definitions from
 * @property locationId Physical location where item is stored
 * @property containerId Container holding the item
 * @property initiativeId Initiative this item relates to
 * @property customFields Additional metadata fields
 */
@Serializable
data class CreateItemRequest(
    val name: String,
    val description: String? = null,
    val quantity: Int = 1,
    val templateId: String? = null,
    val locationId: String? = null,
    val containerId: String? = null,
    val initiativeId: String? = null,
    val customFields: List<CustomFieldInput> = emptyList()
)

/**
 * Input for a custom field value during item creation.
 *
 * @property name Field name
 * @property fieldType Type of data stored in field
 * @property value Serialized field value
 */
@Serializable
data class CustomFieldInput(
    val name: String,
    val fieldType: FieldType,
    val value: String?
)

/**
 * Request to update an existing item's properties.
 * All fields are optional - only provided fields will be updated.
 *
 * @property name New display name
 * @property description New description
 * @property quantity New quantity value
 * @property locationId New location ID
 * @property containerId New container ID
 */
@Serializable
data class UpdateItemRequest(
    val name: String? = null,
    val description: String? = null,
    val quantity: Int? = null,
    val locationId: String? = null,
    val containerId: String? = null
)

/**
 * Request to move an item to a different location or container.
 *
 * @property locationId Target location ID (null to unset)
 * @property containerId Target container ID (null to unset)
 */
@Serializable
data class MoveItemRequest(
    val locationId: String? = null,
    val containerId: String? = null
)

/**
 * Request to create a new physical location.
 *
 * @property name Display name of the location
 * @property description Optional description
 * @property parentId Parent location for hierarchical organization
 */
@Serializable
data class CreateLocationRequest(
    val name: String,
    val description: String? = null,
    val parentId: String? = null
)

/**
 * Request to update an existing location's properties.
 * All fields are optional - only provided fields will be updated.
 *
 * @property name New display name
 * @property description New description
 * @property parentId New parent location ID
 */
@Serializable
data class UpdateLocationRequest(
    val name: String? = null,
    val description: String? = null,
    val parentId: String? = null
)

/**
 * Request to create a new container.
 *
 * @property name Display name of the container
 * @property description Optional description
 * @property locationId Location where container resides
 * @property parentId Parent container for nested organization
 */
@Serializable
data class CreateContainerRequest(
    val name: String,
    val description: String? = null,
    val locationId: String? = null,
    val parentId: String? = null
)

/**
 * Request to update an existing container's properties.
 * All fields are optional - only provided fields will be updated.
 *
 * @property name New display name
 * @property description New description
 * @property locationId New location ID
 */
@Serializable
data class UpdateContainerRequest(
    val name: String? = null,
    val description: String? = null,
    val locationId: String? = null
)

/**
 * Request to create a new item template.
 *
 * @property name Display name of the template
 * @property description Optional description
 * @property icon Optional icon identifier
 * @property fields Field definitions for items using this template
 */
@Serializable
data class CreateTemplateRequest(
    val name: String,
    val description: String? = null,
    val icon: String? = null,
    val fields: List<FieldDefinitionInput>
)

/**
 * Input for defining a custom field in a template.
 *
 * @property name Field name
 * @property fieldType Type of data stored in field
 * @property required Whether field must be populated
 * @property defaultValue Default value when not specified
 * @property enumOptions Valid options for ENUM type fields
 */
@Serializable
data class FieldDefinitionInput(
    val name: String,
    val fieldType: FieldType,
    val required: Boolean = false,
    val defaultValue: String? = null,
    val enumOptions: List<String>? = null
)

// === Response DTOs ===

/**
 * Complete representation of an inventory item.
 *
 * @property id Unique item identifier
 * @property name Display name
 * @property description Optional description
 * @property quantity Current quantity
 * @property templateId Template ID if derived from template
 * @property templateName Display name of template
 * @property locationId Location ID where stored
 * @property locationPath Full hierarchical path to location
 * @property containerId Container ID if in a container
 * @property containerPath Full hierarchical path to container
 * @property initiativeId Related initiative ID
 * @property imageUrl URL to item image
 * @property customFields Custom metadata fields
 * @property createdAt ISO-8601 creation timestamp
 * @property updatedAt ISO-8601 last update timestamp
 */
@Serializable
data class ItemResponse(
    val id: String,
    val name: String,
    val description: String?,
    val quantity: Int,
    val templateId: String?,
    val templateName: String?,
    val locationId: String?,
    val locationPath: String?,
    val containerId: String?,
    val containerPath: String?,
    val initiativeId: String?,
    val imageUrl: String?,
    val customFields: List<CustomFieldResponse>,
    val createdAt: String,
    val updatedAt: String
)

/**
 * Lightweight item representation for list views.
 *
 * @property id Unique item identifier
 * @property name Display name
 * @property quantity Current quantity
 * @property locationPath Full hierarchical path to location
 * @property containerPath Full hierarchical path to container
 * @property imageUrl URL to item image
 */
@Serializable
data class ItemSummaryResponse(
    val id: String,
    val name: String,
    val quantity: Int,
    val locationPath: String?,
    val containerPath: String?,
    val imageUrl: String?
)

/**
 * Custom field value with metadata.
 *
 * @property id Unique field identifier
 * @property name Field name
 * @property fieldType Type of data stored
 * @property value Serialized field value
 * @property required Whether field must be populated
 */
@Serializable
data class CustomFieldResponse(
    val id: String,
    val name: String,
    val fieldType: FieldType,
    val value: String?,
    val required: Boolean
)

/**
 * Physical location representation with statistics.
 *
 * @property id Unique location identifier
 * @property name Display name
 * @property description Optional description
 * @property parentId Parent location ID if nested
 * @property path Full hierarchical path (e.g., "Home > Garage > Shelf")
 * @property itemCount Number of items at this location
 * @property childCount Number of child locations
 * @property children Nested child locations (optional, for tree view)
 */
@Serializable
data class LocationResponse(
    val id: String,
    val name: String,
    val description: String?,
    val parentId: String?,
    val path: String,
    val itemCount: Int,
    val childCount: Int,
    val children: List<LocationResponse>? = null
)

/**
 * Container representation with statistics.
 *
 * @property id Unique container identifier
 * @property name Display name
 * @property description Optional description
 * @property locationId Location where container resides
 * @property locationPath Full hierarchical path to location
 * @property parentId Parent container ID if nested
 * @property itemCount Number of items in this container
 * @property childCount Number of child containers
 */
@Serializable
data class ContainerResponse(
    val id: String,
    val name: String,
    val description: String?,
    val locationId: String?,
    val locationPath: String?,
    val parentId: String?,
    val itemCount: Int,
    val childCount: Int
)

/**
 * Item template with field definitions.
 *
 * @property id Unique template identifier
 * @property name Display name
 * @property description Optional description
 * @property icon Optional icon identifier
 * @property fields Field definitions for this template
 * @property itemCount Number of items using this template
 */
@Serializable
data class ItemTemplateResponse(
    val id: String,
    val name: String,
    val description: String?,
    val icon: String?,
    val fields: List<FieldDefinitionResponse>,
    val itemCount: Int
)

/**
 * Field definition within a template.
 *
 * @property id Unique field definition identifier
 * @property name Field name
 * @property fieldType Type of data stored in field
 * @property required Whether field must be populated
 * @property defaultValue Default value when not specified
 * @property enumOptions Valid options for ENUM type fields
 */
@Serializable
data class FieldDefinitionResponse(
    val id: String,
    val name: String,
    val fieldType: FieldType,
    val required: Boolean,
    val defaultValue: String?,
    val enumOptions: List<String>?
)

/**
 * Response containing items below a quantity threshold.
 *
 * @property items Items with low stock
 * @property threshold Quantity threshold used for filtering
 * @property totalCount Total number of low stock items
 */
@Serializable
data class LowStockResponse(
    val items: List<ItemSummaryResponse>,
    val threshold: Int,
    val totalCount: Int
)
