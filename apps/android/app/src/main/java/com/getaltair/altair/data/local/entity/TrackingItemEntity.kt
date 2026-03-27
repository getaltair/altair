package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey
import java.util.UUID

@Entity(
    tableName = "tracking_items",
    indices = [
        Index(value = ["user_id"]),
        Index(value = ["household_id"]),
        Index(value = ["category_id"]),
        Index(value = ["location_id"]),
        Index(value = ["status"]),
    ],
)
data class TrackingItemEntity(
    @PrimaryKey
    val id: UUID,

    @ColumnInfo(name = "user_id")
    val userId: UUID,

    @ColumnInfo(name = "household_id")
    val householdId: UUID,

    @ColumnInfo(name = "category_id")
    val categoryId: UUID?,

    @ColumnInfo(name = "location_id")
    val locationId: UUID?,

    @ColumnInfo(name = "name")
    val name: String,

    @ColumnInfo(name = "description")
    val description: String?,

    @ColumnInfo(name = "quantity")
    val quantity: Int,

    @ColumnInfo(name = "unit")
    val unit: String?,

    @ColumnInfo(name = "min_quantity")
    val minQuantity: Int?,

    @ColumnInfo(name = "barcode")
    val barcode: String?,

    @ColumnInfo(name = "status")
    val status: String,

    @ColumnInfo(name = "created_at")
    val createdAt: Long,

    @ColumnInfo(name = "updated_at")
    val updatedAt: Long,
)
