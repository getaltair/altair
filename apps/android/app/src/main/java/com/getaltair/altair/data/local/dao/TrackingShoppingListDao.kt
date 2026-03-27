package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.Query
import androidx.room.Update
import androidx.room.Upsert
import com.getaltair.altair.data.local.entity.TrackingShoppingListEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface TrackingShoppingListDao {

    @Query("SELECT * FROM tracking_shopping_lists WHERE household_id = :householdId ORDER BY created_at DESC")
    fun getByHousehold(householdId: UUID): Flow<List<TrackingShoppingListEntity>>

    @Query("SELECT * FROM tracking_shopping_lists WHERE id = :id")
    fun getById(id: UUID): Flow<TrackingShoppingListEntity?>

    @Insert
    suspend fun insert(entity: TrackingShoppingListEntity)

    @Update
    suspend fun update(entity: TrackingShoppingListEntity)

    @Upsert
    suspend fun upsert(entity: TrackingShoppingListEntity)

    @Delete
    suspend fun delete(entity: TrackingShoppingListEntity)
}
