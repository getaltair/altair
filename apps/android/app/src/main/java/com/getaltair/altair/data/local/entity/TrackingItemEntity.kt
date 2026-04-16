package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "tracking_items")
data class TrackingItemEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "name") val name: String,
    @ColumnInfo(name = "description") val description: String?,
    @ColumnInfo(name = "quantity") val quantity: Double,
    @ColumnInfo(name = "barcode") val barcode: String?,
    @ColumnInfo(name = "location_id") val locationId: String?,
    @ColumnInfo(name = "category_id") val categoryId: String?,
    @ColumnInfo(name = "user_id") val userId: String,
    @ColumnInfo(name = "household_id") val householdId: String?,
    @ColumnInfo(name = "initiative_id") val initiativeId: String?,
    @ColumnInfo(name = "expires_at") val expiresAt: String?,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
