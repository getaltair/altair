package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.ForeignKey
import androidx.room.Index
import androidx.room.PrimaryKey
import java.util.UUID

@Entity(
    tableName = "entity_relations",
    foreignKeys = [
        ForeignKey(
            entity = HouseholdEntity::class,
            parentColumns = ["id"],
            childColumns = ["household_id"],
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
            childColumns = ["owner_user_id"],
            onDelete = ForeignKey.NO_ACTION,
        ),
        ForeignKey(
            entity = UserEntity::class,
            parentColumns = ["id"],
            childColumns = ["created_by_user_id"],
            onDelete = ForeignKey.NO_ACTION,
        ),
        ForeignKey(
            entity = UserEntity::class,
            parentColumns = ["id"],
            childColumns = ["updated_by_user_id"],
            onDelete = ForeignKey.NO_ACTION,
        ),
    ],
    indices = [
        Index(value = ["from_entity_type", "from_entity_id"]),
        Index(value = ["to_entity_type", "to_entity_id"]),
        Index(value = ["status"]),
        Index(value = ["household_id"]),
        Index(value = ["initiative_id"]),
        Index(value = ["owner_user_id"]),
        Index(value = ["created_by_user_id"]),
        Index(value = ["updated_by_user_id"]),
    ],
)
data class EntityRelationEntity(
    @PrimaryKey
    val id: UUID,

    @ColumnInfo(name = "from_entity_type")
    val fromEntityType: String,

    @ColumnInfo(name = "from_entity_id")
    val fromEntityId: UUID,

    @ColumnInfo(name = "to_entity_type")
    val toEntityType: String,

    @ColumnInfo(name = "to_entity_id")
    val toEntityId: UUID,

    @ColumnInfo(name = "relation_type")
    val relationType: String,

    @ColumnInfo(name = "source_type")
    val sourceType: String,

    @ColumnInfo(name = "status")
    val status: String,

    @ColumnInfo(name = "confidence")
    val confidence: Double?,

    @ColumnInfo(name = "evidence_json")
    val evidenceJson: String?,

    @ColumnInfo(name = "household_id")
    val householdId: UUID?,

    @ColumnInfo(name = "initiative_id")
    val initiativeId: UUID?,

    @ColumnInfo(name = "owner_user_id")
    val ownerUserId: UUID?,

    @ColumnInfo(name = "created_by_user_id")
    val createdByUserId: UUID?,

    @ColumnInfo(name = "updated_by_user_id")
    val updatedByUserId: UUID?,

    @ColumnInfo(name = "created_by_process")
    val createdByProcess: String?,

    @ColumnInfo(name = "created_at")
    val createdAt: Long,

    @ColumnInfo(name = "updated_at")
    val updatedAt: Long,

    @ColumnInfo(name = "last_confirmed_at")
    val lastConfirmedAt: Long?,
)
