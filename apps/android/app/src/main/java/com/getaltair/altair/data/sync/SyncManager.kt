package com.getaltair.altair.data.sync

import timber.log.Timber

class SyncManager(
    private val connector: PowerSyncConnector,
) {

    fun startSync() {
        Timber.d("SyncManager: startSync() -- connecting PowerSync")
        connector.connect()
    }

    fun stopSync() {
        Timber.d("SyncManager: stopSync() -- disconnecting PowerSync")
        connector.disconnect()
    }

    fun isSyncing(): Boolean = connector.isConnected()
}
