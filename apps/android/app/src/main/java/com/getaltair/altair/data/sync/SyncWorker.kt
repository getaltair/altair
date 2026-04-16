package com.getaltair.altair.data.sync

import android.content.Context
import android.util.Log
import androidx.work.CoroutineWorker
import androidx.work.WorkerParameters
import kotlinx.coroutines.CancellationException
import org.koin.core.component.KoinComponent
import org.koin.core.component.inject
import java.io.IOException

private const val TAG = "SyncWorker"

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
        } catch (e: CancellationException) {
            throw e
        } catch (e: IOException) {
            Log.w(TAG, "Transient sync error, will retry", e)
            Result.retry()
        } catch (e: Exception) {
            Log.e(TAG, "Permanent sync failure, not retrying", e)
            Result.failure()
        }
}
