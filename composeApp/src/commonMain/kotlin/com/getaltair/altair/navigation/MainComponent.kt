package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/**
 * Main component that manages the navigation shell state.
 *
 * This component holds the currently selected tab destination and provides
 * methods to navigate between tabs.
 *
 * @param componentContext The Decompose component context.
 * @param initialDestination The initial destination to show. Defaults to [MainDestination.Home].
 */
class MainComponent(
    componentContext: ComponentContext,
    initialDestination: MainDestination = MainDestination.Home,
) : ComponentContext by componentContext {
    private val _currentDestination = MutableStateFlow(initialDestination)

    /**
     * The currently selected navigation destination.
     */
    val currentDestination: StateFlow<MainDestination> = _currentDestination.asStateFlow()

    /**
     * Navigate to the specified destination.
     *
     * @param destination The destination to navigate to.
     */
    fun navigateTo(destination: MainDestination) {
        _currentDestination.value = destination
    }
}
