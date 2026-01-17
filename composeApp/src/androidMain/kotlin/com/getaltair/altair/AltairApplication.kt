package com.getaltair.altair

import android.app.Application
import com.getaltair.altair.di.initKoin
import org.koin.android.ext.koin.androidContext

/**
 * Application class for Altair Android app.
 * Initializes Koin dependency injection once at app startup.
 */
class AltairApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        initKoin {
            androidContext(this@AltairApplication)
        }
    }
}
