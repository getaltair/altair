package com.getaltair.altair.dto.tracking

import kotlinx.serialization.Serializable

/**
 * Request to create a new item.
 */
@Serializable
data class CreateItemRequest(
    val name: String,
    val description: String? = null,
    val templateId: String? = null,
    val locationId: String? = null,
    val containerId: String? = null,
    val quantity: Int = 1,
    val initiativeId: String? = null,
    val customFields: Map<String, String> = emptyMap(),
)

/**
 * Request to update an existing item.
 */
@Serializable
data class UpdateItemRequest(
    val name: String? = null,
    val description: String? = null,
    val locationId: String? = null,
    val containerId: String? = null,
    val quantity: Int? = null,
    val initiativeId: String? = null,
    val customFields: Map<String, String>? = null,
)

/**
 * Response containing item data.
 *
 * ## String Formats
 * - **IDs** (`id`, `templateId`, `locationId`, `containerId`, `photoAttachmentId`,
 *   `initiativeId`): ULID format (26-character string)
 * - **Timestamps** (`createdAt`, `updatedAt`): ISO-8601 datetime (YYYY-MM-DDTHH:MM:SSZ)
 * - **Paths** (`locationPath`, `containerPath`): Human-readable breadcrumb (e.g., "Home > Office > Desk")
 */
@Serializable
data class ItemResponse(
    val id: String,
    val name: String,
    val description: String?,
    val templateId: String?,
    val locationId: String?,
    val containerId: String?,
    val quantity: Int,
    val photoAttachmentId: String?,
    val initiativeId: String?,
    val createdAt: String,
    val updatedAt: String,
    val customFields: Map<String, String>,
    val locationPath: String?,
    val containerPath: String?,
)

/**
 * Request to create a new location.
 */
@Serializable
data class CreateLocationRequest(
    val name: String,
    val description: String? = null,
    val parentId: String? = null,
    val address: String? = null,
)

/**
 * Request to update an existing location.
 */
@Serializable
data class UpdateLocationRequest(
    val name: String? = null,
    val description: String? = null,
    val parentId: String? = null,
    val address: String? = null,
)

/**
 * Response containing location data.
 */
@Serializable
data class LocationResponse(
    val id: String,
    val name: String,
    val description: String?,
    val parentId: String?,
    val address: String?,
    val createdAt: String,
    val updatedAt: String,
    val itemCount: Int,
    val containerCount: Int,
)

/**
 * Request to create a new container.
 */
@Serializable
data class CreateContainerRequest(
    val name: String,
    val description: String? = null,
    val locationId: String? = null,
    val parentContainerId: String? = null,
    val label: String? = null,
)

/**
 * Request to update an existing container.
 */
@Serializable
data class UpdateContainerRequest(
    val name: String? = null,
    val description: String? = null,
    val locationId: String? = null,
    val parentContainerId: String? = null,
    val label: String? = null,
)

/**
 * Response containing container data.
 */
@Serializable
data class ContainerResponse(
    val id: String,
    val name: String,
    val description: String?,
    val locationId: String?,
    val parentContainerId: String?,
    val label: String?,
    val createdAt: String,
    val updatedAt: String,
    val itemCount: Int,
    val nestedContainerCount: Int,
)

/**
 * Request to create a new item template.
 */
@Serializable
data class CreateItemTemplateRequest(
    val name: String,
    val description: String? = null,
    val icon: String? = null,
    val fields: List<FieldDefinitionRequest> = emptyList(),
)

/**
 * Request to update an existing item template.
 */
@Serializable
data class UpdateItemTemplateRequest(
    val name: String? = null,
    val description: String? = null,
    val icon: String? = null,
)

/**
 * Request to define a field in a template.
 */
@Serializable
data class FieldDefinitionRequest(
    val name: String,
    val fieldType: String,
    val isRequired: Boolean = false,
    val defaultValue: String? = null,
    val enumOptions: List<String>? = null,
)

/**
 * Response containing item template data.
 */
@Serializable
data class ItemTemplateResponse(
    val id: String,
    val name: String,
    val description: String?,
    val icon: String?,
    val createdAt: String,
    val updatedAt: String,
    val fields: List<FieldDefinitionResponse>,
    val itemCount: Int,
)

/**
 * Response containing field definition data.
 */
@Serializable
data class FieldDefinitionResponse(
    val id: String,
    val name: String,
    val fieldType: String,
    val isRequired: Boolean,
    val defaultValue: String?,
    val enumOptions: List<String>?,
    val sortOrder: Int,
)

/**
 * Request to move an item to a new location.
 */
@Serializable
data class MoveItemRequest(
    val locationId: String? = null,
    val containerId: String? = null,
)
