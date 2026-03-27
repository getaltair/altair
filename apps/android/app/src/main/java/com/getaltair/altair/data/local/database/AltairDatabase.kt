package com.getaltair.altair.data.local.database

import androidx.room.Database
import androidx.room.RoomDatabase
import androidx.room.TypeConverters
import com.getaltair.altair.data.local.converter.RoomConverters
import com.getaltair.altair.data.local.dao.AttachmentMetadataDao
import com.getaltair.altair.data.local.dao.CheckinDao
import com.getaltair.altair.data.local.dao.EntityRelationDao
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.FocusSessionDao
import com.getaltair.altair.data.local.dao.HouseholdDao
import com.getaltair.altair.data.local.dao.InitiativeDao
import com.getaltair.altair.data.local.dao.KnowledgeNoteDao
import com.getaltair.altair.data.local.dao.KnowledgeNoteSnapshotDao
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.dao.RoutineDao
import com.getaltair.altair.data.local.dao.TagDao
import com.getaltair.altair.data.local.dao.TrackingCategoryDao
import com.getaltair.altair.data.local.dao.TrackingItemDao
import com.getaltair.altair.data.local.dao.TrackingItemEventDao
import com.getaltair.altair.data.local.dao.TrackingLocationDao
import com.getaltair.altair.data.local.dao.TrackingShoppingListDao
import com.getaltair.altair.data.local.dao.TrackingShoppingListItemDao
import com.getaltair.altair.data.local.dao.UserDao
import com.getaltair.altair.data.local.entity.AttachmentMetadataEntity
import com.getaltair.altair.data.local.entity.CheckinEntity
import com.getaltair.altair.data.local.entity.EntityRelationEntity
import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.data.local.entity.FocusSessionEntity
import com.getaltair.altair.data.local.entity.HouseholdEntity
import com.getaltair.altair.data.local.entity.InitiativeEntity
import com.getaltair.altair.data.local.entity.KnowledgeNoteEntity
import com.getaltair.altair.data.local.entity.KnowledgeNoteSnapshotEntity
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.data.local.entity.RoutineEntity
import com.getaltair.altair.data.local.entity.TagEntity
import com.getaltair.altair.data.local.entity.TrackingCategoryEntity
import com.getaltair.altair.data.local.entity.TrackingItemEntity
import com.getaltair.altair.data.local.entity.TrackingItemEventEntity
import com.getaltair.altair.data.local.entity.TrackingLocationEntity
import com.getaltair.altair.data.local.entity.TrackingShoppingListEntity
import com.getaltair.altair.data.local.entity.TrackingShoppingListItemEntity
import com.getaltair.altair.data.local.entity.UserEntity

@Database(
    entities = [
        UserEntity::class,
        HouseholdEntity::class,
        InitiativeEntity::class,
        EpicEntity::class,
        QuestEntity::class,
        RoutineEntity::class,
        TagEntity::class,
        CheckinEntity::class,
        FocusSessionEntity::class,
        EntityRelationEntity::class,
        KnowledgeNoteEntity::class,
        KnowledgeNoteSnapshotEntity::class,
        TrackingItemEntity::class,
        TrackingItemEventEntity::class,
        TrackingLocationEntity::class,
        TrackingCategoryEntity::class,
        TrackingShoppingListEntity::class,
        TrackingShoppingListItemEntity::class,
        AttachmentMetadataEntity::class,
    ],
    version = 4,
    exportSchema = true,
)
@TypeConverters(RoomConverters::class)
abstract class AltairDatabase : RoomDatabase() {
    abstract fun userDao(): UserDao
    abstract fun householdDao(): HouseholdDao
    abstract fun initiativeDao(): InitiativeDao
    abstract fun epicDao(): EpicDao
    abstract fun questDao(): QuestDao
    abstract fun routineDao(): RoutineDao
    abstract fun tagDao(): TagDao
    abstract fun checkinDao(): CheckinDao
    abstract fun focusSessionDao(): FocusSessionDao
    abstract fun entityRelationDao(): EntityRelationDao
    abstract fun knowledgeNoteDao(): KnowledgeNoteDao
    abstract fun knowledgeNoteSnapshotDao(): KnowledgeNoteSnapshotDao
    abstract fun trackingItemDao(): TrackingItemDao
    abstract fun trackingItemEventDao(): TrackingItemEventDao
    abstract fun trackingLocationDao(): TrackingLocationDao
    abstract fun trackingCategoryDao(): TrackingCategoryDao
    abstract fun trackingShoppingListDao(): TrackingShoppingListDao
    abstract fun trackingShoppingListItemDao(): TrackingShoppingListItemDao
    abstract fun attachmentMetadataDao(): AttachmentMetadataDao
}
