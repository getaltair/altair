package com.getaltair.altair.navigation

import kotlinx.serialization.Serializable

/**
 * Navigation destinations for Decompose.
 *
 * The navigation is structured in two levels:
 * 1. Root level: Auth flow (Login/Register) vs Main app shell
 * 2. Shell level: Tab navigation between modules (see [MainDestination])
 */
@Serializable
sealed class Config {
    /**
     * Main application shell containing tab navigation.
     * Shown when the user is authenticated.
     */
    @Serializable
    data object Main : Config()

    @Serializable
    data object Login : Config()

    @Serializable
    data object Register : Config()
}

/**
 * Tab destinations within the main application shell.
 *
 * These represent the primary navigation destinations accessible
 * from the bottom bar (mobile) or side rail (desktop).
 */
enum class MainDestination {
    /** Today view - daily overview and quick actions. */
    Home,

    /** Guidance module - quests and goals. */
    Guidance,

    /** Knowledge module - notes and information. */
    Knowledge,

    /** Tracking module - metrics and habits. */
    Tracking,

    /** User settings and preferences. */
    Settings,
}
