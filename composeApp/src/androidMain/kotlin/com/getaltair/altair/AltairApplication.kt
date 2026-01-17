package com.getaltair.altair

import android.app.Application
import android.util.Log
import com.getaltair.altair.di.initKoin
import org.koin.android.ext.koin.androidContext
import org.koin.core.error.KoinApplicationAlreadyStartedException

/**
 * Application class for Altair Android app.
 * Initializes Koin dependency injection once at app startup.
 */
class AltairApplication : Application() {
    /**
     * Tracks whether Koin initialization succeeded.
     * Can be checked by activities to display error UI if needed.
     */
    var initializationError: Throwable? = null
        private set

    override fun onCreate() {
        super.onCreate()
        try {
            initKoin {
                androidContext(this@AltairApplication)
            }
            Log.i(TAG, "Koin initialized successfully")
        } catch (e: KoinApplicationAlreadyStartedException) {
            // Koin already initialized - log warning but continue
            Log.w(TAG, "Koin already initialized", e)
        } catch (e: IllegalStateException) {
            Log.e(TAG, "Failed to initialize Koin", e)
            initializationError = e
            // Re-throw to maintain crash behavior for critical failures
            // The error is logged so it can be diagnosed
            throw e
        }
    }

    companion object {
        private const val TAG = "AltairApplication"
    }
}
