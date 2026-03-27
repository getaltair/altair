package com.getaltair.altair.ui.knowledge

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.entity.ContentType
import com.getaltair.altair.domain.entity.KnowledgeNote
import com.getaltair.altair.domain.repository.KnowledgeNoteRepository
import com.getaltair.altair.navigation.Screen
import java.time.Instant
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.firstOrNull
import kotlinx.coroutines.launch

class NoteEditorViewModel(
    savedStateHandle: SavedStateHandle,
    private val knowledgeNoteRepository: KnowledgeNoteRepository,
) : ViewModel() {

    private val noteId: String? = savedStateHandle.get<String>(Screen.NoteEditorEdit.ARG_NOTE_ID)
    private val isEditMode: Boolean = noteId != null

    private val _uiState = MutableStateFlow(NoteEditorUiState())
    val uiState: StateFlow<NoteEditorUiState> = _uiState.asStateFlow()

    init {
        if (isEditMode) {
            loadExistingNote()
        }
    }

    private fun loadExistingNote() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isLoading = true)
            val note = knowledgeNoteRepository.getById(UUID.fromString(noteId))
                .catch { /* leave empty */ }
                .firstOrNull()
            if (note != null) {
                _uiState.value = NoteEditorUiState(
                    title = note.title,
                    content = note.content ?: "",
                    contentType = note.contentType,
                    isPinned = note.isPinned,
                    isLoading = false,
                )
            } else {
                _uiState.value = _uiState.value.copy(isLoading = false)
            }
        }
    }

    fun onTitleChange(title: String) {
        _uiState.value = _uiState.value.copy(
            title = title,
            hasUnsavedChanges = true,
            titleError = false,
        )
    }

    fun onContentChange(content: String) {
        _uiState.value = _uiState.value.copy(
            content = content,
            hasUnsavedChanges = true,
        )
    }

    fun onContentTypeChange(type: ContentType) {
        _uiState.value = _uiState.value.copy(
            contentType = type,
            hasUnsavedChanges = true,
        )
    }

    fun onPinToggle() {
        _uiState.value = _uiState.value.copy(
            isPinned = !_uiState.value.isPinned,
            hasUnsavedChanges = true,
        )
    }

    fun save() {
        val state = _uiState.value
        if (state.title.isBlank()) {
            _uiState.value = state.copy(titleError = true)
            return
        }

        viewModelScope.launch {
            _uiState.value = state.copy(isLoading = true)
            try {
                val now = Instant.now()
                if (isEditMode) {
                    val existing = knowledgeNoteRepository.getById(UUID.fromString(noteId))
                        .catch { /* ignore */ }
                        .firstOrNull()
                    if (existing != null) {
                        knowledgeNoteRepository.update(
                            existing.copy(
                                title = state.title,
                                content = state.content.ifBlank { null },
                                contentType = state.contentType,
                                isPinned = state.isPinned,
                                updatedAt = now,
                            ),
                        )
                    }
                } else {
                    knowledgeNoteRepository.create(
                        KnowledgeNote(
                            id = UUID.randomUUID(),
                            userId = UUID.randomUUID(), // Will be set by repository/server
                            householdId = null,
                            initiativeId = null,
                            title = state.title,
                            content = state.content.ifBlank { null },
                            contentType = state.contentType,
                            isPinned = state.isPinned,
                            createdAt = now,
                            updatedAt = now,
                        ),
                    )
                }
                _uiState.value = state.copy(
                    isLoading = false,
                    isSaved = true,
                    hasUnsavedChanges = false,
                )
            } catch (_: Exception) {
                _uiState.value = state.copy(isLoading = false)
            }
        }
    }
}
