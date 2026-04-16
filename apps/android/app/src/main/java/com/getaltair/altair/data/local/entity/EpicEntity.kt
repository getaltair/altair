package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "epics")
data class EpicEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "initiative_id") val initiativeId: String?,
    @ColumnInfo(name = "title") val title: String,
    @ColumnInfo(name = "description") val description: String?,
    @ColumnInfo(name = "status") val status: String,
    @ColumnInfo(name = "sort_order") val sortOrder: Int,
    @ColumnInfo(name = "user_id") val userId: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
