package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "households")
data class HouseholdEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "owner_id") val ownerId: String,
    @ColumnInfo(name = "name") val name: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
