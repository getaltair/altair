package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Component for the Guidance module (Quests, Epics, Checkpoints, Energy).
 */
interface GuidanceComponent {
    // Add state and methods as features are implemented
}

/**
 * Default implementation of GuidanceComponent.
 */
class DefaultGuidanceComponent(
    componentContext: ComponentContext,
) : GuidanceComponent, ComponentContext by componentContext
