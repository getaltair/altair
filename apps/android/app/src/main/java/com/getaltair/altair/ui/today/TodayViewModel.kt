package com.getaltair.altair.ui.today

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.DailyCheckinDao
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.dao.RoutineDao
import com.getaltair.altair.data.local.dao.UserDao
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.data.local.entity.RoutineEntity
import com.getaltair.altair.data.local.entity.UserEntity
import com.powersync.PowerSyncDatabase
import com.powersync.sync.SyncStatusData
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import java.time.LocalDate
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter
import java.util.UUID

private val TERMINAL_STATUSES = setOf("completed", "cancelled", "deferred")

class TodayViewModel(
    private val userDao: UserDao,
    private val questDao: QuestDao,
    private val routineDao: RoutineDao,
    private val dailyCheckinDao: DailyCheckinDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
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
                    passwordHash = cursor.getString(1),
                    email = cursor.getString(2)!!,
                    displayName = cursor.getString(3),
                    isAdmin = cursor.getLong(4)?.toInt() ?: 0,
                    status = cursor.getString(5)!!,
                    createdAt = cursor.getString(6)!!,
                    updatedAt = cursor.getString(7)!!,
                    deletedAt = cursor.getString(8),
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
                        val today = LocalDate.now().toString()
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
                parameters = listOf(LocalDate.now().toString()),
            ) { cursor ->
                cursor.getString(0) ?: ""
            }.map { it.isNotEmpty() }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), false)

    // Checkin form state
    val checkinEnergy = MutableStateFlow(3)
    val checkinMood = MutableStateFlow(3)

    fun startQuest(id: String) {
        viewModelScope.launch {
            val now = nowIso()
            db.execute(
                "UPDATE quests SET status = 'in_progress', updated_at = ? WHERE id = ? AND status = 'not_started'",
                listOf(now, id),
            )
        }
    }

    fun completeQuest(id: String) {
        viewModelScope.launch {
            val now = nowIso()
            db.execute(
                "UPDATE quests SET status = 'completed', updated_at = ? WHERE id = ? AND status = 'in_progress'",
                listOf(now, id),
            )
        }
    }

    fun submitCheckin(
        energy: Int,
        mood: Int,
    ) {
        val uid = userId ?: return
        viewModelScope.launch {
            val now = nowIso()
            val today = LocalDate.now().toString()
            val id = UUID.randomUUID().toString()
            db.execute(
                """INSERT OR IGNORE INTO daily_checkins
                   (id, user_id, checkin_date, energy_level, mood, notes, created_at, updated_at, deleted_at)
                   VALUES (?, ?, ?, ?, ?, NULL, ?, ?, NULL)""",
                listOf(id, uid, today, energy, mood.toString(), now, now),
            )
        }
    }

    private fun nowIso(): String = LocalDateTime.now().format(DateTimeFormatter.ISO_LOCAL_DATE_TIME)
}
