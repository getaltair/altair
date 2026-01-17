package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.Lifecycle
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume

/**
 * Desktop (JVM) implementation creating ComponentContext with managed lifecycle.
 *
 * The lifecycle starts in RESUMED state. Caller is responsible for managing
 * lifecycle transitions if needed.
 *
 * Example usage:
 * ```kotlin
 * val context = createRootComponentContext()
 * val component = RootComponent(context)
 * // When done:
 * context.getLifecycle().destroy()
 * ```
 */
actual fun createRootComponentContext(): ComponentContext {
    val lifecycle = LifecycleRegistry()
    lifecycle.resume()
    return DefaultComponentContext(lifecycle = lifecycle)
}

/**
 * Gets the LifecycleRegistry from a ComponentContext for lifecycle management.
 * Only works if the context was created via [createRootComponentContext].
 *
 * @throws IllegalStateException if the context is not a DefaultComponentContext
 * @throws IllegalStateException if the lifecycle is not a LifecycleRegistry
 */
fun ComponentContext.getLifecycle(): LifecycleRegistry {
    val defaultContext =
        this as? DefaultComponentContext
            ?: error(
                "getLifecycle() requires DefaultComponentContext, " +
                    "got ${this::class.simpleName}. " +
                    "Only use this with ComponentContext from createRootComponentContext().",
            )

    val lifecycle =
        defaultContext.lifecycle as? LifecycleRegistry
            ?: error(
                "getLifecycle() requires LifecycleRegistry, " +
                    "got ${defaultContext.lifecycle::class.simpleName}. " +
                    "Only use this with ComponentContext from createRootComponentContext().",
            )

    return lifecycle
}

/**
 * Gets the current lifecycle state from a ComponentContext.
 * Convenience method that doesn't require the full LifecycleRegistry.
 */
fun ComponentContext.getLifecycleState(): Lifecycle.State =
    (this as? DefaultComponentContext)?.lifecycle?.state
        ?: error("Cannot get lifecycle state from ${this::class.simpleName}")
