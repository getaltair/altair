package com.getaltair.altair.ui.today

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.dao.RoutineDao
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.data.local.entity.RoutineEntity
import com.getaltair.altair.data.local.entity.UserEntity
import com.powersync.PowerSyncDatabase
import com.powersync.sync.SyncStatusData
import kotlinx.coroutines.CancellationException
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
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import java.util.UUID

private const val TAG = "TodayViewModel"
private val TERMINAL_STATUSES = setOf("completed", "cancelled", "deferred")

class TodayViewModel(
    private val questDao: QuestDao,
    private val routineDao: RoutineDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error

    // Current user (first synced user in the local DB)
    @OptIn(ExperimentalCoroutinesApi::class)
    val currentUser: StateFlow<UserEntity?> =
        db
            .watch<UserEntity>(
                sql = "SELECT * FROM users WHERE deleted_at IS NULL LIMIT 1",
                parameters = emptyList(),
            ) { cursor ->
                UserEntity(
                    id = cursor.getString(0)!!,
                    email = cursor.getString(1)!!,
                    displayName = cursor.getString(2),
                    isAdmin = cursor.getLong(3)?.toInt() ?: 0,
                    status = cursor.getString(4)!!,
                    createdAt = cursor.getString(5)!!,
                    updatedAt = cursor.getString(6)!!,
                    deletedAt = cursor.getString(7),
                )
            }.map { it.firstOrNull() }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    val syncStatus: StateFlow<SyncStatusData> =
        db.currentStatus
            .asFlow()
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), db.currentStatus)

    private val userId: String? get() = currentUser.value?.id

    // Today's quests: due_date = today OR null, status NOT in terminal
    @OptIn(ExperimentalCoroutinesApi::class)
    val todayQuests: StateFlow<List<QuestEntity>> =
        currentUser
            .flatMapLatest { user ->
                if (user == null) {
                    flowOf(emptyList())
                } else {
                    questDao.watchAll(user.id).map { quests ->
                        val today = today()
                        quests.filter { quest ->
                            quest.status !in TERMINAL_STATUSES &&
                                (quest.dueDate == null || quest.dueDate.startsWith(today))
                        }
                    }
                }
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    @OptIn(ExperimentalCoroutinesApi::class)
    val dueRoutines: StateFlow<List<RoutineEntity>> =
        currentUser
            .flatMapLatest { user ->
                if (user == null) {
                    flowOf(emptyList())
                } else {
                    routineDao.watchAll(user.id).map { routines ->
                        routines.filter { it.status != "inactive" }
                    }
                }
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    // Whether today's check-in is done
    val isTodayCheckinDone: StateFlow<Boolean> =
        db
            .watch<String>(
                sql = "SELECT id FROM daily_checkins WHERE checkin_date = ? AND deleted_at IS NULL LIMIT 1",
                parameters = listOf(today()),
            ) { cursor ->
                cursor.getString(0) ?: ""
            }.map { it.isNotEmpty() }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), false)

    // Checkin form state — write via setCheckinEnergy/setCheckinMood
    private val _checkinEnergy = MutableStateFlow(3)
    val checkinEnergy: StateFlow<Int> = _checkinEnergy

    private val _checkinMood = MutableStateFlow(3)
    val checkinMood: StateFlow<Int> = _checkinMood

    fun setCheckinEnergy(value: Int) {
        _checkinEnergy.value = value
    }

    fun setCheckinMood(value: Int) {
        _checkinMood.value = value
    }

    fun startQuest(id: String) {
        viewModelScope.launch {
            try {
                val now = Clock.System.now().toString()
                db.execute(
                    "UPDATE quests SET status = 'in_progress', updated_at = ? WHERE id = ? AND status = 'not_started'",
                    listOf(now, id),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to start quest", e)
                _error.value = e.message ?: "Failed to start quest"
            }
        }
    }

    fun completeQuest(id: String) {
        viewModelScope.launch {
            try {
                val now = Clock.System.now().toString()
                db.execute(
                    "UPDATE quests SET status = 'completed', updated_at = ? WHERE id = ? AND status = 'in_progress'",
                    listOf(now, id),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to complete quest", e)
                _error.value = e.message ?: "Failed to complete quest"
            }
        }
    }

    fun submitCheckin(
        energy: Int,
        mood: Int,
    ) {
        val uid =
            userId ?: run {
                _error.value = "User not available — try again after sync"
                return
            }
        require(energy in 1..5) { "energyLevel must be 1–5, got $energy" }
        viewModelScope.launch {
            try {
                val now = Clock.System.now().toString()
                val todayStr = today()
                val id = UUID.randomUUID().toString()
                db.execute(
                    """INSERT OR IGNORE INTO daily_checkins
                   (id, user_id, checkin_date, energy_level, mood, notes, created_at, updated_at, deleted_at)
                   VALUES (?, ?, ?, ?, ?, NULL, ?, ?, NULL)""",
                    // mood is stored as VARCHAR(30) on server; toString() aligns with schema
                    listOf(id, uid, todayStr, energy, mood.toString(), now, now),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to submit check-in", e)
                _error.value = e.message ?: "Failed to submit check-in"
            }
        }
    }

    fun clearError() {
        _error.value = null
    }

    private fun today(): String =
        Clock.System
            .now()
            .toLocalDateTime(TimeZone.currentSystemDefault())
            .date
            .toString()
}
