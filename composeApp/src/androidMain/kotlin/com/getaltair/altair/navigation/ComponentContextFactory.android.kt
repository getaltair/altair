package com.getaltair.altair.navigation

import android.util.Log
import com.arkivanov.decompose.ComponentContext

private const val TAG = "ComponentContextFactory"

/**
 * Android implementation that prevents direct use of createRootComponentContext().
 *
 * On Android, use `defaultComponentContext()` from ComponentActivity instead,
 * which properly integrates with the Activity lifecycle and handles configuration changes.
 *
 * @throws IllegalStateException always - this function should not be called on Android
 */
actual fun createRootComponentContext(): ComponentContext {
    val message =
        "createRootComponentContext() should not be called on Android. " +
            "Use defaultComponentContext() from Activity instead."
    Log.e(TAG, message)
    throw IllegalStateException(message)
}
