package com.getaltair.altair.data.local.dao

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.getaltair.altair.data.local.entity.AttachmentMetadataEntity
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface AttachmentMetadataDao {
    @Query("SELECT * FROM attachment_metadata WHERE entity_id = :entityId")
    fun getByEntityId(entityId: UUID): Flow<List<AttachmentMetadataEntity>>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(entity: AttachmentMetadataEntity)
}
