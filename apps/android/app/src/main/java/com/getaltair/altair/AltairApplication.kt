package com.getaltair.altair

import android.app.Application
import android.app.NotificationChannel
import android.app.NotificationManager
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.util.Log
import androidx.work.Constraints
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.NetworkType
import androidx.work.PeriodicWorkRequestBuilder
import androidx.work.WorkManager
import com.getaltair.altair.data.sync.SyncCoordinator
import com.getaltair.altair.data.sync.SyncWorker
import com.getaltair.altair.di.databaseModule
import com.getaltair.altair.di.networkModule
import com.getaltair.altair.di.preferencesModule
import com.getaltair.altair.di.repositoryModule
import com.getaltair.altair.di.syncModule
import com.getaltair.altair.di.viewModelModule
import org.koin.android.ext.android.get
import org.koin.android.ext.koin.androidContext
import org.koin.core.context.startKoin
import java.util.concurrent.TimeUnit

private const val TAG = "AltairApplication"

class AltairApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        try {
            startKoin {
                androidContext(this@AltairApplication)
                modules(
                    databaseModule,
                    syncModule,
                    preferencesModule,
                    networkModule,
                    repositoryModule,
                    viewModelModule,
                )
            }
        } catch (e: Exception) {
            Log.e(TAG, "Koin initialization failed", e)
            return
        }

        try {
            WorkManager.getInstance(this).enqueueUniquePeriodicWork(
                "sync_periodic",
                ExistingPeriodicWorkPolicy.KEEP,
                PeriodicWorkRequestBuilder<SyncWorker>(15, TimeUnit.MINUTES)
                    .setConstraints(
                        Constraints
                            .Builder()
                            .setRequiredNetworkType(NetworkType.CONNECTED)
                            .build(),
                    ).addTag("sync_periodic")
                    .build(),
            )
        } catch (e: Exception) {
            Log.e(TAG, "Failed to schedule periodic sync", e)
        }

        NotificationChannel(
            "FOCUS_TIMER_CHANNEL",
            "Focus Timer",
            NotificationManager.IMPORTANCE_LOW,
        ).also { channel ->
            getSystemService(NotificationManager::class.java).createNotificationChannel(channel)
        }

        val connectivityManager = getSystemService(CONNECTIVITY_SERVICE) as ConnectivityManager
        val request =
            NetworkRequest
                .Builder()
                .addCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
                .build()
        connectivityManager.registerNetworkCallback(
            request,
            object : ConnectivityManager.NetworkCallback() {
                override fun onAvailable(network: Network) {
                    try {
                        get<SyncCoordinator>().enqueueExpedited(this@AltairApplication)
                    } catch (e: Exception) {
                        Log.e(TAG, "Failed to enqueue expedited sync on network available", e)
                    }
                }
            },
        )
    }
}
