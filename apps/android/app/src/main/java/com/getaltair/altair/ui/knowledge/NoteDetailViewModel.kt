package com.getaltair.altair.ui.knowledge

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.data.local.dao.EntityRelationDao
import com.getaltair.altair.data.local.dao.NoteDao
import com.getaltair.altair.data.local.dao.NoteSnapshotDao
import com.getaltair.altair.data.local.entity.EntityRelationEntity
import com.getaltair.altair.data.local.entity.NoteEntity
import com.getaltair.altair.data.local.entity.NoteSnapshotEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import java.util.UUID

class NoteDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val noteDao: NoteDao,
    private val entityRelationDao: EntityRelationDao,
    private val noteSnapshotDao: NoteSnapshotDao,
    private val db: PowerSyncDatabase,
    private val tokenPreferences: TokenPreferences,
) : ViewModel() {
    private val noteId: String = checkNotNull(savedStateHandle["id"])

    private val userId: String
        get() = tokenPreferences.accessToken?.let { decodeUserIdFromJwt(it) } ?: ""

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
            val now = nowIso()
            db.execute(
                "UPDATE notes SET title = ?, content = ?, updated_at = ? WHERE id = ?",
                listOf(title, content, now, noteId),
            )
        }
    }

    fun linkNote(
        sourceNoteId: String,
        targetNoteId: String,
    ) {
        viewModelScope.launch {
            val now = nowIso()
            val id = UUID.randomUUID().toString()
            db.execute(
                "INSERT INTO entity_relations " +
                    "(id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, " +
                    "relation_type, source_type, status, confidence, evidence, user_id, " +
                    "created_at, updated_at, deleted_at) " +
                    "VALUES (?, 'note', ?, 'note', ?, 'note_link', 'manual', 'active', NULL, NULL, ?, ?, ?, NULL)",
                listOf(id, sourceNoteId, targetNoteId, userId, now, now),
            )
        }
    }
}
