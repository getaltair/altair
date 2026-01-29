package com.getaltair.altair.shared.dto.system

import com.getaltair.altair.shared.domain.common.CaptureSource
import com.getaltair.altair.shared.domain.common.InitiativeStatus
import kotlinx.serialization.Serializable

// ==================== Initiative DTOs ====================

/**
 * Request to create a new Initiative (Project or Area).
 *
 * @property name The initiative name (required, max 200 characters)
 * @property description Optional markdown description
 * @property parentId Optional parent Initiative ID for hierarchical organization
 * @property ongoing True for Areas (no completion), False for Projects (completable)
 * @property targetDate Target completion date in YYYY-MM-DD format (ignored if ongoing=true)
 */
@Serializable
data class CreateInitiativeRequest(
    val name: String,
    val description: String? = null,
    val parentId: String? = null,
    val ongoing: Boolean = false,
    val targetDate: String? = null
)

/**
 * Request to update an existing Initiative.
 * All fields are optional - only provided fields will be updated.
 *
 * @property name New initiative name
 * @property description New description
 * @property parentId New parent Initiative ID (use null to unlink)
 * @property ongoing Whether this is an ongoing Area
 * @property targetDate New target date in YYYY-MM-DD format
 * @property status New lifecycle status
 */
@Serializable
data class UpdateInitiativeRequest(
    val name: String? = null,
    val description: String? = null,
    val parentId: String? = null,
    val ongoing: Boolean? = null,
    val targetDate: String? = null,
    val status: InitiativeStatus? = null
)

/**
 * Full Initiative representation for API responses.
 *
 * @property id Unique Initiative identifier
 * @property name Initiative name
 * @property description Detailed description (nullable)
 * @property parentId Parent Initiative ID for hierarchy (nullable)
 * @property ongoing True for Areas, False for Projects
 * @property targetDate Target completion date in YYYY-MM-DD format (nullable)
 * @property status Current lifecycle status
 * @property focused Whether this is the user's currently focused initiative
 * @property questCount Total number of Quests associated with this initiative
 * @property noteCount Total number of Notes associated with this initiative
 * @property itemCount Total number of Items associated with this initiative
 * @property children List of child Initiatives for tree view (nullable)
 * @property createdAt ISO 8601 timestamp
 * @property updatedAt ISO 8601 timestamp
 */
@Serializable
data class InitiativeResponse(
    val id: String,
    val name: String,
    val description: String?,
    val parentId: String?,
    val ongoing: Boolean,
    val targetDate: String?,
    val status: InitiativeStatus,
    val focused: Boolean,
    val questCount: Int,
    val noteCount: Int,
    val itemCount: Int,
    val children: List<InitiativeResponse>?,
    val createdAt: String,
    val updatedAt: String
)

// ==================== Inbox DTOs ====================

/**
 * Request to create a new Universal Inbox capture item.
 *
 * @property content Text content captured by the user (required)
 * @property source How this item was captured (keyboard, voice, camera, etc.)
 */
@Serializable
data class CreateInboxItemRequest(
    val content: String,
    val source: CaptureSource
)

/**
 * Inbox item representation for API responses.
 *
 * @property id Unique inbox item identifier
 * @property content Captured text content
 * @property source How this item was captured
 * @property attachments List of file attachments
 * @property createdAt ISO 8601 timestamp
 */
@Serializable
data class InboxItemResponse(
    val id: String,
    val content: String,
    val source: CaptureSource,
    val attachments: List<InboxAttachmentResponse>,
    val createdAt: String
)

/**
 * Attachment metadata for inbox items.
 *
 * @property id Unique attachment identifier
 * @property filename Original filename
 * @property mimeType MIME type of the file
 * @property sizeBytes File size in bytes
 */
@Serializable
data class InboxAttachmentResponse(
    val id: String,
    val filename: String,
    val mimeType: String,
    val sizeBytes: Long
)

/**
 * Request to triage an inbox item into a specific target entity.
 *
 * @property targetType Type of entity to create: "quest", "note", "item", "source_document"
 * @property title Title for the new entity (required)
 * @property initiativeId Optional Initiative ID to associate with the new entity
 * @property energyCost Energy cost for Quest (required if targetType="quest")
 * @property folderId Folder ID for Note (optional if targetType="note")
 * @property locationId Location ID for Item (optional if targetType="item")
 */
@Serializable
data class TriageRequest(
    val targetType: String,
    val title: String,
    val initiativeId: String? = null,
    val energyCost: Int? = null,
    val folderId: String? = null,
    val locationId: String? = null
)

/**
 * Response after successful inbox item triage.
 *
 * @property targetType Type of entity created
 * @property entityId ID of the newly created entity
 */
@Serializable
data class TriageResponse(
    val targetType: String,
    val entityId: String
)

// ==================== Routine DTOs ====================

/**
 * Request to create a new recurring Routine.
 *
 * @property name The routine name (required, max 200 characters)
 * @property description Optional markdown description
 * @property schedule Serialized schedule string (e.g., "daily", "weekly:1,3,5", "monthly:15")
 * @property timeOfDay Specific time when Quest should spawn in HH:mm format (nullable)
 * @property energyCost Energy cost for spawned Quests (1-5)
 * @property initiativeId Optional Initiative ID to assign to spawned Quests
 */
@Serializable
data class CreateRoutineRequest(
    val name: String,
    val description: String? = null,
    val schedule: String,
    val timeOfDay: String? = null,
    val energyCost: Int,
    val initiativeId: String? = null
)

/**
 * Request to update an existing Routine.
 * All fields are optional - only provided fields will be updated.
 *
 * @property name New routine name
 * @property description New description
 * @property schedule New schedule string
 * @property timeOfDay New time of day in HH:mm format
 * @property energyCost New energy cost
 * @property active Whether this routine is currently spawning Quests
 */
@Serializable
data class UpdateRoutineRequest(
    val name: String? = null,
    val description: String? = null,
    val schedule: String? = null,
    val timeOfDay: String? = null,
    val energyCost: Int? = null,
    val active: Boolean? = null
)

/**
 * Routine representation for API responses.
 *
 * @property id Unique Routine identifier
 * @property name Routine name
 * @property description Detailed description (nullable)
 * @property schedule Serialized schedule string
 * @property scheduleDescription Human-readable schedule description (e.g., "Every Monday and Friday")
 * @property timeOfDay Time when Quest spawns in HH:mm format (nullable)
 * @property energyCost Energy cost for spawned Quests (1-5)
 * @property initiativeId Parent Initiative ID (nullable)
 * @property active Whether this routine is currently spawning Quests
 * @property nextDue ISO 8601 timestamp of next scheduled spawn (nullable)
 * @property createdAt ISO 8601 timestamp
 * @property updatedAt ISO 8601 timestamp
 */
@Serializable
data class RoutineResponse(
    val id: String,
    val name: String,
    val description: String?,
    val schedule: String,
    val scheduleDescription: String,
    val timeOfDay: String?,
    val energyCost: Int,
    val initiativeId: String?,
    val active: Boolean,
    val nextDue: String?,
    val createdAt: String,
    val updatedAt: String
)
