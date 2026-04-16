package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "quests")
data class QuestEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "title") val title: String,
    @ColumnInfo(name = "description") val description: String?,
    @ColumnInfo(name = "status") val status: String,
    @ColumnInfo(name = "priority") val priority: String,
    @ColumnInfo(name = "due_date") val dueDate: String?,
    @ColumnInfo(name = "epic_id") val epicId: String?,
    @ColumnInfo(name = "initiative_id") val initiativeId: String?,
    @ColumnInfo(name = "routine_id") val routineId: String?,
    @ColumnInfo(name = "user_id") val userId: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
