package com.getaltair.altair.contracts

// Source of truth: packages/contracts/entity-types.json, relation-types.json, sync-streams.json
// Note: JSON serialization annotations are deferred to Step 8 (Android Client)
//       when the JSON library (Gson / Moshi / kotlinx.serialization) is chosen.

/** A polymorphic reference to any entity by type and UUID. */
data class EntityRef(
    val entityType: String, // EntityType.value
    val entityId: String, // UUID string
)

/** Mirrors the entity_relations table schema from docs/specs/05-erd.md. */
data class RelationRecord(
    val id: String,
    val fromEntityType: String, // EntityType.value
    val fromEntityId: String,
    val toEntityType: String, // EntityType.value
    val toEntityId: String,
    val relationType: String, // RelationType.value
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
data class AttachmentRecord(
    val id: String,
    val entityType: String, // EntityType.value
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
data class SyncSubscriptionRequest(
    val streams: List<String>, // SyncStream.value
    val userId: String,
)
