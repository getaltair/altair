package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "shopping_list_items")
data class ShoppingListItemEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "shopping_list_id") val shoppingListId: String,
    @ColumnInfo(name = "item_id") val itemId: String?,
    @ColumnInfo(name = "name") val name: String,
    @ColumnInfo(name = "quantity") val quantity: Double,
    @ColumnInfo(name = "status") val status: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
