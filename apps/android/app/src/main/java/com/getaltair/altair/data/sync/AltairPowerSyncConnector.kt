package com.getaltair.altair.data.sync

import com.powersync.PowerSyncDatabase
import com.powersync.connectors.PowerSyncBackendConnector
import com.powersync.connectors.PowerSyncCredentials
import com.powersync.db.crud.UpdateType

class AltairPowerSyncConnector(
    private val powerSyncUrl: String,
    private val getToken: suspend () -> String,
    private val uploadToServer: suspend (table: String, id: String, data: Map<String, String?>, isDelete: Boolean) -> Unit,
) : PowerSyncBackendConnector() {
    override suspend fun fetchCredentials(): PowerSyncCredentials =
        PowerSyncCredentials(
            endpoint = powerSyncUrl,
            token = getToken(),
        )

    override suspend fun uploadData(database: PowerSyncDatabase) {
        val batch = database.getCrudBatch(100) ?: return
        try {
            for (op in batch.crud) {
                when (op.op) {
                    UpdateType.PUT, UpdateType.PATCH -> uploadToServer(op.table, op.id, op.opData ?: emptyMap(), false)
                    UpdateType.DELETE -> uploadToServer(op.table, op.id, emptyMap(), true)
                }
            }
            batch.complete(null)
        } catch (e: Exception) {
            throw e
        }
    }
}
