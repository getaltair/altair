package com.getaltair.altair

import android.app.Application
import com.getaltair.altair.di.databaseModule
import com.getaltair.altair.di.repositoryModule
import com.getaltair.altair.di.syncModule
import com.getaltair.altair.di.viewModelModule
import org.koin.android.ext.koin.androidContext
import org.koin.android.ext.koin.androidLogger
import org.koin.core.context.startKoin
import timber.log.Timber

class AltairApp : Application() {
    override fun onCreate() {
        super.onCreate()

        // Logging
        Timber.plant(Timber.DebugTree())

        // Dependency injection
        startKoin {
            androidLogger()
            androidContext(this@AltairApp)
            modules(
                databaseModule,
                repositoryModule,
                syncModule,
                viewModelModule,
            )
        }

        Timber.d("Altair started")
    }
}
