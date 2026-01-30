package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.value.MutableValue
import com.arkivanov.decompose.value.Value
import com.getaltair.altair.viewmodel.InboxViewModel
import com.getaltair.altair.viewmodel.TodayViewModel

/**
 * Home screen component - main entry point of the app.
 *
 * Displays two-tab layout:
 * - Inbox: Universal Inbox for capturing and triaging items
 * - Today: Today view with active quest, energy budget, and ready quests
 *
 * This is the primary landing screen for daily workflow.
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param inboxViewModel ViewModel for Inbox operations
 * @param todayViewModel ViewModel for Today view operations
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class HomeComponent(
    componentContext: ComponentContext,
    inboxViewModel: InboxViewModel,
    todayViewModel: TodayViewModel,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    private val _selectedTab = MutableValue(HomeTab.INBOX)

    /**
     * Currently selected tab in the Home screen.
     */
    val selectedTab: Value<HomeTab> = _selectedTab

    /**
     * Inbox component - handles Universal Inbox capture and triage.
     */
    val inboxComponent: InboxComponent = InboxComponent(
        componentContext = componentContext,
        inboxViewModel = inboxViewModel
    )

    /**
     * Today component - handles Today view with active quest and energy.
     */
    val todayComponent: TodayComponent = TodayComponent(
        componentContext = componentContext,
        todayViewModel = todayViewModel,
        onQuestClick = { questId -> onOutput(Output.NavigateToQuestDetail(questId)) }
    )

    /**
     * Switch between Inbox and Today tabs.
     *
     * @param tab The tab to select
     */
    fun selectTab(tab: HomeTab) {
        _selectedTab.value = tab
    }

    /**
     * Navigate to settings screen.
     */
    fun onSettingsClicked() {
        onOutput(Output.NavigateToSettings)
    }

    /**
     * Home screen tabs.
     */
    enum class HomeTab {
        /**
         * Universal Inbox - capture and triage items.
         */
        INBOX,

        /**
         * Today view - active quest, energy budget, ready quests.
         */
        TODAY
    }

    /**
     * Navigation outputs for the Home screen.
     */
    sealed interface Output {
        /**
         * Navigate to settings screen.
         */
        data object NavigateToSettings : Output

        /**
         * Navigate to quest detail screen.
         *
         * @property questId The quest identifier to display
         */
        data class NavigateToQuestDetail(val questId: String) : Output
    }
}
