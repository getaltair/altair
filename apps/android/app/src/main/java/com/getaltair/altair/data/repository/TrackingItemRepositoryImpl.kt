package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.TrackingItemDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.TrackingItem
import com.getaltair.altair.domain.repository.TrackingItemRepository
import com.getaltair.altair.util.mapToDomain
import java.util.UUID
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class TrackingItemRepositoryImpl(
    private val trackingItemDao: TrackingItemDao,
    private val userId: () -> UUID,
) : TrackingItemRepository {

    override fun getAll(): Flow<List<TrackingItem>> =
        trackingItemDao.getByUserId(userId()).mapToDomain { it.toDomain() }

    override fun getById(id: UUID): Flow<TrackingItem?> =
        trackingItemDao.getById(id).map { it?.toDomain() }

    override fun search(query: String): Flow<List<TrackingItem>> =
        trackingItemDao.searchByName(userId(), query).mapToDomain { it.toDomain() }

    override fun getByBarcode(barcode: String): Flow<TrackingItem?> =
        trackingItemDao.getByBarcode(userId(), barcode).map { it?.toDomain() }

    override suspend fun create(item: TrackingItem) {
        trackingItemDao.insert(item.toEntity())
    }

    override suspend fun update(item: TrackingItem) {
        trackingItemDao.update(item.toEntity())
    }

    override suspend fun delete(item: TrackingItem) {
        trackingItemDao.delete(item.toEntity())
    }
}
