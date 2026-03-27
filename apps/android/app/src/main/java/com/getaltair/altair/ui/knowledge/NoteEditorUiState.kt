package com.getaltair.altair.ui.knowledge

import com.getaltair.altair.domain.entity.ContentType

data class NoteEditorUiState(
    val title: String = "",
    val content: String = "",
    val contentType: ContentType = ContentType.MARKDOWN,
    val isPinned: Boolean = false,
    val isLoading: Boolean = false,
    val isSaved: Boolean = false,
    val hasUnsavedChanges: Boolean = false,
    val titleError: Boolean = false,
    val saveError: String? = null,
)
