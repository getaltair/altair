package com.getaltair.altair.ui.knowledge

import com.getaltair.altair.domain.entity.KnowledgeNote

enum class NoteFilter { ALL, PINNED, MARKDOWN, PLAIN }

data class NoteListUiState(
    val notes: List<KnowledgeNote> = emptyList(),
    val searchQuery: String = "",
    val activeFilter: NoteFilter = NoteFilter.ALL,
    val isLoading: Boolean = false,
)
