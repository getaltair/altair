package com.getaltair.altair.ui.knowledge

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.NoteDao
import com.getaltair.altair.data.local.entity.NoteEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import kotlinx.datetime.Clock
import java.util.UUID

private const val TAG = "KnowledgeViewModel"

@OptIn(ExperimentalCoroutinesApi::class)
class KnowledgeViewModel(
    private val noteDao: NoteDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
    val searchQuery = MutableStateFlow("")

    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error

    // Reactive user ID from DB — avoids stale JWT at construction and null fallback
    private val currentUserId: StateFlow<String?> =
        db
            .watch<String>(
                sql = "SELECT id FROM users WHERE deleted_at IS NULL LIMIT 1",
                parameters = emptyList(),
            ) { cursor -> cursor.getString(0) ?: "" }
            .map { list -> list.firstOrNull()?.takeIf { it.isNotEmpty() } }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    val notes: StateFlow<List<NoteEntity>> =
        combine(
            currentUserId.flatMapLatest { uid ->
                if (uid == null) flowOf(emptyList()) else noteDao.watchAll(uid)
            },
            searchQuery,
        ) { allNotes, query ->
            if (query.isBlank()) {
                allNotes.sortedByDescending { it.updatedAt }
            } else {
                allNotes
                    .filter {
                        it.title.contains(query, ignoreCase = true) ||
                            it.content?.contains(query, ignoreCase = true) == true
                    }.sortedByDescending { it.updatedAt }
            }
        }.stateIn(
            scope = viewModelScope,
            started = SharingStarted.WhileSubscribed(5_000),
            initialValue = emptyList(),
        )

    fun searchNotes(query: String) {
        searchQuery.value = query
    }

    fun createNote(
        title: String,
        content: String,
    ) {
        viewModelScope.launch {
            val uid =
                currentUserId.value ?: run {
                    _error.value = "User not available — cannot create note"
                    return@launch
                }
            try {
                val now = Clock.System.now().toString()
                val id = UUID.randomUUID().toString()
                db.execute(
                    "INSERT INTO notes (id, title, content, user_id, initiative_id, created_at, updated_at, deleted_at) " +
                        "VALUES (?, ?, ?, ?, NULL, ?, ?, NULL)",
                    listOf(id, title, content, uid, now, now),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create note", e)
                _error.value = e.message ?: "Failed to create note"
            }
        }
    }

    fun updateNote(
        id: String,
        title: String,
        content: String,
    ) {
        viewModelScope.launch {
            try {
                val now = Clock.System.now().toString()
                db.execute(
                    "UPDATE notes SET title = ?, content = ?, updated_at = ? WHERE id = ?",
                    listOf(title, content, now, id),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to update note", e)
                _error.value = e.message ?: "Failed to update note"
            }
        }
    }
}
