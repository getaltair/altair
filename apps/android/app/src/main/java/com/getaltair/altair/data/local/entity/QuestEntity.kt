package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.ForeignKey
import androidx.room.Index
import androidx.room.PrimaryKey
import java.util.UUID

@Entity(
    tableName = "guidance_quests",
    foreignKeys = [
        ForeignKey(
            entity = EpicEntity::class,
            parentColumns = ["id"],
            childColumns = ["epic_id"],
            onDelete = ForeignKey.NO_ACTION,
        ),
        ForeignKey(
            entity = InitiativeEntity::class,
            parentColumns = ["id"],
            childColumns = ["initiative_id"],
            onDelete = ForeignKey.NO_ACTION,
        ),
        ForeignKey(
            entity = UserEntity::class,
            parentColumns = ["id"],
            childColumns = ["user_id"],
            onDelete = ForeignKey.NO_ACTION,
        ),
        ForeignKey(
            entity = HouseholdEntity::class,
            parentColumns = ["id"],
            childColumns = ["household_id"],
            onDelete = ForeignKey.NO_ACTION,
        ),
    ],
    indices = [
        Index(value = ["user_id"]),
        Index(value = ["epic_id"]),
        Index(value = ["initiative_id"]),
        Index(value = ["household_id"]),
        Index(value = ["status"]),
        Index(value = ["due_date"]),
    ],
)
data class QuestEntity(
    @PrimaryKey
    val id: UUID,

    @ColumnInfo(name = "epic_id")
    val epicId: UUID?,

    @ColumnInfo(name = "initiative_id")
    val initiativeId: UUID?,

    @ColumnInfo(name = "user_id")
    val userId: UUID,

    @ColumnInfo(name = "household_id")
    val householdId: UUID?,

    @ColumnInfo(name = "name")
    val name: String,

    @ColumnInfo(name = "description")
    val description: String?,

    @ColumnInfo(name = "status")
    val status: String,

    @ColumnInfo(name = "priority")
    val priority: String,

    @ColumnInfo(name = "due_date")
    val dueDate: String?,

    @ColumnInfo(name = "estimated_minutes")
    val estimatedMinutes: Int?,

    @ColumnInfo(name = "completed_at")
    val completedAt: Long?,

    @ColumnInfo(name = "created_at")
    val createdAt: Long,

    @ColumnInfo(name = "updated_at")
    val updatedAt: Long,
)
