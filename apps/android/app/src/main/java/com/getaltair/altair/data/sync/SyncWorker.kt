package com.getaltair.altair.data.sync

import android.content.Context
import androidx.work.CoroutineWorker
import androidx.work.WorkerParameters
import org.koin.core.component.KoinComponent
import org.koin.core.component.inject

class SyncWorker(
    context: Context,
    params: WorkerParameters,
) : CoroutineWorker(context, params),
    KoinComponent {
    private val syncCoordinator: SyncCoordinator by inject()

    override suspend fun doWork(): Result =
        try {
            syncCoordinator.triggerSync()
            Result.success()
        } catch (e: Exception) {
            Result.retry()
        }
}
