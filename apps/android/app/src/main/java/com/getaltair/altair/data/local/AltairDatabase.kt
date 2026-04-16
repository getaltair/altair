package com.getaltair.altair.data.local

import android.content.Context
import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import androidx.room.migration.Migration
import androidx.sqlite.db.SupportSQLiteDatabase
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
    version = 2,
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
        // Removes password_hash from users; all data re-synced via PowerSync
        private val MIGRATION_1_2 =
            object : Migration(1, 2) {
                override fun migrate(db: SupportSQLiteDatabase) {
                    db.execSQL(
                        "CREATE TABLE users_new (id TEXT NOT NULL PRIMARY KEY, email TEXT NOT NULL, display_name TEXT, is_admin INTEGER NOT NULL DEFAULT 0, status TEXT NOT NULL DEFAULT '', created_at TEXT NOT NULL DEFAULT '', updated_at TEXT NOT NULL DEFAULT '', deleted_at TEXT)",
                    )
                    db.execSQL(
                        "INSERT INTO users_new SELECT id, email, display_name, is_admin, status, created_at, updated_at, deleted_at FROM users",
                    )
                    db.execSQL("DROP TABLE users")
                    db.execSQL("ALTER TABLE users_new RENAME TO users")
                }
            }

        fun create(context: Context): AltairDatabase {
            val driver = BundledSQLiteDriver()
            driver.loadPowerSyncExtension()
            return Room
                .databaseBuilder(context, AltairDatabase::class.java, "altair.db")
                .setDriver(driver)
                .addMigrations(MIGRATION_1_2)
                .build()
        }
    }
}
