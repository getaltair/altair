package com.getaltair.altair.ui.knowledge

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.entity.ContentType
import com.getaltair.altair.domain.repository.KnowledgeNoteRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class NoteListViewModel(
    private val knowledgeNoteRepository: KnowledgeNoteRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow(NoteListUiState(isLoading = true))
    val uiState: StateFlow<NoteListUiState> = _uiState.asStateFlow()

    init {
        loadNotes()
    }

    private fun loadNotes() {
        viewModelScope.launch {
            knowledgeNoteRepository.getAll()
                .catch { _uiState.value = _uiState.value.copy(isLoading = false) }
                .collect { notes ->
                    _uiState.value = _uiState.value.copy(
                        notes = applyFilter(notes, _uiState.value.activeFilter),
                        isLoading = false,
                    )
                }
        }
    }

    fun onSearchQueryChange(query: String) {
        _uiState.value = _uiState.value.copy(searchQuery = query)
        viewModelScope.launch {
            val flow = if (query.isBlank()) {
                knowledgeNoteRepository.getAll()
            } else {
                knowledgeNoteRepository.search(query)
            }
            flow.catch { /* keep current state */ }
                .collect { notes ->
                    _uiState.value = _uiState.value.copy(
                        notes = applyFilter(notes, _uiState.value.activeFilter),
                        isLoading = false,
                    )
                }
        }
    }

    fun onFilterChange(filter: NoteFilter) {
        _uiState.value = _uiState.value.copy(activeFilter = filter)
        viewModelScope.launch {
            val flow = if (_uiState.value.searchQuery.isBlank()) {
                knowledgeNoteRepository.getAll()
            } else {
                knowledgeNoteRepository.search(_uiState.value.searchQuery)
            }
            flow.catch { /* keep current state */ }
                .collect { notes ->
                    _uiState.value = _uiState.value.copy(
                        notes = applyFilter(notes, filter),
                    )
                }
        }
    }

    private fun applyFilter(
        notes: List<com.getaltair.altair.domain.entity.KnowledgeNote>,
        filter: NoteFilter,
    ) = when (filter) {
        NoteFilter.ALL -> notes
        NoteFilter.PINNED -> notes.filter { it.isPinned }
        NoteFilter.MARKDOWN -> notes.filter { it.contentType == ContentType.MARKDOWN }
        NoteFilter.PLAIN -> notes.filter { it.contentType == ContentType.PLAIN }
    }
}
