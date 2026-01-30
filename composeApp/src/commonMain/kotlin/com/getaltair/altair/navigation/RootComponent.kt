package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.DelicateDecomposeApi
import com.arkivanov.decompose.router.stack.ChildStack
import com.arkivanov.decompose.router.stack.StackNavigation
import com.arkivanov.decompose.router.stack.childStack
import com.arkivanov.decompose.router.stack.pop
import com.arkivanov.decompose.router.stack.push
import com.arkivanov.decompose.value.Value
import kotlinx.serialization.Serializable

/**
 * Root navigation component for Altair.
 * Manages the main navigation stack across all platforms.
 */
class RootComponent(
    componentContext: ComponentContext
) : ComponentContext by componentContext {

    private val navigation = StackNavigation<Config>()

    val stack: Value<ChildStack<Config, Child>> = childStack(
        source = navigation,
        serializer = null, // State persistence disabled for now - will enable in future phase
        initialConfiguration = Config.Home,
        handleBackButton = true,
        childFactory = ::createChild
    )

    private fun createChild(config: Config, componentContext: ComponentContext): Child =
        when (config) {
            is Config.Home -> Child.Home(HomeComponent(componentContext, ::onHomeOutput))
            is Config.Settings -> Child.Settings(SettingsComponent(componentContext, ::onSettingsOutput))
            is Config.Guidance -> Child.Guidance(GuidanceComponent(componentContext, ::onGuidanceOutput))
            is Config.Knowledge -> Child.Knowledge(KnowledgeComponent(componentContext, ::onKnowledgeOutput))
            is Config.Tracking -> Child.Tracking(TrackingComponent(componentContext, ::onTrackingOutput))
        }

    @OptIn(DelicateDecomposeApi::class)
    private fun onHomeOutput(output: HomeComponent.Output) {
        when (output) {
            HomeComponent.Output.NavigateToSettings -> navigation.push(Config.Settings)
        }
    }

    @OptIn(DelicateDecomposeApi::class)
    private fun onSettingsOutput(output: SettingsComponent.Output) {
        when (output) {
            SettingsComponent.Output.NavigateBack -> navigation.pop()
        }
    }

    private fun onGuidanceOutput(output: GuidanceComponent.Output) {
        // Handle when needed
    }

    private fun onKnowledgeOutput(output: KnowledgeComponent.Output) {
        // Handle when needed
    }

    private fun onTrackingOutput(output: TrackingComponent.Output) {
        // Handle when needed
    }

    /**
     * Navigate to a specific destination.
     * Used by bottom navigation bar to switch between main sections.
     * Note: Currently uses stack navigation. Will migrate to TabNavigation in future phase.
     */
    @OptIn(DelicateDecomposeApi::class)
    fun navigateTo(config: Config) {
        navigation.push(config)
    }

    /**
     * Navigation configuration - defines all possible destinations.
     * Serializable for state restoration.
     */
    @Serializable
    sealed interface Config {
        @Serializable
        data object Home : Config

        @Serializable
        data object Settings : Config

        @Serializable
        data object Guidance : Config

        @Serializable
        data object Knowledge : Config

        @Serializable
        data object Tracking : Config
    }

    /**
     * Child components - the actual screen implementations.
     */
    sealed interface Child {
        data class Home(val component: HomeComponent) : Child
        data class Settings(val component: SettingsComponent) : Child
        data class Guidance(val component: GuidanceComponent) : Child
        data class Knowledge(val component: KnowledgeComponent) : Child
        data class Tracking(val component: TrackingComponent) : Child
    }
}
