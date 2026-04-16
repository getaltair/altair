package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "entity_relations")
data class EntityRelationEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "from_entity_type") val fromEntityType: String,
    @ColumnInfo(name = "from_entity_id") val fromEntityId: String,
    @ColumnInfo(name = "to_entity_type") val toEntityType: String,
    @ColumnInfo(name = "to_entity_id") val toEntityId: String,
    @ColumnInfo(name = "relation_type") val relationType: String,
    @ColumnInfo(name = "source_type") val sourceType: String,
    @ColumnInfo(name = "status") val status: String,
    @ColumnInfo(name = "confidence") val confidence: Double?,
    @ColumnInfo(name = "evidence") val evidence: String?,
    @ColumnInfo(name = "user_id") val userId: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
