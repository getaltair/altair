package com.getaltair.altair

import androidx.compose.ui.window.ComposeUIViewController
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.getaltair.altair.di.initKoinIos
import com.getaltair.altair.navigation.RootComponent
import platform.UIKit.UIViewController

fun MainViewController(): UIViewController {
    // Initialize Koin for iOS
    initKoinIos()

    // Create lifecycle and root component
    val lifecycle = LifecycleRegistry()
    val rootComponent = RootComponent(
        componentContext = DefaultComponentContext(lifecycle = lifecycle)
    )

    return ComposeUIViewController { App(rootComponent) }
}