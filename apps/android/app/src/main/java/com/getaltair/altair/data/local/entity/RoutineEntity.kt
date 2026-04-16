package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "routines")
data class RoutineEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "title") val title: String,
    @ColumnInfo(name = "description") val description: String?,
    @ColumnInfo(name = "frequency_type") val frequencyType: String,
    @ColumnInfo(name = "frequency_config") val frequencyConfig: String?,
    @ColumnInfo(name = "status") val status: String,
    @ColumnInfo(name = "user_id") val userId: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
