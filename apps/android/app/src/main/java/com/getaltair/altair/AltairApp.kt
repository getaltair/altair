package com.getaltair.altair

import android.app.Application
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
            modules(emptyList()) // Add modules as you build features
        }

        Timber.d("Altair started")
    }
}
