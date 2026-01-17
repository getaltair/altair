package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume

/**
 * iOS implementation creating ComponentContext with managed lifecycle.
 * The lifecycle starts in RESUMED state, matching SwiftUI view appearance.
 */
actual fun createRootComponentContext(): ComponentContext {
    val lifecycle = LifecycleRegistry()
    lifecycle.resume()
    return DefaultComponentContext(lifecycle = lifecycle)
}
