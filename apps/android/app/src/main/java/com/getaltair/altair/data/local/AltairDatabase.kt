package com.getaltair.altair.data.local

import android.content.Context
import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import androidx.sqlite.driver.bundled.BundledSQLiteDriver
import com.getaltair.altair.data.local.dao.*
import com.getaltair.altair.data.local.entity.*
import com.powersync.integrations.room.loadPowerSyncExtension

@Database(
    entities = [
        UserEntity::class,
        HouseholdEntity::class,
        HouseholdMembershipEntity::class,
        InitiativeEntity::class,
        EpicEntity::class,
        QuestEntity::class,
        RoutineEntity::class,
        FocusSessionEntity::class,
        DailyCheckinEntity::class,
        NoteEntity::class,
        NoteSnapshotEntity::class,
        EntityRelationEntity::class,
        TagEntity::class,
        EntityTagEntity::class,
        TrackingItemEntity::class,
        TrackingItemEventEntity::class,
        TrackingLocationEntity::class,
        TrackingCategoryEntity::class,
        ShoppingListEntity::class,
        ShoppingListItemEntity::class,
    ],
    version = 1,
    exportSchema = false,
)
abstract class AltairDatabase : RoomDatabase() {
    abstract fun userDao(): UserDao

    abstract fun householdDao(): HouseholdDao

    abstract fun initiativeDao(): InitiativeDao

    abstract fun epicDao(): EpicDao

    abstract fun questDao(): QuestDao

    abstract fun routineDao(): RoutineDao

    abstract fun focusSessionDao(): FocusSessionDao

    abstract fun dailyCheckinDao(): DailyCheckinDao

    abstract fun noteDao(): NoteDao

    abstract fun noteSnapshotDao(): NoteSnapshotDao

    abstract fun entityRelationDao(): EntityRelationDao

    abstract fun tagDao(): TagDao

    abstract fun trackingItemDao(): TrackingItemDao

    abstract fun trackingItemEventDao(): TrackingItemEventDao

    abstract fun trackingLocationDao(): TrackingLocationDao

    abstract fun trackingCategoryDao(): TrackingCategoryDao

    abstract fun shoppingListDao(): ShoppingListDao

    abstract fun shoppingListItemDao(): ShoppingListItemDao

    companion object {
        fun create(context: Context): AltairDatabase {
            val driver = BundledSQLiteDriver()
            driver.loadPowerSyncExtension()
            return Room
                .databaseBuilder(context, AltairDatabase::class.java, "altair.db")
                .setDriver(driver)
                .build()
        }
    }
}
