package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.Index
import androidx.room.PrimaryKey
import java.util.UUID

@Entity(
    tableName = "tracking_shopping_list_items",
    indices = [
        Index(value = ["shopping_list_id"]),
    ],
)
data class TrackingShoppingListItemEntity(
    @PrimaryKey
    val id: UUID,

    @ColumnInfo(name = "shopping_list_id")
    val shoppingListId: UUID,

    @ColumnInfo(name = "item_id")
    val itemId: UUID?,

    @ColumnInfo(name = "name")
    val name: String,

    @ColumnInfo(name = "quantity")
    val quantity: Int,

    @ColumnInfo(name = "unit")
    val unit: String?,

    @ColumnInfo(name = "is_checked")
    val isChecked: Int,

    @ColumnInfo(name = "created_at")
    val createdAt: Long,
)
