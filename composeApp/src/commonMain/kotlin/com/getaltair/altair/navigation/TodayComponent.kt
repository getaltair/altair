package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.getaltair.altair.viewmodel.TodayViewModel

/**
 * Today component - handles Today view display and interactions.
 *
 * Displays comprehensive view of current day:
 * - Energy budget (available, spent, remaining)
 * - Active quest (WIP=1 enforcement)
 * - Ready quests (can be started)
 * - Routine instances (due today)
 * - Completed quests (finished today)
 *
 * Provides primary workflow for ADHD-aware daily task execution:
 * 1. Check energy budget
 * 2. Start a quest (if no active quest)
 * 3. Work on active quest
 * 4. Complete quest
 * 5. Repeat
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param todayViewModel ViewModel managing Today view state and operations
 * @param onQuestClick Callback for quest item clicks, navigates to detail view
 */
class TodayComponent(
    componentContext: ComponentContext,
    val todayViewModel: TodayViewModel,
    private val onQuestClick: (String) -> Unit
) : ComponentContext by componentContext {

    /**
     * Initialize component - load today view data.
     */
    init {
        todayViewModel.loadTodayView()
    }

    /**
     * Handle click on a quest item.
     * Navigates to quest detail view.
     *
     * @param questId The quest identifier
     */
    fun onQuestClick(questId: String) {
        onQuestClick.invoke(questId)
    }

    /**
     * Start a quest (transition to ACTIVE status).
     *
     * WIP=1 enforcement: Only one quest can be active at a time.
     * If another quest is already active, this will fail with conflict error.
     *
     * @param questId The quest identifier to start
     */
    fun onStartQuest(questId: String) {
        todayViewModel.startQuest(questId)
    }

    /**
     * Complete the active quest.
     *
     * Transitions quest to COMPLETED status and adds energy cost to budget spent.
     *
     * @param questId The quest identifier to complete
     */
    fun onCompleteQuest(questId: String) {
        todayViewModel.completeQuest(questId)
    }

    /**
     * Return a quest to backlog.
     *
     * Transitions quest from ACTIVE back to BACKLOG, freeing the WIP slot.
     *
     * @param questId The quest identifier to backlog
     */
    fun onBacklogQuest(questId: String) {
        todayViewModel.backlogQuest(questId)
    }

    /**
     * Refresh today view data.
     *
     * Reloads energy budget, active quest, ready quests, and completed quests.
     */
    fun refresh() {
        todayViewModel.loadTodayView()
    }
}
