package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.router.stack.ChildStack
import com.arkivanov.decompose.router.stack.StackNavigation
import com.arkivanov.decompose.router.stack.childStack
import com.arkivanov.decompose.router.stack.pop
import com.arkivanov.decompose.router.stack.pushNew
import com.arkivanov.decompose.value.Value
import kotlinx.serialization.Serializable

/**
 * Root component managing top-level navigation for the Altair application.
 *
 * Implements StackNavigation for the three main modules:
 * - Guidance (Quests, Epics, Energy)
 * - Knowledge (Notes, Folders, Tags)
 * - Tracking (Items, Locations, Containers)
 */
interface RootComponent {
    val childStack: Value<ChildStack<*, Child>>

    fun onGuidanceClicked()
    fun onKnowledgeClicked()
    fun onTrackingClicked()
    fun onBackPressed()

    sealed class Child {
        data class Guidance(val component: GuidanceComponent) : Child()
        data class Knowledge(val component: KnowledgeComponent) : Child()
        data class Tracking(val component: TrackingComponent) : Child()
    }
}

/**
 * Default implementation of RootComponent using Decompose's StackNavigation.
 */
class DefaultRootComponent(componentContext: ComponentContext) :
    RootComponent,
    ComponentContext by componentContext {

    private val navigation = StackNavigation<Config>()

    override val childStack: Value<ChildStack<*, RootComponent.Child>> =
        childStack(
            source = navigation,
            serializer = Config.serializer(),
            initialConfiguration = Config.Guidance,
            handleBackButton = true,
            childFactory = ::createChild,
        )

    private fun createChild(config: Config, componentContext: ComponentContext): RootComponent.Child = when (config) {
        Config.Guidance -> RootComponent.Child.Guidance(
            DefaultGuidanceComponent(componentContext),
        )

        Config.Knowledge -> RootComponent.Child.Knowledge(
            DefaultKnowledgeComponent(componentContext),
        )

        Config.Tracking -> RootComponent.Child.Tracking(
            DefaultTrackingComponent(componentContext),
        )
    }

    override fun onGuidanceClicked() {
        navigation.pushNew(Config.Guidance)
    }

    override fun onKnowledgeClicked() {
        navigation.pushNew(Config.Knowledge)
    }

    override fun onTrackingClicked() {
        navigation.pushNew(Config.Tracking)
    }

    override fun onBackPressed() {
        navigation.pop()
    }

    @Serializable
    private sealed class Config {
        @Serializable
        data object Guidance : Config()

        @Serializable
        data object Knowledge : Config()

        @Serializable
        data object Tracking : Config()
    }
}
