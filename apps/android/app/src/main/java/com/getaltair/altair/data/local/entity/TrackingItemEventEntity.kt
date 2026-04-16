package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "tracking_item_events")
data class TrackingItemEventEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "item_id") val itemId: String,
    @ColumnInfo(name = "event_type") val eventType: String,
    @ColumnInfo(name = "quantity_change") val quantityChange: Double?,
    @ColumnInfo(name = "from_location_id") val fromLocationId: String?,
    @ColumnInfo(name = "to_location_id") val toLocationId: String?,
    @ColumnInfo(name = "notes") val notes: String?,
    @ColumnInfo(name = "occurred_at") val occurredAt: String?,
    @ColumnInfo(name = "created_at") val createdAt: String,
)
