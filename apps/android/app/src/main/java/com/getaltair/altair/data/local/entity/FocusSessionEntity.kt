package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "focus_sessions")
data class FocusSessionEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "quest_id") val questId: String?,
    @ColumnInfo(name = "started_at") val startedAt: String,
    @ColumnInfo(name = "ended_at") val endedAt: String?,
    @ColumnInfo(name = "duration_minutes") val durationMinutes: Int?,
    @ColumnInfo(name = "user_id") val userId: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
