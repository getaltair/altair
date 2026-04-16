package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "entity_tags")
data class EntityTagEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "entity_id") val entityId: String,
    @ColumnInfo(name = "entity_type") val entityType: String,
    @ColumnInfo(name = "tag_id") val tagId: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
)
