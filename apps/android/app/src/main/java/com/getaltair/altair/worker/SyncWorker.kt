package com.getaltair.altair.worker

import android.content.Context
import androidx.work.CoroutineWorker
import androidx.work.WorkerParameters
import com.getaltair.altair.data.sync.SyncManager
import org.koin.core.component.KoinComponent
import org.koin.core.component.inject
import timber.log.Timber

class SyncWorker(
    context: Context,
    params: WorkerParameters,
) : CoroutineWorker(context, params), KoinComponent {

    private val syncManager: SyncManager by inject()

    override suspend fun doWork(): Result {
        if (runAttemptCount >= 5) {
            Timber.e("SyncWorker: giving up after $runAttemptCount attempts")
            return Result.failure()
        }
        Timber.d("SyncWorker: starting periodic sync")
        return try {
            syncManager.startSync()
            Result.success()
        } catch (e: Exception) {
            Timber.e(e, "SyncWorker: sync failed")
            Result.retry()
        }
    }

    companion object {
        const val WORK_NAME = "altair_periodic_sync"
    }
}
