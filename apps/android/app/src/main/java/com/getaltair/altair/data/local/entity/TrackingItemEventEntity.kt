package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey
import java.util.UUID

@Entity(
    tableName = "tracking_item_events",
    indices = [
        Index(value = ["item_id"]),
        Index(value = ["user_id"]),
    ],
)
data class TrackingItemEventEntity(
    @PrimaryKey
    val id: UUID,

    @ColumnInfo(name = "item_id")
    val itemId: UUID,

    @ColumnInfo(name = "user_id")
    val userId: UUID,

    @ColumnInfo(name = "event_type")
    val eventType: String,

    @ColumnInfo(name = "quantity_change")
    val quantityChange: Int,

    @ColumnInfo(name = "notes")
    val notes: String?,

    @ColumnInfo(name = "created_at")
    val createdAt: Long,
)
