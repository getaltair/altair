package com.getaltair.altair.ui.guidance

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.InitiativeDao
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.dao.RoutineDao
import com.getaltair.altair.data.local.entity.InitiativeEntity
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.data.local.entity.RoutineEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import kotlinx.datetime.Clock

class GuidanceViewModel(
    private val initiativeDao: InitiativeDao,
    private val questDao: QuestDao,
    private val routineDao: RoutineDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
    private val userId: StateFlow<String?> =
        db
            .watch<String>(
                sql = "SELECT id FROM users WHERE deleted_at IS NULL LIMIT 1",
                parameters = emptyList(),
            ) { cursor ->
                cursor.getString(0) ?: ""
            }.map { it.firstOrNull()?.ifEmpty { null } }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    @OptIn(ExperimentalCoroutinesApi::class)
    val initiatives: StateFlow<List<InitiativeEntity>> =
        userId
            .flatMapLatest { uid ->
                if (uid == null) {
                    flowOf(emptyList())
                } else {
                    initiativeDao.watchAll(uid)
                }
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    @OptIn(ExperimentalCoroutinesApi::class)
    val quests: StateFlow<List<QuestEntity>> =
        userId
            .flatMapLatest { uid ->
                if (uid == null) {
                    flowOf(emptyList())
                } else {
                    questDao.watchAll(uid)
                }
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    @OptIn(ExperimentalCoroutinesApi::class)
    val routines: StateFlow<List<RoutineEntity>> =
        userId
            .flatMapLatest { uid ->
                if (uid == null) {
                    flowOf(emptyList())
                } else {
                    routineDao.watchAll(uid)
                }
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    // Status filter for quest list tab
    val statusFilter = MutableStateFlow<String?>(null)

    val filteredQuests: StateFlow<List<QuestEntity>> =
        quests
            .map { list ->
                val filter = statusFilter.value
                if (filter == null) list else list.filter { it.status == filter }
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    fun markRoutineDone(routineId: String) {
        viewModelScope.launch {
            val now = Clock.System.now().toString()
            db.execute(
                "UPDATE routines SET updated_at = ? WHERE id = ?",
                listOf(now, routineId),
            )
        }
    }
}
