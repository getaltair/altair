package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.TrackingShoppingListDao
import com.getaltair.altair.data.local.dao.TrackingShoppingListItemDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.TrackingShoppingList
import com.getaltair.altair.domain.entity.TrackingShoppingListItem
import com.getaltair.altair.domain.repository.TrackingShoppingListRepository
import com.getaltair.altair.util.mapToDomain
import java.util.UUID
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class TrackingShoppingListRepositoryImpl(
    private val shoppingListDao: TrackingShoppingListDao,
    private val shoppingListItemDao: TrackingShoppingListItemDao,
    private val userId: () -> UUID,
) : TrackingShoppingListRepository {

    override fun getAll(): Flow<List<TrackingShoppingList>> =
        shoppingListDao.getByUserId(userId()).mapToDomain { it.toDomain() }

    override fun getById(id: UUID): Flow<TrackingShoppingList?> =
        shoppingListDao.getById(id).map { it?.toDomain() }

    override fun getItemsByListId(listId: UUID): Flow<List<TrackingShoppingListItem>> =
        shoppingListItemDao.getByListId(listId).mapToDomain { it.toDomain() }

    override suspend fun create(list: TrackingShoppingList) {
        shoppingListDao.insert(list.copy(userId = userId()).toEntity())
    }

    override suspend fun update(list: TrackingShoppingList) {
        shoppingListDao.update(list.toEntity())
    }

    override suspend fun delete(list: TrackingShoppingList) {
        shoppingListDao.delete(list.toEntity())
    }

    override suspend fun addItem(item: TrackingShoppingListItem) {
        shoppingListItemDao.insert(item.toEntity())
    }

    override suspend fun toggleItem(itemId: UUID) {
        shoppingListItemDao.toggleChecked(itemId)
    }
}
