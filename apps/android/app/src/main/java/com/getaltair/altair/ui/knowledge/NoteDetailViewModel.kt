package com.getaltair.altair.ui.knowledge

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.EntityRelationDao
import com.getaltair.altair.domain.repository.KnowledgeNoteRepository
import com.getaltair.altair.navigation.Screen
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class NoteDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val knowledgeNoteRepository: KnowledgeNoteRepository,
    private val entityRelationDao: EntityRelationDao,
) : ViewModel() {

    private val noteId: UUID = UUID.fromString(
        requireNotNull(savedStateHandle.get<String>(Screen.NoteDetail.ARG_NOTE_ID)) {
            "Missing note ID navigation argument"
        },
    )

    private val _uiState = MutableStateFlow(NoteDetailUiState())
    val uiState: StateFlow<NoteDetailUiState> = _uiState.asStateFlow()

    init {
        loadNote()
        loadRelations()
    }

    private fun loadNote() {
        viewModelScope.launch {
            knowledgeNoteRepository.getById(noteId)
                .catch {
                    _uiState.value = _uiState.value.copy(isLoading = false)
                }
                .collect { note ->
                    _uiState.value = _uiState.value.copy(
                        note = note,
                        isLoading = false,
                    )
                }
        }
    }

    private fun loadRelations() {
        viewModelScope.launch {
            entityRelationDao.getByEntity("knowledge_note", noteId)
                .catch { /* keep empty relations */ }
                .collect { relations ->
                    _uiState.value = _uiState.value.copy(relations = relations)
                }
        }
    }

    fun togglePin() {
        viewModelScope.launch {
            try {
                knowledgeNoteRepository.togglePin(noteId)
            } catch (_: Exception) {
                // Pin state will update via the flow collection
            }
        }
    }

    fun onDeleteRequest() {
        _uiState.value = _uiState.value.copy(showDeleteConfirm = true)
    }

    fun onDeleteDismiss() {
        _uiState.value = _uiState.value.copy(showDeleteConfirm = false)
    }

    fun onDeleteConfirm() {
        viewModelScope.launch {
            try {
                knowledgeNoteRepository.delete(noteId)
                _uiState.value = _uiState.value.copy(
                    showDeleteConfirm = false,
                    isDeleted = true,
                )
            } catch (_: Exception) {
                _uiState.value = _uiState.value.copy(showDeleteConfirm = false)
            }
        }
    }
}
