package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume

/**
 * Desktop (JVM) implementation creating ComponentContext with managed lifecycle.
 * Caller is responsible for calling lifecycle.destroy() when done.
 */
actual fun createRootComponentContext(): ComponentContext {
    val lifecycle = LifecycleRegistry()
    lifecycle.resume()
    return DefaultComponentContext(lifecycle = lifecycle)
}

/**
 * Gets the LifecycleRegistry from a ComponentContext for cleanup.
 * Only works if the context was created via createRootComponentContext().
 */
fun ComponentContext.getLifecycle(): LifecycleRegistry =
    requireNotNull((this as? DefaultComponentContext)?.lifecycle as? LifecycleRegistry) {
        "getLifecycle() only works with ComponentContext from createRootComponentContext()"
    }
