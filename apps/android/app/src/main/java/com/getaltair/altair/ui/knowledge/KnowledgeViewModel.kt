package com.getaltair.altair.ui.knowledge

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.data.local.dao.NoteDao
import com.getaltair.altair.data.local.entity.NoteEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import java.util.UUID

class KnowledgeViewModel(
    private val noteDao: NoteDao,
    private val db: PowerSyncDatabase,
    private val tokenPreferences: TokenPreferences,
) : ViewModel() {
    private val userId: String
        get() = tokenPreferences.accessToken?.let { decodeUserIdFromJwt(it) } ?: ""

    val searchQuery = MutableStateFlow("")

    val notes: StateFlow<List<NoteEntity>> =
        combine(
            noteDao.watchAll(userId),
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
            val now = nowIso()
            val id = UUID.randomUUID().toString()
            db.execute(
                "INSERT INTO notes (id, title, content, user_id, initiative_id, created_at, updated_at, deleted_at) " +
                    "VALUES (?, ?, ?, ?, NULL, ?, ?, NULL)",
                listOf(id, title, content, userId, now, now),
            )
        }
    }

    fun updateNote(
        id: String,
        title: String,
        content: String,
    ) {
        viewModelScope.launch {
            val now = nowIso()
            db.execute(
                "UPDATE notes SET title = ?, content = ?, updated_at = ? WHERE id = ?",
                listOf(title, content, now, id),
            )
        }
    }
}
