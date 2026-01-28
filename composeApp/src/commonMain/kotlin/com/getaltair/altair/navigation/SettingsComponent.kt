package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Settings screen component.
 * Placeholder for Phase 1 - will expand with actual settings in later phases.
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class SettingsComponent(
    componentContext: ComponentContext,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    fun onBackClicked() {
        onOutput(Output.NavigateBack)
    }

    sealed interface Output {
        data object NavigateBack : Output
    }
}
