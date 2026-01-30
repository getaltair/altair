package com.getaltair.altair.viewmodel

import com.getaltair.altair.api.GuidanceApi
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.dto.guidance.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * ViewModel for Guidance module screens.
 *
 * Manages state and operations for:
 * - Quest list (all quests, filtered by status)
 * - Quest detail (single quest with checkpoints)
 * - Quest actions (start, complete, abandon, backlog)
 * - Checkpoint operations (add, toggle, delete, reorder)
 *
 * Provides centralized state management for quest-related UI components.
 *
 * @property guidanceApi API client for guidance operations
 * @property scope Coroutine scope for launching async operations
 */
class GuidanceViewModel(
    private val guidanceApi: GuidanceApi,
    private val scope: CoroutineScope
) {
    // === Quest List State ===

    private val _quests = MutableStateFlow<UiState<List<QuestSummaryResponse>>>(UiState.Loading)
    val quests: StateFlow<UiState<List<QuestSummaryResponse>>> = _quests.asStateFlow()

    private val _currentFilter = MutableStateFlow<QuestStatus?>(null)
    val currentFilter: StateFlow<QuestStatus?> = _currentFilter.asStateFlow()

    // === Quest Detail State ===

    private val _questDetail = MutableStateFlow<UiState<QuestResponse>>(UiState.Loading)
    val questDetail: StateFlow<UiState<QuestResponse>> = _questDetail.asStateFlow()

    // === Actions ===

    /**
     * Load all quests for the current user.
     *
     * @param status Optional filter by quest status
     */
    fun loadQuests(status: QuestStatus? = null) {
        scope.launch {
            _currentFilter.value = status
            _quests.value = UiState.Loading
            guidanceApi.getQuests(status?.name).fold(
                ifLeft = { error ->
                    _quests.value = UiState.Error(error.toString())
                },
                ifRight = { questList ->
                    _quests.value = UiState.Success(questList)
                }
            )
        }
    }

    /**
     * Load detailed quest information.
     *
     * @param questId Quest identifier
     */
    fun loadQuestDetail(questId: String) {
        scope.launch {
            _questDetail.value = UiState.Loading
            guidanceApi.getQuest(questId).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error(error.toString())
                },
                ifRight = { quest ->
                    _questDetail.value = UiState.Success(quest)
                }
            )
        }
    }

    /**
     * Create a new quest.
     *
     * @param request Quest creation parameters
     */
    fun createQuest(request: CreateQuestRequest) {
        scope.launch {
            guidanceApi.createQuest(request).fold(
                ifLeft = { error ->
                    _quests.value = UiState.Error("Failed to create quest: $error")
                },
                ifRight = { newQuest ->
                    // Reload quest list to include new quest
                    loadQuests(_currentFilter.value)
                }
            )
        }
    }

    /**
     * Start a quest (transition to ACTIVE status).
     *
     * WIP=1 enforcement: Only one quest can be active at a time.
     *
     * @param questId Quest identifier
     */
    fun startQuest(questId: String) {
        scope.launch {
            guidanceApi.startQuest(questId).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to start quest: $error")
                },
                ifRight = { updatedQuest ->
                    // Update detail view if currently viewing this quest
                    _questDetail.value = UiState.Success(updatedQuest)
                    // Reload quest list to reflect status change
                    loadQuests(_currentFilter.value)
                }
            )
        }
    }

    /**
     * Complete a quest (transition to COMPLETED status).
     *
     * Adds energy cost to today's budget spent.
     *
     * @param questId Quest identifier
     */
    fun completeQuest(questId: String) {
        scope.launch {
            guidanceApi.completeQuest(questId).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to complete quest: $error")
                },
                ifRight = { updatedQuest ->
                    // Update detail view if currently viewing this quest
                    _questDetail.value = UiState.Success(updatedQuest)
                    // Reload quest list to reflect status change
                    loadQuests(_currentFilter.value)
                }
            )
        }
    }

    /**
     * Abandon a quest (transition to ABANDONED status).
     *
     * @param questId Quest identifier
     */
    fun abandonQuest(questId: String) {
        scope.launch {
            guidanceApi.abandonQuest(questId).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to abandon quest: $error")
                },
                ifRight = { updatedQuest ->
                    // Update detail view if currently viewing this quest
                    _questDetail.value = UiState.Success(updatedQuest)
                    // Reload quest list to reflect status change
                    loadQuests(_currentFilter.value)
                }
            )
        }
    }

    /**
     * Return a quest to backlog (transition to BACKLOG status).
     *
     * @param questId Quest identifier
     */
    fun backlogQuest(questId: String) {
        scope.launch {
            guidanceApi.backlogQuest(questId).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to backlog quest: $error")
                },
                ifRight = { updatedQuest ->
                    // Update detail view if currently viewing this quest
                    _questDetail.value = UiState.Success(updatedQuest)
                    // Reload quest list to reflect status change
                    loadQuests(_currentFilter.value)
                }
            )
        }
    }

    /**
     * Update quest fields.
     *
     * @param questId Quest identifier
     * @param request Updated quest fields
     */
    fun updateQuest(questId: String, request: UpdateQuestRequest) {
        scope.launch {
            guidanceApi.updateQuest(questId, request).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to update quest: $error")
                },
                ifRight = { updatedQuest ->
                    // Update detail view if currently viewing this quest
                    _questDetail.value = UiState.Success(updatedQuest)
                    // Reload quest list to reflect changes
                    loadQuests(_currentFilter.value)
                }
            )
        }
    }

    /**
     * Delete a quest.
     *
     * @param questId Quest identifier
     */
    fun deleteQuest(questId: String) {
        scope.launch {
            guidanceApi.deleteQuest(questId).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to delete quest: $error")
                },
                ifRight = {
                    // Reload quest list to remove deleted quest
                    loadQuests(_currentFilter.value)
                }
            )
        }
    }

    // === Checkpoint Operations ===

    /**
     * Toggle checkpoint completion status.
     *
     * @param checkpointId Checkpoint identifier
     * @param completed New completion status
     */
    fun toggleCheckpoint(checkpointId: String, completed: Boolean) {
        scope.launch {
            val request = UpdateCheckpointRequest(completed = completed)
            guidanceApi.updateCheckpoint(checkpointId, request).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to update checkpoint: $error")
                },
                ifRight = { updatedCheckpoint ->
                    // Reload quest detail to reflect checkpoint change
                    val currentDetail = (_questDetail.value as? UiState.Success)?.data
                    if (currentDetail != null) {
                        loadQuestDetail(currentDetail.id)
                    }
                }
            )
        }
    }

    /**
     * Add a new checkpoint to a quest.
     *
     * @param questId Quest identifier
     * @param title Checkpoint description
     */
    fun addCheckpoint(questId: String, title: String) {
        scope.launch {
            val request = CreateCheckpointRequest(
                questId = questId,
                title = title
            )
            guidanceApi.addCheckpoint(questId, request).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to add checkpoint: $error")
                },
                ifRight = { newCheckpoint ->
                    // Reload quest detail to include new checkpoint
                    loadQuestDetail(questId)
                }
            )
        }
    }

    /**
     * Delete a checkpoint.
     *
     * @param checkpointId Checkpoint identifier
     */
    fun deleteCheckpoint(checkpointId: String) {
        scope.launch {
            guidanceApi.deleteCheckpoint(checkpointId).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to delete checkpoint: $error")
                },
                ifRight = {
                    // Reload quest detail to remove deleted checkpoint
                    val currentDetail = (_questDetail.value as? UiState.Success)?.data
                    if (currentDetail != null) {
                        loadQuestDetail(currentDetail.id)
                    }
                }
            )
        }
    }

    /**
     * Reorder checkpoints within a quest.
     *
     * @param questId Quest identifier
     * @param checkpointIds Ordered list of checkpoint IDs
     */
    fun reorderCheckpoints(questId: String, checkpointIds: List<String>) {
        scope.launch {
            val request = ReorderCheckpointsRequest(order = checkpointIds)
            guidanceApi.reorderCheckpoints(questId, request).fold(
                ifLeft = { error ->
                    _questDetail.value = UiState.Error("Failed to reorder checkpoints: $error")
                },
                ifRight = {
                    // Reload quest detail to reflect new order
                    loadQuestDetail(questId)
                }
            )
        }
    }
}
