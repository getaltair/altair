package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "household_memberships")
data class HouseholdMembershipEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "household_id") val householdId: String,
    @ColumnInfo(name = "user_id") val userId: String,
    @ColumnInfo(name = "role") val role: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
