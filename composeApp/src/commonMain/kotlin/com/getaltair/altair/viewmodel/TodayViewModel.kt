package com.getaltair.altair.viewmodel

import com.getaltair.altair.api.GuidanceApi
import com.getaltair.altair.shared.dto.guidance.TodayViewResponse
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * ViewModel for Today view screen.
 *
 * Manages state and operations for the Today view, which displays:
 * - Energy budget (available, spent, remaining)
 * - Active quest (WIP=1 enforcement - only one at a time)
 * - Ready quests (available to start)
 * - Routine instances (due today)
 * - Completed quests (finished today)
 *
 * Provides the primary workflow for ADHD-aware daily task execution:
 * 1. Check energy budget
 * 2. Start a quest (if no active quest)
 * 3. Work on active quest
 * 4. Complete quest
 * 5. Repeat
 *
 * @property guidanceApi API client for guidance operations
 * @property scope Coroutine scope for launching async operations
 */
class TodayViewModel(
    private val guidanceApi: GuidanceApi,
    private val scope: CoroutineScope
) {
    // === State ===

    private val _todayView = MutableStateFlow<UiState<TodayViewResponse>>(UiState.Loading)
    val todayView: StateFlow<UiState<TodayViewResponse>> = _todayView.asStateFlow()

    // === Actions ===

    /**
     * Load today view data.
     *
     * Fetches comprehensive view of current day including:
     * - Energy budget
     * - Active quest
     * - Ready quests
     * - Routine instances
     * - Completed quests
     */
    fun loadTodayView() {
        scope.launch {
            _todayView.value = UiState.Loading
            guidanceApi.getTodayView().fold(
                ifLeft = { error ->
                    _todayView.value = UiState.Error(error.toString())
                },
                ifRight = { todayData ->
                    _todayView.value = UiState.Success(todayData)
                }
            )
        }
    }

    /**
     * Start a quest (transition to ACTIVE status).
     *
     * WIP=1 enforcement: Only one quest can be active at a time.
     * If another quest is already active, this will fail with conflict error.
     *
     * @param questId The quest identifier to start
     */
    fun startQuest(questId: String) {
        scope.launch {
            guidanceApi.startQuest(questId).fold(
                ifLeft = { error ->
                    _todayView.value = UiState.Error("Failed to start quest: $error")
                    // Restore previous state after showing error
                    val currentData = (_todayView.value as? UiState.Success)?.data
                    if (currentData != null) {
                        _todayView.value = UiState.Success(currentData)
                    }
                },
                ifRight = { startedQuest ->
                    // Reload today view to reflect new active quest
                    loadTodayView()
                }
            )
        }
    }

    /**
     * Complete a quest (transition to COMPLETED status).
     *
     * Adds energy cost to today's budget spent.
     * Frees up the WIP slot for another quest.
     *
     * @param questId The quest identifier to complete
     */
    fun completeQuest(questId: String) {
        scope.launch {
            guidanceApi.completeQuest(questId).fold(
                ifLeft = { error ->
                    _todayView.value = UiState.Error("Failed to complete quest: $error")
                    // Restore previous state after showing error
                    val currentData = (_todayView.value as? UiState.Success)?.data
                    if (currentData != null) {
                        _todayView.value = UiState.Success(currentData)
                    }
                },
                ifRight = { completedQuest ->
                    // Reload today view to update energy budget and quest lists
                    loadTodayView()
                }
            )
        }
    }

    /**
     * Abandon a quest (transition to ABANDONED status).
     *
     * Use when quest is no longer relevant or should not be pursued.
     * Frees up the WIP slot if this was the active quest.
     *
     * @param questId The quest identifier to abandon
     */
    fun abandonQuest(questId: String) {
        scope.launch {
            guidanceApi.abandonQuest(questId).fold(
                ifLeft = { error ->
                    _todayView.value = UiState.Error("Failed to abandon quest: $error")
                    // Restore previous state after showing error
                    val currentData = (_todayView.value as? UiState.Success)?.data
                    if (currentData != null) {
                        _todayView.value = UiState.Success(currentData)
                    }
                },
                ifRight = { abandonedQuest ->
                    // Reload today view to update quest lists
                    loadTodayView()
                }
            )
        }
    }

    /**
     * Return a quest to backlog (transition to BACKLOG status).
     *
     * Frees up the WIP slot for another quest.
     *
     * @param questId The quest identifier to backlog
     */
    fun backlogQuest(questId: String) {
        scope.launch {
            guidanceApi.backlogQuest(questId).fold(
                ifLeft = { error ->
                    _todayView.value = UiState.Error("Failed to backlog quest: $error")
                    // Restore previous state after showing error
                    val currentData = (_todayView.value as? UiState.Success)?.data
                    if (currentData != null) {
                        _todayView.value = UiState.Success(currentData)
                    }
                },
                ifRight = { backloggedQuest ->
                    // Reload today view to update quest lists
                    loadTodayView()
                }
            )
        }
    }
}
