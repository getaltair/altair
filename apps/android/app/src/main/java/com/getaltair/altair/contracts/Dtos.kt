package com.getaltair.altair.contracts

import kotlinx.serialization.Serializable

// DTOs wrapping contracts defined in EntityType.kt, RelationType.kt, and SyncStream.kt.

/** A polymorphic reference to any entity by type and UUID. */
@Serializable
data class EntityRef(
    val entityType: EntityType,
    val entityId: String, // UUID string
)

/** Mirrors the entity_relations table schema from docs/specs/05-erd.md. */
@Serializable
data class RelationRecord(
    val id: String,
    val fromEntityType: EntityType,
    val fromEntityId: String,
    val toEntityType: EntityType,
    val toEntityId: String,
    val relationType: RelationType,
    val sourceType: String,
    val status: String,
    val confidence: Double?,
    val evidence: String?,
    val userId: String,
    val createdAt: String, // ISO 8601
    val updatedAt: String,
    val deletedAt: String?,
)

/** Mirrors the attachments table schema from docs/specs/05-erd.md. */
@Serializable
data class AttachmentRecord(
    val id: String,
    val entityType: EntityType,
    val entityId: String,
    val fileName: String,
    val contentType: String,
    val sizeBytes: Long?,
    val state: String,
    val storagePath: String?,
    val userId: String,
    val createdAt: String, // ISO 8601
    val updatedAt: String,
    val deletedAt: String?,
)

/** Parameters for subscribing to PowerSync sync streams. */
@Serializable
data class SyncSubscriptionRequest(
    val streams: List<String>, // SyncStream.value
    val userId: String,
)
