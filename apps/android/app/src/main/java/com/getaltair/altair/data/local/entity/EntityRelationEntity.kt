package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey
import java.util.UUID

@Entity(
    tableName = "entity_relations",
    indices = [
        Index(value = ["from_entity_type", "from_entity_id"]),
        Index(value = ["to_entity_type", "to_entity_id"]),
        Index(value = ["status"]),
    ],
)
data class EntityRelationEntity(
    @PrimaryKey
    val id: UUID,

    @ColumnInfo(name = "from_entity_type")
    val fromEntityType: String,

    @ColumnInfo(name = "from_entity_id")
    val fromEntityId: UUID,

    @ColumnInfo(name = "to_entity_type")
    val toEntityType: String,

    @ColumnInfo(name = "to_entity_id")
    val toEntityId: UUID,

    @ColumnInfo(name = "relation_type")
    val relationType: String,

    @ColumnInfo(name = "source_type")
    val sourceType: String,

    @ColumnInfo(name = "status")
    val status: String,

    @ColumnInfo(name = "confidence")
    val confidence: Double?,

    @ColumnInfo(name = "created_at")
    val createdAt: Long,

    @ColumnInfo(name = "updated_at")
    val updatedAt: Long,
)
