package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Home screen component - main entry point of the app.
 * Will contain Guidance module entry in future phases.
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class HomeComponent(
    componentContext: ComponentContext,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    fun onSettingsClicked() {
        onOutput(Output.NavigateToSettings)
    }

    sealed interface Output {
        data object NavigateToSettings : Output
    }
}
