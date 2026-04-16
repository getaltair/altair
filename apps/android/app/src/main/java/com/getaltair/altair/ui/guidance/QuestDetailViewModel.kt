package com.getaltair.altair.ui.guidance

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.entity.QuestEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter

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
    private val questDao: QuestDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
    private var questId: String = ""

    lateinit var quest: StateFlow<QuestEntity?>

    fun init(id: String) {
        if (questId == id) return
        questId = id
        quest =
            questDao
                .watchById(id)
                .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)
    }

    fun validTransitions(currentStatus: String): List<String> = VALID_TRANSITIONS[currentStatus] ?: emptyList()

    fun transitionStatus(newStatus: String) {
        viewModelScope.launch {
            val now = nowIso()
            db.execute(
                "UPDATE quests SET status = ?, updated_at = ? WHERE id = ?",
                listOf(newStatus, now, questId),
            )
        }
    }

    private fun nowIso(): String = LocalDateTime.now().format(DateTimeFormatter.ISO_LOCAL_DATE_TIME)
}
