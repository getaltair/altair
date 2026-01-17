package com.getaltair.altair.navigation

import kotlinx.serialization.Serializable

/**
 * Navigation destinations for Decompose.
 * Additional destinations (Guidance, Knowledge, Tracking, Settings)
 * will be added as features are implemented.
 */
@Serializable
sealed class Config {
    @Serializable
    data object Home : Config()
}
