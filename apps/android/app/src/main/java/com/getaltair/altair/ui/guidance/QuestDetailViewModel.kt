package com.getaltair.altair.ui.guidance

import android.util.Log
import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.entity.QuestEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

private const val TAG = "QuestDetailViewModel"

// Valid transitions per 06-state-machines.md
private val VALID_TRANSITIONS: Map<String, List<String>> =
    mapOf(
        "not_started" to listOf("in_progress", "deferred", "cancelled"),
        "in_progress" to listOf("completed", "deferred", "cancelled"),
        "deferred" to listOf("not_started", "in_progress", "cancelled"),
        "completed" to emptyList(),
        "cancelled" to emptyList(),
    )

class QuestDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val questDao: QuestDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
    private val questId: String = checkNotNull(savedStateHandle["id"])

    val quest: StateFlow<QuestEntity?> =
        questDao
            .watchById(questId)
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error

    fun validTransitions(currentStatus: String): List<String> = VALID_TRANSITIONS[currentStatus] ?: emptyList()

    fun transitionStatus(newStatus: String) {
        viewModelScope.launch {
            try {
                val now =
                    kotlinx.datetime.Clock.System
                        .now()
                        .toString()
                db.execute(
                    "UPDATE quests SET status = ?, updated_at = ? WHERE id = ?",
                    listOf(newStatus, now, questId),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to transition quest status", e)
                _error.value = e.message ?: "Failed to update quest"
            }
        }
    }
}
