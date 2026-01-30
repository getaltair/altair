package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Guidance module component - task/quest management.
 * Handles WIP=1 quest execution and energy tracking.
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class GuidanceComponent(
    componentContext: ComponentContext,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    /**
     * Navigation outputs for the Guidance module.
     * Will be expanded as we add quest details, routine management, etc.
     */
    sealed interface Output {
        // Future: navigate to quest detail, etc.
    }
}
