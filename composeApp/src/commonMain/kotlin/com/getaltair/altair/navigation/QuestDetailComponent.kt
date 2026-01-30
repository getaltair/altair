package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.getaltair.altair.viewmodel.GuidanceViewModel

/**
 * Quest detail component - displays single quest with full details.
 *
 * Shows:
 * - Quest metadata (title, description, energy cost, status)
 * - Checkpoints list (with completion tracking)
 * - Actions (start, complete, abandon, edit)
 *
 * Supports quest execution workflow:
 * - View quest details and checkpoints
 * - Mark checkpoints as complete
 * - Start/complete/abandon quest
 * - Edit quest details
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param questId The quest identifier to display
 * @param guidanceViewModel ViewModel managing quest operations
 * @param onBack Callback to navigate back to quest list
 */
class QuestDetailComponent(
    componentContext: ComponentContext,
    val questId: String,
    val guidanceViewModel: GuidanceViewModel,
    private val onBack: () -> Unit
) : ComponentContext by componentContext {

    /**
     * Initialize component - load quest details.
     */
    init {
        guidanceViewModel.loadQuestDetail(questId)
    }

    /**
     * Navigate back to quest list.
     */
    fun onBack() {
        onBack.invoke()
    }

    /**
     * Open quest edit screen.
     *
     * Future: will navigate to edit screen or open edit dialog.
     */
    fun onEdit() {
        // Future: open edit screen or dialog
        // For now, this is a placeholder for UI implementation
    }

    /**
     * Start this quest (transition to ACTIVE status).
     *
     * WIP=1 enforcement: Only one quest can be active at a time.
     */
    fun onStart() {
        guidanceViewModel.startQuest(questId)
    }

    /**
     * Complete this quest (transition to COMPLETED status).
     *
     * Adds energy cost to today's budget spent.
     */
    fun onComplete() {
        guidanceViewModel.completeQuest(questId)
    }

    /**
     * Abandon this quest (transition to ABANDONED status).
     *
     * Use when quest is no longer relevant or should not be pursued.
     */
    fun onAbandon() {
        guidanceViewModel.abandonQuest(questId)
    }

    /**
     * Return this quest to backlog.
     *
     * Transitions from ACTIVE back to BACKLOG, freeing the WIP slot.
     */
    fun onBacklog() {
        guidanceViewModel.backlogQuest(questId)
    }

    /**
     * Toggle checkpoint completion status.
     *
     * @param checkpointId The checkpoint identifier
     * @param completed New completion status
     */
    fun onCheckpointToggle(checkpointId: String, completed: Boolean) {
        guidanceViewModel.toggleCheckpoint(checkpointId, completed)
    }

    /**
     * Add a new checkpoint to this quest.
     *
     * @param title Checkpoint description
     */
    fun onAddCheckpoint(title: String) {
        guidanceViewModel.addCheckpoint(questId, title)
    }

    /**
     * Delete a checkpoint.
     *
     * @param checkpointId The checkpoint identifier
     */
    fun onDeleteCheckpoint(checkpointId: String) {
        guidanceViewModel.deleteCheckpoint(checkpointId)
    }
}
