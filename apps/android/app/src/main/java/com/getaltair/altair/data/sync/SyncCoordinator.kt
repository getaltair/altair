package com.getaltair.altair.data.sync

import android.content.Context
import android.util.Log
import androidx.work.Constraints
import androidx.work.ExistingWorkPolicy
import androidx.work.NetworkType
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.OutOfQuotaPolicy
import androidx.work.WorkManager
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

private const val TAG = "SyncCoordinator"
private const val SYNC_DEBOUNCE_WINDOW_MS = 5 * 60 * 1_000L

class SyncCoordinator(
    private val powerSyncDatabase: PowerSyncDatabase,
    private val connector: AltairPowerSyncConnector,
    private val clock: () -> Long = System::currentTimeMillis,
    private val workManagerProvider: (Context) -> WorkManager = WorkManager::getInstance,
) {
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private var syncJob: Job? = null
    private var lastSyncTime: Long = 0L

    fun startSync() {
        syncJob?.cancel()
        syncJob =
            scope.launch {
                try {
                    powerSyncDatabase.connect(connector)
                } catch (e: CancellationException) {
                    throw e
                } catch (e: Exception) {
                    Log.e(TAG, "PowerSync connect failed", e)
                }
            }
    }

    fun stopSync() {
        syncJob?.cancel()
        syncJob = null
        scope.launch {
            try {
                powerSyncDatabase.disconnect()
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "PowerSync disconnect failed", e)
            }
        }
    }

    suspend fun triggerSync() {
        powerSyncDatabase.connect(connector)
    }

    fun enqueueExpedited(context: Context) {
        if (clock() - lastSyncTime < SYNC_DEBOUNCE_WINDOW_MS) return
        workManagerProvider(context).enqueueUniqueWork(
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
        lastSyncTime = clock()
    }
}
