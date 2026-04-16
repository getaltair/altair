package com.getaltair.altair.data.sync

import com.getaltair.altair.data.network.SyncApi
import com.getaltair.altair.data.network.UpsertRequest
import com.powersync.PowerSyncDatabase
import com.powersync.connectors.PowerSyncBackendConnector
import com.powersync.connectors.PowerSyncCredentials
import com.powersync.db.crud.UpdateType

class AltairPowerSyncConnector(
    private val powerSyncUrl: String,
    private val getToken: suspend () -> String,
    private val syncApi: SyncApi,
) : PowerSyncBackendConnector() {
    override suspend fun fetchCredentials(): PowerSyncCredentials =
        PowerSyncCredentials(
            endpoint = powerSyncUrl,
            token = getToken(),
        )

    override suspend fun uploadData(database: PowerSyncDatabase) {
        val batch = database.getCrudBatch(100) ?: return
        for (op in batch.crud) {
            when (op.op) {
                UpdateType.PUT, UpdateType.PATCH -> syncApi.upsert(op.table, op.id, UpsertRequest(op.opData ?: emptyMap()))
                UpdateType.DELETE -> syncApi.delete(op.table, op.id)
            }
        }
        batch.complete(null)
    }
}
