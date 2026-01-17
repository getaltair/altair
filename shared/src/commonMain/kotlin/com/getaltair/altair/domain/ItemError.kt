package com.getaltair.altair.domain

import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Errors specific to Item operations in the Tracking module.
 *
 * These errors extend [DomainError] to enable exhaustive when-matching
 * for Item-specific error handling while maintaining compatibility
 * with generic error handlers.
 */
@Serializable
sealed interface ItemError : DomainError {
    /**
     * The requested item could not be found.
     *
     * @property id The ULID of the item that was not found
     */
    @Serializable
    @SerialName("item_not_found")
    data class NotFound(val id: Ulid) : ItemError {
        override fun toUserMessage(): String = "The requested item could not be found."
    }

    /**
     * The quantity specified for the item is invalid.
     *
     * @property itemId The ULID of the item
     * @property quantity The invalid quantity value
     * @property reason Description of why the quantity is invalid
     */
    @Serializable
    @SerialName("item_invalid_quantity")
    data class InvalidQuantity(
        val itemId: Ulid,
        val quantity: Int,
        val reason: String,
    ) : ItemError {
        override fun toUserMessage(): String = "Invalid quantity: $reason"
    }

    /**
     * The container assignment would create a cycle in the containment hierarchy.
     *
     * This can occur when placing a container inside itself or inside
     * one of its descendants.
     *
     * @property containerId The container that would cause a cycle
     * @property targetContainerId The container it was being placed into
     */
    @Serializable
    @SerialName("item_container_cycle")
    data class ContainerCycle(
        val containerId: Ulid,
        val targetContainerId: Ulid,
    ) : ItemError {
        override fun toUserMessage(): String =
            "Cannot place this container inside itself or one of its contents."
    }

    /**
     * The location specified for the item does not exist.
     *
     * @property locationId The ULID of the location that was not found
     */
    @Serializable
    @SerialName("item_location_not_found")
    data class LocationNotFound(val locationId: Ulid) : ItemError {
        override fun toUserMessage(): String = "The specified location could not be found."
    }

    /**
     * The container specified for the item does not exist.
     *
     * @property containerId The ULID of the container that was not found
     */
    @Serializable
    @SerialName("item_container_not_found")
    data class ContainerNotFound(val containerId: Ulid) : ItemError {
        override fun toUserMessage(): String = "The specified container could not be found."
    }

    /**
     * The template specified for the item does not exist.
     *
     * @property templateId The ULID of the template that was not found
     */
    @Serializable
    @SerialName("item_template_not_found")
    data class TemplateNotFound(val templateId: Ulid) : ItemError {
        override fun toUserMessage(): String = "The specified item template could not be found."
    }
}
