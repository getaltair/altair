package com.getaltair.altair.dto.system

import kotlinx.serialization.Serializable

/**
 * Request to create a new initiative.
 */
@Serializable
data class CreateInitiativeRequest(
    val name: String,
    val description: String? = null,
    val color: String? = null,
    val icon: String? = null,
)

/**
 * Request to update an existing initiative.
 */
@Serializable
data class UpdateInitiativeRequest(
    val name: String? = null,
    val description: String? = null,
    val color: String? = null,
    val icon: String? = null,
    val status: String? = null,
)

/**
 * Response containing initiative data.
 */
@Serializable
data class InitiativeResponse(
    val id: String,
    val name: String,
    val description: String?,
    val color: String?,
    val icon: String?,
    val status: String,
    val createdAt: String,
    val updatedAt: String,
    val linkedEntityCount: Int,
)

/**
 * Request to capture an item into the inbox.
 */
@Serializable
data class CaptureRequest(
    val content: String,
    val source: String,
    val attachmentIds: List<String> = emptyList(),
)

/**
 * Request to triage an inbox item into another entity.
 */
@Serializable
data class TriageRequest(
    val inboxItemId: String,
    val targetType: String,
    val targetData: TriageTargetData,
)

/**
 * Target data for triaging inbox items.
 */
@Serializable
data class TriageTargetData(
    val title: String? = null,
    val content: String? = null,
    val energyCost: Int? = null,
    val epicId: String? = null,
    val folderId: String? = null,
    val initiativeId: String? = null,
    val templateId: String? = null,
    val locationId: String? = null,
)

/**
 * Response containing inbox item data.
 */
@Serializable
data class InboxItemResponse(
    val id: String,
    val content: String,
    val source: String,
    val attachmentIds: List<String>,
    val createdAt: String,
    val updatedAt: String,
)

/**
 * Request to create a new routine.
 */
@Serializable
data class CreateRoutineRequest(
    val title: String,
    val description: String? = null,
    val energyCost: Int,
    val schedule: ScheduleRequest,
    val scheduledTime: String? = null,
    val initiativeId: String? = null,
)

/**
 * Request to update an existing routine.
 */
@Serializable
data class UpdateRoutineRequest(
    val title: String? = null,
    val description: String? = null,
    val energyCost: Int? = null,
    val schedule: ScheduleRequest? = null,
    val scheduledTime: String? = null,
    val initiativeId: String? = null,
    val isActive: Boolean? = null,
)

/**
 * Schedule configuration for routines.
 */
@Serializable
data class ScheduleRequest(
    val type: String,
    val daysOfWeek: List<String>? = null,
    val dayOfMonth: Int? = null,
    val weekOfMonth: String? = null,
    val intervalDays: Int? = null,
)

/**
 * Response containing routine data.
 */
@Serializable
data class RoutineResponse(
    val id: String,
    val title: String,
    val description: String?,
    val energyCost: Int,
    val schedule: ScheduleResponse,
    val scheduledTime: String?,
    val initiativeId: String?,
    val isActive: Boolean,
    val lastSpawnedAt: String?,
    val createdAt: String,
    val updatedAt: String,
)

/**
 * Schedule response with serialized schedule data.
 */
@Serializable
data class ScheduleResponse(
    val type: String,
    val displayText: String,
    val daysOfWeek: List<String>?,
    val dayOfMonth: Int?,
    val weekOfMonth: String?,
    val intervalDays: Int?,
)
