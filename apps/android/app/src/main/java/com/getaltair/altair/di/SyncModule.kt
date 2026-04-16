package com.getaltair.altair.di

import co.touchlab.kermit.Logger
import com.getaltair.altair.BuildConfig
import com.getaltair.altair.data.sync.AltairPowerSyncConnector
import com.getaltair.altair.data.sync.AltairPowerSyncSchema
import com.getaltair.altair.data.sync.SyncCoordinator
import com.powersync.PowerSyncDatabase
import com.powersync.integrations.room.RoomConnectionPool
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import org.koin.dsl.module

val syncModule =
    module {
        single<PowerSyncDatabase> {
            val pool = get<RoomConnectionPool>()
            PowerSyncDatabase.PowerSyncOpenFactory.opened(
                pool,
                CoroutineScope(SupervisorJob()),
                AltairPowerSyncSchema,
                "altair.db",
                Logger.withTag("PowerSync"),
            )
        }

        single<AltairPowerSyncConnector> {
            AltairPowerSyncConnector(
                powerSyncUrl = BuildConfig.POWERSYNC_URL,
                getToken = { get<com.getaltair.altair.data.auth.TokenPreferences>().accessToken ?: "" },
                uploadToServer = { _, _, _, _ -> /* wired in S016 (WorkManager sync) */ },
            )
        }

        single<SyncCoordinator> {
            SyncCoordinator(
                powerSyncDatabase = get(),
                connector = get(),
            )
        }
    }
