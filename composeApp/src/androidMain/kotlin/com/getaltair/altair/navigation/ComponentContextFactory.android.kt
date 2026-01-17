package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Android implementation using Activity's defaultComponentContext.
 * Must be called from Activity context.
 */
actual fun createRootComponentContext(): ComponentContext =
    throw IllegalStateException(
        "createRootComponentContext() should not be called on Android. " +
            "Use defaultComponentContext() from Activity instead.",
    )
