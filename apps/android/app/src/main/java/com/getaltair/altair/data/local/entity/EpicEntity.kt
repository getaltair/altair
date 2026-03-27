package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.ForeignKey
import androidx.room.Index
import androidx.room.PrimaryKey
import java.util.UUID

@Entity(
    tableName = "guidance_epics",
    foreignKeys = [
        ForeignKey(
            entity = InitiativeEntity::class,
            parentColumns = ["id"],
            childColumns = ["initiative_id"],
        ),
        ForeignKey(
            entity = UserEntity::class,
            parentColumns = ["id"],
            childColumns = ["user_id"],
        ),
    ],
    indices = [
        Index(value = ["user_id"]),
        Index(value = ["initiative_id"]),
    ],
)
data class EpicEntity(
    @PrimaryKey
    val id: UUID,

    @ColumnInfo(name = "initiative_id")
    val initiativeId: UUID?,

    @ColumnInfo(name = "user_id")
    val userId: UUID,

    @ColumnInfo(name = "name")
    val name: String,

    @ColumnInfo(name = "description")
    val description: String?,

    @ColumnInfo(name = "status")
    val status: String,

    @ColumnInfo(name = "priority")
    val priority: String,

    @ColumnInfo(name = "created_at")
    val createdAt: Long,

    @ColumnInfo(name = "updated_at")
    val updatedAt: Long,
)
