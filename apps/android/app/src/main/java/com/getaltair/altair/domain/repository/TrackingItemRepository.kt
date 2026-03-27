package com.getaltair.altair.domain.repository

import com.getaltair.altair.domain.entity.TrackingItem
import java.util.UUID
import kotlinx.coroutines.flow.Flow

interface TrackingItemRepository {
    fun getAll(): Flow<List<TrackingItem>>
    fun getById(id: UUID): Flow<TrackingItem?>
    fun search(query: String): Flow<List<TrackingItem>>
    fun getByBarcode(barcode: String): Flow<TrackingItem?>
    suspend fun create(item: TrackingItem)
    suspend fun update(item: TrackingItem)
    suspend fun delete(item: TrackingItem)
}
