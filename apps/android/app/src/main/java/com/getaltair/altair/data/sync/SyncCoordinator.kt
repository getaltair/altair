package com.getaltair.altair.data.sync

import android.content.Context
import androidx.work.Constraints
import androidx.work.ExistingWorkPolicy
import androidx.work.NetworkType
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.OutOfQuotaPolicy
import androidx.work.WorkManager
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

class SyncCoordinator(
    private val powerSyncDatabase: PowerSyncDatabase,
    private val connector: AltairPowerSyncConnector,
) {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private var syncJob: Job? = null

    fun startSync() {
        syncJob?.cancel()
        syncJob =
            scope.launch {
                powerSyncDatabase.connect(connector)
            }
    }

    fun stopSync() {
        syncJob?.cancel()
        syncJob = null
        scope.launch {
            powerSyncDatabase.disconnect()
        }
    }

    suspend fun triggerSync() {
        powerSyncDatabase.connect(connector)
    }

    fun enqueueExpedited(context: Context) {
        WorkManager.getInstance(context).enqueueUniqueWork(
            "sync_expedited",
            ExistingWorkPolicy.REPLACE,
            OneTimeWorkRequestBuilder<SyncWorker>()
                .setExpedited(OutOfQuotaPolicy.RUN_AS_NON_EXPEDITED_WORK_REQUEST)
                .setConstraints(
                    Constraints
                        .Builder()
                        .setRequiredNetworkType(NetworkType.CONNECTED)
                        .build(),
                ).addTag("sync_expedited")
                .build(),
        )
    }
}
