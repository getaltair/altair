package com.getaltair.altair.di

import com.getaltair.altair.data.sync.PowerSyncConnector
import com.getaltair.altair.data.sync.SyncManager
import org.koin.dsl.module

val syncModule = module {

    single { PowerSyncConnector() }

    single { SyncManager(connector = get()) }
}
