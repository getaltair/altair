package com.getaltair.altair.di

import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.sync.AltairPowerSyncSchema
import com.powersync.integrations.room.RoomConnectionPool
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

val databaseModule =
    module {
        single<AltairDatabase> {
            AltairDatabase.create(androidContext())
        }

        single<RoomConnectionPool> {
            RoomConnectionPool(get<AltairDatabase>(), AltairPowerSyncSchema)
        }

        single { get<AltairDatabase>().userDao() }
        single { get<AltairDatabase>().householdDao() }
        single { get<AltairDatabase>().initiativeDao() }
        single { get<AltairDatabase>().epicDao() }
        single { get<AltairDatabase>().questDao() }
        single { get<AltairDatabase>().routineDao() }
        single { get<AltairDatabase>().focusSessionDao() }
        single { get<AltairDatabase>().dailyCheckinDao() }
        single { get<AltairDatabase>().noteDao() }
        single { get<AltairDatabase>().noteSnapshotDao() }
        single { get<AltairDatabase>().entityRelationDao() }
        single { get<AltairDatabase>().tagDao() }
        single { get<AltairDatabase>().trackingItemDao() }
        single { get<AltairDatabase>().trackingItemEventDao() }
        single { get<AltairDatabase>().trackingLocationDao() }
        single { get<AltairDatabase>().trackingCategoryDao() }
        single { get<AltairDatabase>().shoppingListDao() }
        single { get<AltairDatabase>().shoppingListItemDao() }
    }
