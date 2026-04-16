package com.getaltair.altair.di

import android.app.Application
import android.content.Context
import android.content.SharedPreferences
import androidx.lifecycle.SavedStateHandle
import androidx.room.RoomDatabase
import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.local.dao.DailyCheckinDao
import com.getaltair.altair.data.local.dao.EntityRelationDao
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.FocusSessionDao
import com.getaltair.altair.data.local.dao.HouseholdDao
import com.getaltair.altair.data.local.dao.InitiativeDao
import com.getaltair.altair.data.local.dao.NoteDao
import com.getaltair.altair.data.local.dao.NoteSnapshotDao
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.dao.RoutineDao
import com.getaltair.altair.data.local.dao.ShoppingListDao
import com.getaltair.altair.data.local.dao.ShoppingListItemDao
import com.getaltair.altair.data.local.dao.TagDao
import com.getaltair.altair.data.local.dao.TrackingCategoryDao
import com.getaltair.altair.data.local.dao.TrackingItemDao
import com.getaltair.altair.data.local.dao.TrackingItemEventDao
import com.getaltair.altair.data.local.dao.TrackingLocationDao
import com.getaltair.altair.data.local.dao.UserDao
import com.getaltair.altair.data.network.AuthApi
import com.getaltair.altair.data.network.SyncApi
import com.getaltair.altair.data.sync.AltairPowerSyncConnector
import com.getaltair.altair.data.sync.SyncCoordinator
import com.getaltair.altair.domain.repository.AuthRepository
import com.powersync.PowerSyncDatabase
import com.powersync.integrations.room.RoomConnectionPool
import org.junit.Test
import org.koin.test.verify.verify

/**
 * Verifies the full Koin DI graph has no missing or circular dependencies (FA-014).
 *
 * Each module is verified with extraTypes listing all types that a module may need
 * which are provided by other modules at runtime. Module.verify() checks each module
 * in isolation, so cross-module dependencies must be declared as extraTypes.
 */
class KoinModulesTest {
    @Test
    fun koin_modules_have_no_missing_or_circular_dependencies() {
        val extraTypes =
            listOf(
                // Android framework — provided by Koin-Android at runtime
                Application::class,
                Context::class,
                SharedPreferences::class,
                SavedStateHandle::class,
                // Room — AltairDatabase and its supertype needed by RoomConnectionPool ctor
                RoomDatabase::class,
                AltairDatabase::class,
                // From databaseModule — DAOs consumed by viewModelModule
                UserDao::class,
                QuestDao::class,
                RoutineDao::class,
                DailyCheckinDao::class,
                InitiativeDao::class,
                EpicDao::class,
                FocusSessionDao::class,
                NoteDao::class,
                NoteSnapshotDao::class,
                EntityRelationDao::class,
                TagDao::class,
                TrackingItemDao::class,
                TrackingItemEventDao::class,
                TrackingLocationDao::class,
                TrackingCategoryDao::class,
                ShoppingListDao::class,
                ShoppingListItemDao::class,
                HouseholdDao::class,
                RoomConnectionPool::class,
                // From preferencesModule — consumed by networkModule, syncModule,
                // repositoryModule, and viewModelModule
                TokenPreferences::class,
                // From networkModule — consumed by repositoryModule
                AuthApi::class,
                SyncApi::class,
                // From syncModule — consumed by repositoryModule and viewModelModule
                PowerSyncDatabase::class,
                SyncCoordinator::class,
                AltairPowerSyncConnector::class,
                // From repositoryModule — consumed by viewModelModule
                AuthRepository::class,
                // Kotlin function/lambda types — used as constructor params in
                // AltairPowerSyncConnector (getToken, uploadToServer) and are provided
                // inline in the factory lambda, not injected via get()
                Function0::class,
                Function1::class,
                Function2::class,
                Function3::class,
                Function4::class,
                Function5::class,
                // String — used as constructor param in AltairPowerSyncConnector (powerSyncUrl)
                // and TrackingViewModel (householdId with default value)
                String::class,
            )

        databaseModule.verify(extraTypes = extraTypes)
        preferencesModule.verify(extraTypes = extraTypes)
        networkModule.verify(extraTypes = extraTypes)
        repositoryModule.verify(extraTypes = extraTypes)
        syncModule.verify(extraTypes = extraTypes)
        viewModelModule.verify(extraTypes = extraTypes)
    }
}
