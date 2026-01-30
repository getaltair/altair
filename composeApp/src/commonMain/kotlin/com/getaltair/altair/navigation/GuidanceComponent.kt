package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.router.stack.ChildStack
import com.arkivanov.decompose.router.stack.StackNavigation
import com.arkivanov.decompose.router.stack.childStack
import com.arkivanov.decompose.router.stack.pop
import com.arkivanov.decompose.router.stack.push
import com.arkivanov.decompose.value.Value
import com.getaltair.altair.viewmodel.GuidanceViewModel
import kotlinx.serialization.Serializable

/**
 * Guidance module component - task/quest management.
 * Handles WIP=1 quest execution and energy tracking.
 *
 * Manages navigation within the Guidance section:
 * - Quest list view (default)
 * - Quest detail view with checkpoints
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param guidanceViewModel ViewModel for quest operations
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class GuidanceComponent(
    componentContext: ComponentContext,
    private val guidanceViewModel: GuidanceViewModel,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    private val navigation = StackNavigation<Config>()

    /**
     * Navigation stack containing current child component.
     */
    val childStack: Value<ChildStack<Config, Child>> = childStack(
        source = navigation,
        serializer = null, // State persistence disabled for now
        initialConfiguration = Config.List,
        handleBackButton = true,
        childFactory = ::createChild
    )

    private fun createChild(config: Config, componentContext: ComponentContext): Child =
        when (config) {
            is Config.List -> Child.QuestList(
                QuestListComponent(
                    componentContext = componentContext,
                    guidanceViewModel = guidanceViewModel,
                    onQuestClick = ::onQuestSelected
                )
            )
            is Config.Detail -> Child.QuestDetail(
                QuestDetailComponent(
                    componentContext = componentContext,
                    questId = config.questId,
                    guidanceViewModel = guidanceViewModel,
                    onBack = ::onBackFromDetail
                )
            )
        }

    /**
     * Navigate to quest detail screen.
     *
     * @param questId The quest identifier to display
     */
    fun onQuestSelected(questId: String) {
        navigation.push(Config.Detail(questId))
    }

    /**
     * Navigate back from quest detail to quest list.
     */
    fun onBackFromDetail() {
        navigation.pop()
    }

    /**
     * Trigger quest creation flow.
     * Currently delegates to output for handling by parent component.
     */
    fun onCreateQuest() {
        onOutput(Output.NavigateToQuestCreation)
    }

    /**
     * Navigation outputs for the Guidance module.
     */
    sealed interface Output {
        /**
         * Navigate to quest creation screen.
         */
        data object NavigateToQuestCreation : Output
    }

    /**
     * Navigation configuration - defines all possible destinations within Guidance.
     */
    @Serializable
    sealed interface Config {
        /**
         * Quest list screen - shows all quests organized by status.
         */
        @Serializable
        data object List : Config

        /**
         * Quest detail screen - shows single quest with checkpoints.
         *
         * @property questId The quest identifier to display
         */
        @Serializable
        data class Detail(val questId: String) : Config
    }

    /**
     * Child components - the actual screen implementations.
     */
    sealed interface Child {
        /**
         * Quest list screen component.
         */
        data class QuestList(val component: QuestListComponent) : Child

        /**
         * Quest detail screen component.
         */
        data class QuestDetail(val component: QuestDetailComponent) : Child
    }
}
