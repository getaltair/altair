package com.getaltair.altair.domain.repository

import com.getaltair.altair.domain.entity.TrackingShoppingList
import com.getaltair.altair.domain.entity.TrackingShoppingListItem
import java.util.UUID
import kotlinx.coroutines.flow.Flow

interface TrackingShoppingListRepository {
    fun getAll(): Flow<List<TrackingShoppingList>>
    fun getById(id: UUID): Flow<TrackingShoppingList?>
    fun getItemsByListId(listId: UUID): Flow<List<TrackingShoppingListItem>>
    suspend fun create(list: TrackingShoppingList)
    suspend fun update(list: TrackingShoppingList)
    suspend fun delete(list: TrackingShoppingList)
    suspend fun addItem(item: TrackingShoppingListItem)
    suspend fun toggleItem(itemId: UUID)
}
