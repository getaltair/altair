package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Tracking module component - inventory management.
 * Handles items, locations, and containers.
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class TrackingComponent(
    componentContext: ComponentContext,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    /**
     * Navigation outputs for the Tracking module.
     * Will be expanded as we add item details, location navigation, etc.
     */
    sealed interface Output {
        // Future: navigate to item detail, etc.
    }
}
