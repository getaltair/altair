package com.getaltair.altair

import android.app.Application
import com.getaltair.altair.BuildConfig
import androidx.work.Constraints
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.NetworkType
import androidx.work.PeriodicWorkRequestBuilder
import androidx.work.WorkManager
import com.getaltair.altair.di.databaseModule
import com.getaltair.altair.di.preferencesModule
import com.getaltair.altair.di.repositoryModule
import com.getaltair.altair.di.syncModule
import com.getaltair.altair.di.viewModelModule
import com.getaltair.altair.worker.SyncWorker
import java.util.concurrent.TimeUnit
import org.koin.android.ext.koin.androidContext
import org.koin.android.ext.koin.androidLogger
import org.koin.core.context.startKoin
import timber.log.Timber

class AltairApp : Application() {
    override fun onCreate() {
        super.onCreate()

        // Logging
        if (BuildConfig.DEBUG) {
            Timber.plant(Timber.DebugTree())
        }

        // Dependency injection
        startKoin {
            androidLogger()
            androidContext(this@AltairApp)
            modules(
                databaseModule,
                preferencesModule,
                repositoryModule,
                syncModule,
                viewModelModule,
            )
        }

        // Schedule periodic sync via WorkManager
        scheduleSyncWorker()

        Timber.d("Altair started")
    }

    private fun scheduleSyncWorker() {
        try {
            val constraints = Constraints.Builder()
                .setRequiredNetworkType(NetworkType.CONNECTED)
                .build()

            val syncRequest = PeriodicWorkRequestBuilder<SyncWorker>(
                15, TimeUnit.MINUTES,
            ).setConstraints(constraints).build()

            WorkManager.getInstance(this).enqueueUniquePeriodicWork(
                SyncWorker.WORK_NAME,
                ExistingPeriodicWorkPolicy.KEEP,
                syncRequest,
            )
        } catch (e: Exception) {
            Timber.e(e, "Failed to schedule sync worker")
        }
    }
}
