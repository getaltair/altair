package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.viewmodel.GuidanceViewModel

/**
 * Quest list component - displays all quests organized by status.
 *
 * Shows quests grouped by status:
 * - ACTIVE: Currently in-progress quest (WIP=1, max 1 item)
 * - BACKLOG: Ready to start
 * - COMPLETED: Finished quests
 * - ABANDONED: Discarded quests
 *
 * Supports quest management operations:
 * - View all quests
 * - Filter by status
 * - Click to view detail
 * - Quick actions (start, complete, abandon)
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param guidanceViewModel ViewModel managing quest operations
 * @param onQuestClick Callback for quest item clicks, navigates to detail view
 */
class QuestListComponent(
    componentContext: ComponentContext,
    val guidanceViewModel: GuidanceViewModel,
    private val onQuestClick: (String) -> Unit
) : ComponentContext by componentContext {

    /**
     * Initialize component - load all quests.
     */
    init {
        guidanceViewModel.loadQuests()
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
     * Filter quests by status.
     *
     * @param status The quest status to filter by (null for all quests)
     */
    fun filterByStatus(status: QuestStatus?) {
        guidanceViewModel.loadQuests(status)
    }

    /**
     * Start a quest (transition to ACTIVE status).
     *
     * WIP=1 enforcement: Only one quest can be active at a time.
     *
     * @param questId The quest identifier to start
     */
    fun onStartQuest(questId: String) {
        guidanceViewModel.startQuest(questId)
    }

    /**
     * Complete a quest (transition to COMPLETED status).
     *
     * @param questId The quest identifier to complete
     */
    fun onCompleteQuest(questId: String) {
        guidanceViewModel.completeQuest(questId)
    }

    /**
     * Abandon a quest (transition to ABANDONED status).
     *
     * @param questId The quest identifier to abandon
     */
    fun onAbandonQuest(questId: String) {
        guidanceViewModel.abandonQuest(questId)
    }

    /**
     * Return a quest to backlog.
     *
     * @param questId The quest identifier to backlog
     */
    fun onBacklogQuest(questId: String) {
        guidanceViewModel.backlogQuest(questId)
    }

    /**
     * Open quest creation screen.
     *
     * Future: will navigate to creation screen or open creation dialog.
     */
    fun onCreateQuest() {
        // Future: open creation screen or dialog
        // For now, this is a placeholder for UI implementation
    }

    /**
     * Refresh quest list.
     */
    fun refresh() {
        guidanceViewModel.loadQuests()
    }
}
