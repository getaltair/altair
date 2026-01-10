package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Component for the Tracking module (Items, Locations, Containers).
 */
interface TrackingComponent {
    // Add state and methods as features are implemented
}

/**
 * Default implementation of TrackingComponent.
 */
class DefaultTrackingComponent(
    componentContext: ComponentContext,
) : TrackingComponent, ComponentContext by componentContext
