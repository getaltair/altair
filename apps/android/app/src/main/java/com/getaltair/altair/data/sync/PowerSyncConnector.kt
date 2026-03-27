package com.getaltair.altair.data.sync

import timber.log.Timber

class PowerSyncConnector {

    fun connect() {
        Timber.d("PowerSyncConnector: connect() stub -- real wiring deferred to server setup")
    }

    fun disconnect() {
        Timber.d("PowerSyncConnector: disconnect() stub")
    }

    fun isConnected(): Boolean = false
}
