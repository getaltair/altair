package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.TrackingShoppingListItemEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingShoppingListItemDao {

    @Query("SELECT * FROM tracking_shopping_list_items WHERE shopping_list_id = :listId ORDER BY created_at ASC")
    fun getByListId(listId: UUID): Flow<List<TrackingShoppingListItemEntity>>

    @Query("SELECT * FROM tracking_shopping_list_items WHERE id = :id")
    fun getById(id: UUID): Flow<TrackingShoppingListItemEntity?>

    @Query("UPDATE tracking_shopping_list_items SET is_checked = CASE WHEN is_checked = 1 THEN 0 ELSE 1 END WHERE id = :id")
    suspend fun toggleChecked(id: UUID)

    @Insert
    suspend fun insert(entity: TrackingShoppingListItemEntity)

    @Update
    suspend fun update(entity: TrackingShoppingListItemEntity)

    @Upsert
    suspend fun upsert(entity: TrackingShoppingListItemEntity)

    @Delete
    suspend fun delete(entity: TrackingShoppingListItemEntity)
}
