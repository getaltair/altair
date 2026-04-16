package com.getaltair.altair.ui.knowledge

import android.util.Log
import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.EntityRelationDao
import com.getaltair.altair.data.local.dao.NoteDao
import com.getaltair.altair.data.local.dao.NoteSnapshotDao
import com.getaltair.altair.data.local.entity.EntityRelationEntity
import com.getaltair.altair.data.local.entity.NoteEntity
import com.getaltair.altair.data.local.entity.NoteSnapshotEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import kotlinx.datetime.Clock
import java.util.UUID

private const val TAG = "NoteDetailViewModel"

class NoteDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val noteDao: NoteDao,
    private val entityRelationDao: EntityRelationDao,
    private val noteSnapshotDao: NoteSnapshotDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
    private val noteId: String = checkNotNull(savedStateHandle["id"])

    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error

    // Reactive user ID — avoids null fallback from JWT decode failure
    private val currentUserId: StateFlow<String?> =
        db
            .watch<String?>(
                sql = "SELECT id FROM users WHERE deleted_at IS NULL LIMIT 1",
                parameters = emptyList(),
            ) { cursor -> cursor.getString(0) }
            .map { it.firstOrNull() }
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), null)

    val note: StateFlow<NoteEntity?> =
        noteDao
            .watchById(noteId)
            .stateIn(
                scope = viewModelScope,
                started = SharingStarted.WhileSubscribed(5_000),
                initialValue = null,
            )

    val backlinks: StateFlow<List<EntityRelationEntity>> =
        entityRelationDao
            .watchBacklinksForNote(noteId)
            .stateIn(
                scope = viewModelScope,
                started = SharingStarted.WhileSubscribed(5_000),
                initialValue = emptyList(),
            )

    val snapshots: StateFlow<List<NoteSnapshotEntity>> =
        noteSnapshotDao
            .watchAll(noteId)
            .stateIn(
                scope = viewModelScope,
                started = SharingStarted.WhileSubscribed(5_000),
                initialValue = emptyList(),
            )

    fun saveNote(
        title: String,
        content: String,
    ) {
        viewModelScope.launch {
            try {
                val now = Clock.System.now().toString()
                db.execute(
                    "UPDATE notes SET title = ?, content = ?, updated_at = ? WHERE id = ?",
                    listOf(title, content, now, noteId),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to save note", e)
                _error.value = e.message ?: "Failed to save note"
            }
        }
    }

    fun linkNote(
        sourceNoteId: String,
        targetNoteId: String,
    ) {
        viewModelScope.launch {
            val uid =
                currentUserId.value ?: run {
                    _error.value = "User not available — cannot link note"
                    return@launch
                }
            try {
                val now = Clock.System.now().toString()
                val id = UUID.randomUUID().toString()
                db.execute(
                    "INSERT INTO entity_relations " +
                        "(id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, " +
                        "relation_type, source_type, status, confidence, evidence, user_id, " +
                        "created_at, updated_at, deleted_at) " +
                        "VALUES (?, 'note', ?, 'note', ?, 'note_link', 'manual', 'active', NULL, NULL, ?, ?, ?, NULL)",
                    listOf(id, sourceNoteId, targetNoteId, uid, now, now),
                )
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to link note", e)
                _error.value = e.message ?: "Failed to link note"
            }
        }
    }
}
